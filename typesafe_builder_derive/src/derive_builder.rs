mod eval_condition;
mod generate_build_methods;
mod generate_setter_methods;
mod validate_condition;

use crate::{Input, input::Requirement};
use eval_condition::eval_condition;
use generate_build_methods::generate_build_methods;
use generate_setter_methods::generate_setter_methods;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, PathArguments, Type};
use validate_condition::validate_condition_fields;

type FieldInfo = (Ident, Type, Requirement, Option<syn::Expr>);

pub fn derive_builder_impl(input: Input) -> Result<TokenStream2, darling::Error> {
    let name = input.ident();
    let builder_name = Ident::new(&input.builder_name(), name.span());
    let generics = input.generics();

    let field_infos = extract_field_infos(&input)?;
    let n_fields = field_infos.len();
    let type_params = generate_type_params(n_fields, name.span());
    let default_generics = (0..n_fields)
        .map(|_| quote! { _TypesafeBuilderEmpty })
        .collect::<Vec<_>>();

    let (_, _, where_clause) = generics.split_for_impl();
    let generic_params = &generics.params;

    let builder_fields = generate_builder_fields(&field_infos, &type_params);
    let builder_initialization = generate_builder_initialization(&field_infos);
    let setter_methods = generate_setter_methods(
        &field_infos,
        &type_params,
        &builder_name,
        generics,
        name.span(),
    );
    let build_impls = generate_build_methods(&field_infos, &builder_name, name, generics);

    let builder_struct = if generic_params.is_empty() {
        quote! {
            pub struct #builder_name < #( #type_params ),* > {
                #( #builder_fields )*
            }
        }
    } else {
        quote! {
            pub struct #builder_name < #generic_params, #( #type_params ),* > {
                #( #builder_fields )*
            }
        }
    };

    let new_impl = if generic_params.is_empty() {
        quote! {
            impl #builder_name < #( #default_generics ),* > {
                #[inline]
                pub fn new() -> Self {
                    Self { #( #builder_initialization )* }
                 }
            }
        }
    } else {
        quote! {
            impl < #generic_params > #builder_name < #generic_params, #( #default_generics ),* > #where_clause {
                #[inline]
                pub fn new() -> Self {
                    Self { #( #builder_initialization )* }
                 }
            }
        }
    };

    let setter_impl = if generic_params.is_empty() {
        quote! {
            impl < #( #type_params ),* > #builder_name < #( #type_params ),* > {
                #( #setter_methods )*
            }
        }
    } else {
        quote! {
            impl < #generic_params, #( #type_params ),* > #builder_name < #generic_params, #( #type_params ),* > #where_clause {
                #( #setter_methods )*
            }
        }
    };

    Ok(quote! {
        #builder_struct

        #new_impl

        #setter_impl

        #( #build_impls )*
    })
}

fn extract_field_infos(builder_input: &Input) -> Result<Vec<FieldInfo>, darling::Error> {
    let mut field_infos = Vec::new();
    let mut all_field_names = Vec::new();

    for field in builder_input
        .data()
        .take_struct()
        .expect("only named structs are supported")
        .fields
        .iter()
    {
        let ident = field
            .ident()
            .cloned()
            .expect("darling guarantees named fields");
        all_field_names.push(ident.to_string());
    }

    for field in builder_input
        .data()
        .take_struct()
        .expect("only named structs are supported")
        .fields
        .iter()
    {
        let ident = field
            .ident()
            .cloned()
            .expect("darling guarantees named fields");

        let req = field
            .requirement()
            .map_err(|err| darling::Error::custom(format!("Invalid requirement: {}", err)))?;

        if let Requirement::Conditional(expr) = &req {
            if let Err(err) = validate_condition_fields(expr, &all_field_names) {
                return Err(darling::Error::custom(err));
            }
        }

        if let Requirement::OptionalIf(expr) = &req {
            if let Err(err) = validate_condition_fields(expr, &all_field_names) {
                return Err(darling::Error::custom(err));
            }
        }

        let requirement_is_option_based = match &req {
            Requirement::Optional | Requirement::Conditional(_) | Requirement::OptionalIf(_) => {
                true
            }
            Requirement::Always | Requirement::Default => false,
        };

        if requirement_is_option_based && !is_type_option(field.ty()) {
            let requirement_name = match &req {
                Requirement::Optional => "optional",
                Requirement::Conditional(_) => "required_if",
                Requirement::OptionalIf(_) => "optional_if",
                _ => unreachable!(),
            };
            return Err(darling::Error::custom(format!(
                "Field `{}` marked with `#[builder({})]` must be of type `Option<T>`",
                ident, requirement_name
            ))
            .with_span(&field.ty()));
        }

        field_infos.push((ident, field.ty().clone(), req, field.default().cloned()));
    }

    Ok(field_infos)
}

fn generate_type_params(n_fields: usize, span: proc_macro2::Span) -> Vec<Ident> {
    (0..n_fields)
        .map(|i| Ident::new(&format!("_TypesafeBuilder{}", i), span))
        .collect()
}

fn generate_builder_fields<'a>(
    field_infos: &'a [FieldInfo],
    type_params: &'a [Ident],
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    field_infos
        .iter()
        .zip(type_params.iter())
        .map(|((ident, ty, req, _), tp)| {
            let phantom = Ident::new(&format!("_{}", ident), ident.span());
            match req {
                &Requirement::Always | &Requirement::Default => {
                    quote! {
                        #ident : Option<#ty>,
                        #phantom : std::marker::PhantomData<#tp>,
                    }
                }
                &Requirement::Optional
                | &Requirement::Conditional(_)
                | &Requirement::OptionalIf(_) => {
                    quote! {
                        #ident : #ty,
                        #phantom : std::marker::PhantomData<#tp>,
                    }
                }
            }
        })
}

fn generate_builder_initialization<'a>(
    field_infos: &'a [FieldInfo],
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    field_infos.iter().map(|(ident, _ty, req, default)| {
        let phantom = Ident::new(&format!("_{}", ident), ident.span());
        match req {
            Requirement::Default => {
                if let Some(default_expr) = default {
                    quote! {
                        #ident : Some(#default_expr),
                        #phantom : std::marker::PhantomData,
                    }
                } else {
                    quote! {
                        #ident : None,
                        #phantom : std::marker::PhantomData,
                    }
                }
            }
            _ => {
                quote! {
                    #ident : None,
                    #phantom : std::marker::PhantomData,
                }
            }
        }
    })
}

fn is_type_option(field_ty: &Type) -> bool {
    if let Type::Path(type_path) = field_ty {
        if let Some(last_segment) = type_path.path.segments.last() {
            if last_segment.ident == "Option" {
                if let PathArguments::AngleBracketed(params) = &last_segment.arguments {
                    return !params.args.is_empty(); // Option<T> has one arg
                }
            }
        }
    }
    false
}

fn extract_arg_type(field_ty: &Type, req: &Requirement) -> proc_macro2::TokenStream {
    match req {
        Requirement::Optional | Requirement::Conditional(_) | Requirement::OptionalIf(_) => {
            if let Type::Path(path) = field_ty {
                if let Some(seg) = path.path.segments.last() {
                    if seg.ident == "Option" {
                        if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                            if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
                                return quote! { #inner };
                            }
                        }
                    }
                }
            }
            quote! { #field_ty }
        }
        Requirement::Always | Requirement::Default => quote! { #field_ty },
    }
}

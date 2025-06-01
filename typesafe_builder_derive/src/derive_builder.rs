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

pub fn derive_builder_impl(input: Input) -> Result<TokenStream2, darling::Error> {
    let name = input.ident();
    let builder_name = Ident::new(&format!("{}Builder", name), name.span());

    let field_infos = extract_field_infos(&input)?;
    let n_fields = field_infos.len();
    let type_params = generate_type_params(n_fields, name.span());
    let default_generics: Vec<_> = (0..n_fields).map(|_| quote! { Empty }).collect();

    let builder_fields = generate_builder_fields(&field_infos, &type_params);
    let builder_initialization = generate_builder_initialization(&field_infos);
    let setter_methods =
        generate_setter_methods(&field_infos, &type_params, &builder_name, name.span());
    let build_impls = generate_build_methods(&field_infos, &builder_name, name);

    Ok(quote! {
        pub struct #builder_name < #( #type_params ),* > {
            #( #builder_fields )*
        }

        impl #builder_name < #( #default_generics ),* > {
            #[inline]
            pub fn new() -> Self {
                Self { #( #builder_initialization )* }
             }
        }

        impl < #( #type_params ),* > #builder_name < #( #type_params ),* > {
            #( #setter_methods )*
        }

        #( #build_impls )*
    })
}

fn extract_field_infos(
    builder_input: &Input,
) -> Result<Vec<(Ident, Type, Requirement)>, darling::Error> {
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

        field_infos.push((ident, field.ty().clone(), req));
    }

    Ok(field_infos)
}

fn generate_type_params(n_fields: usize, span: proc_macro2::Span) -> Vec<Ident> {
    (0..n_fields)
        .map(|i| Ident::new(&format!("T{}", i), span))
        .collect()
}

fn generate_builder_fields<'a>(
    field_infos: &'a [(Ident, Type, Requirement)],
    type_params: &'a [Ident],
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    field_infos
        .iter()
        .zip(type_params.iter())
        .map(|((ident, ty, req), tp)| {
            let phantom = Ident::new(&format!("_{}", ident), ident.span());
            match req {
                &Requirement::Always => {
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
    field_infos: &'a [(Ident, Type, Requirement)],
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    field_infos.iter().map(|(ident, _ty, _)| {
        let phantom = Ident::new(&format!("_{}", ident), ident.span());
        quote! {
            #ident : None,
            #phantom : std::marker::PhantomData,
        }
    })
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
        Requirement::Always => quote! { #field_ty },
    }
}

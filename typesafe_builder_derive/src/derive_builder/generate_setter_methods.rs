use crate::{derive_builder::extract_arg_type, input::Requirement};
use quote::quote;
use syn::{Generics, Ident, Type};

type FieldInfo = (Ident, Type, Requirement, Option<syn::Expr>, bool);

pub fn generate_setter_methods<'a>(
    field_infos: &'a [FieldInfo],
    type_params: &'a [Ident],
    builder_name: &'a Ident,
    generics: &'a Generics,
    span: proc_macro2::Span,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    field_infos.iter().enumerate().map(
        move |(idx, (field_ident, field_ty, req, _default, into_flag))| {
            let mut new_types = type_params.to_vec();
            new_types[idx] = Ident::new("_TypesafeBuilderFilled", span);

            let generic_params = &generics.params;
            let new_builder_ty = if generic_params.is_empty() {
                quote! { #builder_name < #( #new_types ),* > }
            } else {
                quote! { #builder_name < #generic_params, #( #new_types ),* > }
            };

            let inner_ty = extract_arg_type(field_ty, req);
            let arg_ty = if *into_flag {
                quote! { impl Into<#inner_ty> }
            } else {
                inner_ty.clone()
            };
            let param_name = if *into_flag {
                let name_str = field_ident.to_string();
                let capitalized = format!(
                    "{}{}",
                    name_str.chars().next().unwrap().to_uppercase(),
                    &name_str[1..]
                );
                Ident::new(&format!("Value{capitalized}"), field_ident.span())
            } else {
                Ident::new("VALUE", field_ident.span())
            };

            let setters_assign = field_infos.iter().map(|(fname, _ty, _, _default, _)| {
                let phantom = Ident::new(&format!("_{fname}"), fname.span());
                if fname == field_ident {
                    let value_expr = if *into_flag {
                        quote! { value.into() }
                    } else {
                        quote! { value }
                    };
                    match req {
                        Requirement::Optional => quote! {
                            #fname : Some(#value_expr),
                            #phantom : std::marker::PhantomData
                        },
                        Requirement::Conditional(_)
                        | Requirement::Always
                        | Requirement::OptionalIf(_)
                        | Requirement::Default => quote! {
                            #fname : Some(#value_expr),
                            #phantom : std::marker::PhantomData
                        },
                    }
                } else {
                    quote! { #fname : self.#fname, #phantom : self.#phantom }
                }
            });

            let function_name = Ident::new(&format!("with_{field_ident}"), field_ident.span());
            let builder_constructor = if generic_params.is_empty() {
                quote! { #builder_name::< #( #new_types ),* > }
            } else {
                quote! { #builder_name::< #generic_params, #( #new_types ),* > }
            };

            if *into_flag {
                quote! {
                    pub fn #function_name<#param_name>(self, value: #param_name) -> #new_builder_ty
                    where
                        #param_name: Into<#inner_ty>
                    {
                        #builder_constructor {
                            #( #setters_assign, )*
                        }
                    }
                }
            } else {
                quote! {
                    pub fn #function_name(self, value: #arg_ty) -> #new_builder_ty {
                        #builder_constructor {
                            #( #setters_assign, )*
                        }
                    }
                }
            }
        },
    )
}

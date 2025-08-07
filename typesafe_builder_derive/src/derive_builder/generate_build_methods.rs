use super::eval_condition;
use crate::input::Requirement;
use quote::quote;
use std::collections::HashMap;
use syn::{Generics, Ident, Type};

type FieldInfo = (Ident, Type, Requirement, Option<syn::Expr>, bool);

pub fn generate_build_methods(
    field_infos: &[FieldInfo],
    builder_name: &Ident,
    struct_name: &Ident,
    generics: &Generics,
) -> Vec<proc_macro2::TokenStream> {
    let n_fields = field_infos.len();
    let mut build_impls = Vec::new();
    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let generic_params = &generics.params;

    for mask in 0..(1_u32 << n_fields) {
        if !is_mask_valid(mask, field_infos) {
            continue;
        }

        let builder_generics = (0..n_fields)
            .map(|i| {
                if (mask & (1 << i)) != 0 {
                    quote! { _TypesafeBuilderFilled }
                } else {
                    quote! { _TypesafeBuilderEmpty }
                }
            })
            .collect::<Vec<_>>();

        let build_fields = field_infos
            .iter()
            .map(|(ident, _ty, req, default, _)| match req {
                Requirement::Always => {
                    quote! { #ident : self.#ident.unwrap() }
                }
                Requirement::Default => {
                    if let Some(default_expr) = default {
                        quote! { #ident : self.#ident.unwrap_or_else(|| #default_expr) }
                    } else {
                        quote! { #ident : self.#ident.unwrap() }
                    }
                }
                Requirement::Conditional(_) => {
                    quote! { #ident : self.#ident }
                }
                Requirement::Optional => quote! { #ident : self.#ident },
                Requirement::OptionalIf(_) => {
                    quote! { #ident : self.#ident }
                }
            });

        let impl_block = if generic_params.is_empty() {
            quote! {
                impl #builder_name < #( #builder_generics ),* > {
                    pub fn build(self) -> #struct_name {
                        #struct_name {
                            #( #build_fields, )*
                        }
                    }
                }
            }
        } else {
            quote! {
                impl < #generic_params > #builder_name < #generic_params, #( #builder_generics ),* > #where_clause {
                    pub fn build(self) -> #struct_name #ty_generics {
                        #struct_name {
                            #( #build_fields, )*
                        }
                    }
                }
            }
        };

        build_impls.push(impl_block);
    }

    build_impls
}

fn is_mask_valid(mask: u32, field_infos: &[FieldInfo]) -> bool {
    let _n_fields = field_infos.len();
    let mut var_map = HashMap::<String, bool>::new();

    for (idx, (ident, _, _, _, _)) in field_infos.iter().enumerate() {
        var_map.insert(ident.to_string(), (mask & (1 << idx)) != 0);
    }

    for (idx, (_, _, req, _default, _)) in field_infos.iter().enumerate() {
        match req {
            Requirement::Always => {
                if (mask & (1 << idx)) == 0 {
                    return false;
                }
            }
            Requirement::Default => {
                // Default fields can always be built (they have default values)
            }
            Requirement::Conditional(expr) => {
                let mut cond_vars = var_map.clone();
                let field_name = field_infos[idx].0.to_string();
                cond_vars.remove(&field_name);
                let needed = eval_condition(expr, &cond_vars);
                let filled = (mask & (1 << idx)) != 0;
                if needed && !filled {
                    return false;
                }
            }
            Requirement::Optional => {}
            Requirement::OptionalIf(expr) => {
                let mut cond_vars = var_map.clone();
                let field_name = field_infos[idx].0.to_string();
                cond_vars.remove(&field_name);
                let is_optional = eval_condition(expr, &cond_vars);
                let filled = (mask & (1 << idx)) != 0;

                if !is_optional && !filled {
                    return false;
                }
            }
        }
    }

    true
}

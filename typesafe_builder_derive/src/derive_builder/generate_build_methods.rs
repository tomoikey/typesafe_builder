use super::eval_condition;
use crate::input::Requirement;
use quote::quote;
use std::collections::HashMap;
use syn::{Ident, Type};

pub fn generate_build_methods(
    field_infos: &[(Ident, Type, Requirement)],
    builder_name: &Ident,
    struct_name: &Ident,
) -> Vec<proc_macro2::TokenStream> {
    let n_fields = field_infos.len();
    let mut build_impls = Vec::new();

    for mask in 0..(1_u32 << n_fields) {
        if !is_mask_valid(mask, field_infos) {
            continue;
        }

        let generics = (0..n_fields).map(|i| {
            if (mask & (1 << i)) != 0 {
                quote! { Filled }
            } else {
                quote! { Empty }
            }
        });

        let build_fields = field_infos.iter().map(|(ident, _ty, req)| match req {
            Requirement::Always => {
                quote! { #ident : self.#ident.unwrap() }
            }
            Requirement::Conditional(_) => {
                quote! { #ident : self.#ident }
            }
            Requirement::Optional => quote! { #ident : self.#ident },
            Requirement::OptionalIf(_) => {
                quote! { #ident : self.#ident }
            }
        });

        build_impls.push(quote! {
            impl #builder_name < #( #generics ),* > {
                pub fn build(self) -> #struct_name {
                    #struct_name {
                        #( #build_fields, )*
                    }
                }
            }
        });
    }

    build_impls
}

fn is_mask_valid(mask: u32, field_infos: &[(Ident, Type, Requirement)]) -> bool {
    let _n_fields = field_infos.len();
    let mut var_map = HashMap::<String, bool>::new();

    for (idx, (ident, _, _)) in field_infos.iter().enumerate() {
        var_map.insert(ident.to_string(), (mask & (1 << idx)) != 0);
    }

    for (idx, (_, _, req)) in field_infos.iter().enumerate() {
        match req {
            Requirement::Always => {
                if (mask & (1 << idx)) == 0 {
                    return false;
                }
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

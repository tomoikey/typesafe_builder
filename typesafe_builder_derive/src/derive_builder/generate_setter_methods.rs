use crate::{derive_builder::extract_arg_type, input::Requirement};
use quote::quote;
use syn::{Generics, Ident, Type};

pub fn generate_setter_methods<'a>(
    field_infos: &'a [(Ident, Type, Requirement)],
    type_params: &'a [Ident],
    builder_name: &'a Ident,
    generics: &'a Generics,
    span: proc_macro2::Span,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    field_infos
        .iter()
        .enumerate()
        .map(move |(idx, (field_ident, field_ty, req))| {
            let mut new_types = type_params.to_vec();
            new_types[idx] = Ident::new("_TypesafeBuilderFilled", span);

            let generic_params = &generics.params;
            let new_builder_ty = if generic_params.is_empty() {
                quote! { #builder_name < #( #new_types ),* > }
            } else {
                quote! { #builder_name < #generic_params, #( #new_types ),* > }
            };

            let arg_ty = extract_arg_type(field_ty, req);

            let setters_assign = field_infos.iter().map(|(fname, _ty, _)| {
                let phantom = Ident::new(&format!("_{}", fname), fname.span());
                if fname == field_ident {
                    match req {
                        Requirement::Optional => quote! {
                            #fname : Some(value),
                            #phantom : std::marker::PhantomData
                        },
                        Requirement::Conditional(_)
                        | Requirement::Always
                        | Requirement::OptionalIf(_) => quote! {
                            #fname : Some(value),
                            #phantom : std::marker::PhantomData
                        },
                    }
                } else {
                    quote! { #fname : self.#fname, #phantom : self.#phantom }
                }
            });

            let function_name = Ident::new(&format!("with_{}", field_ident), field_ident.span());
            let builder_constructor = if generic_params.is_empty() {
                quote! { #builder_name::< #( #new_types ),* > }
            } else {
                quote! { #builder_name::< #generic_params, #( #new_types ),* > }
            };

            quote! {
                pub fn #function_name(self, value: #arg_ty) -> #new_builder_ty {
                    #builder_constructor {
                        #( #setters_assign, )*
                    }
                }
            }
        })
}

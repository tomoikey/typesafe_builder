use crate::{derive_builder::extract_arg_type, input::Requirement};
use quote::quote;
use syn::{Ident, Type};

pub fn generate_setter_methods<'a>(
    field_infos: &'a [(Ident, Type, Requirement)],
    type_params: &'a [Ident],
    builder_name: &'a Ident,
    span: proc_macro2::Span,
) -> impl Iterator<Item = proc_macro2::TokenStream> + 'a {
    field_infos
        .iter()
        .enumerate()
        .map(move |(idx, (field_ident, field_ty, req))| {
            let mut new_types = type_params.to_vec();
            new_types[idx] = Ident::new("Filled", span);

            let new_builder_ty = quote! { #builder_name < #( #new_types ),* > };
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
            quote! {
                pub fn #function_name(self, value: #arg_ty) -> #new_builder_ty {
                    #builder_name::< #( #new_types ),* > {
                        #( #setters_assign, )*
                    }
                }
            }
        })
}

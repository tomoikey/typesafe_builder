use proc_macro::TokenStream;
use quote::quote;
use syn::{
    BinOp, Expr, ExprBinary, ExprPath, ExprUnary, Ident, PathArguments, Type, UnOp,
    parse_macro_input,
};

use darling::{FromDeriveInput, FromField, util::Flag};

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
struct BuilderField {
    ident: Option<Ident>,
    ty: Type,

    /// #[builder(optional)]
    #[darling(rename = "optional", default)]
    optional_flag: Flag,

    /// #[builder(required)]
    #[darling(rename = "required", default)]
    required_flag: Flag,

    /// #[builder(required_if"...")]
    #[darling(rename = "required_if", default)]
    required_if: Option<String>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
struct BuilderInput {
    ident: Ident,
    data: darling::ast::Data<(), BuilderField>,
}

enum Requirement {
    Always,
    Optional,
    Conditional(Expr),
}

impl BuilderField {
    fn requirement(&self) -> syn::Result<Requirement> {
        if let Some(req_if) = &self.required_if {
            let expr_result: Result<Expr, _> = syn::parse_str(&req_if);
            let expr: Expr = expr_result?;
            Ok(Requirement::Conditional(expr))
        } else if self.required_flag.is_present() {
            Ok(Requirement::Always)
        } else if self.optional_flag.is_present() {
            Ok(Requirement::Optional)
        } else {
            Err(syn::Error::new(self.ident.as_ref().unwrap().span(), "missing required field"))
        }
    }
}

fn eval_expr(expr: &Expr, vars: &std::collections::HashMap<String, bool>) -> bool {
    match expr {
        Expr::Path(ExprPath { path, .. }) => {
            let key = path.segments.last().unwrap().ident.to_string();
            let val = vars.get(&key).copied().unwrap_or(false);
            val
        }
        Expr::Paren(expr_paren) => eval_expr(&expr_paren.expr, vars),
        Expr::Unary(ExprUnary {
            op: UnOp::Not(_),
            expr,
            ..
        }) => {
            let v = !eval_expr(expr, vars);
            v
        }
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => {
            let l = eval_expr(left, vars);
            let r = eval_expr(right, vars);
            let res = match op {
                BinOp::And(_) => l && r,
                BinOp::Or(_) => l || r,
                _ => false,
            };

            res
        }
        _ => false,
    }
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::DeriveInput);

    let builder_input = match BuilderInput::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let name = &builder_input.ident;
    let builder_name = Ident::new(&format!("{}Builder", name), name.span());

    let mut field_infos = Vec::<(Ident, Type, Requirement)>::new();
    for field in builder_input
        .data
        .take_struct()
        .expect("only named structs are supported")
        .fields
    {
        let ident = field
            .ident
            .clone()
            .expect("darling guarantees named fields");
        let req = match field.requirement() {
            Ok(r) => r,
            Err(err) => return err.into_compile_error().into(),
        };
        field_infos.push((ident, field.ty.clone(), req));
    }

    let n_fields = field_infos.len();

    let type_params: Vec<_> = (0..n_fields)
        .map(|i| Ident::new(&format!("T{}", i), name.span()))
        .collect();

    let default_generics: Vec<_> = (0..n_fields).map(|_| quote! { Empty }).collect();

    let builder_fields =
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
                    &Requirement::Optional | &Requirement::Conditional(_) => {
                        quote! {
                            #ident : #ty,
                            #phantom : std::marker::PhantomData<#tp>,
                        }
                    }
                }
            });

    let default_inits = field_infos.iter().map(|(ident, _ty, _)| {
        let phantom = Ident::new(&format!("_{}", ident), ident.span());
        quote! {
            #ident : None,
            #phantom : std::marker::PhantomData,
        }
    });

    let setter_methods =
        field_infos
            .iter()
            .enumerate()
            .map(|(idx, (field_ident, field_ty, req))| {
                let mut new_types = type_params.clone();
                new_types[idx] = Ident::new("Filled", name.span());

                let new_builder_ty = quote! { #builder_name < #( #new_types ),* > };

                let arg_ty = match req {
                    Requirement::Optional | Requirement::Conditional(_) => {
                        if let Type::Path(path) = field_ty {
                            if let Some(seg) = path.path.segments.last() {
                                if seg.ident == "Option" {
                                    if let PathArguments::AngleBracketed(ab) = &seg.arguments {
                                        if let Some(syn::GenericArgument::Type(inner)) =
                                            ab.args.first()
                                        {
                                            quote! { #inner }
                                        } else {
                                            quote! { #field_ty }
                                        }
                                    } else {
                                        quote! { #field_ty }
                                    }
                                } else {
                                    quote! { #field_ty }
                                }
                            } else {
                                quote! { #field_ty }
                            }
                        } else {
                            quote! { #field_ty }
                        }
                    }
                    Requirement::Always => quote! { #field_ty },
                };

                let setters_assign = field_infos.iter().map(|(fname, _ty, _)| {
                    let phantom = Ident::new(&format!("_{}", fname), fname.span());
                    if fname == field_ident {
                        match req {
                            Requirement::Optional => quote! {
                                #fname : Some(value),
                                #phantom : std::marker::PhantomData
                            },
                            Requirement::Conditional(_) | Requirement::Always => quote! {
                                #fname : Some(value),
                                #phantom : std::marker::PhantomData
                            },
                        }
                    } else {
                        quote! { #fname : self.#fname, #phantom : self.#phantom }
                    }
                });

                let function_name =
                    Ident::new(&format!("with_{}", field_ident), field_ident.span());
                quote! {
                    pub fn #function_name(self, value: #arg_ty) -> #new_builder_ty {
                        #builder_name::< #( #new_types ),* > {
                            #( #setters_assign, )*
                        }
                    }
                }
            });

    let mut build_impls = Vec::new();
    for mask in 0..(1_u32 << n_fields) {
        let generics = (0..n_fields).map(|i| {
            if (mask & (1 << i)) != 0 {
                quote! { Filled }
            } else {
                quote! { Empty }
            }
        });

        let mut var_map = std::collections::HashMap::<String, bool>::new();
        for (idx, (ident, _, _)) in field_infos.iter().enumerate() {
            var_map.insert(ident.to_string(), (mask & (1 << idx)) != 0);
        }

        let mut ok = true;
        for (idx, (_, _, req)) in field_infos.iter().enumerate() {
            match req {
                Requirement::Always => {
                    if (mask & (1 << idx)) == 0 {
                        ok = false;
                        break;
                    }
                }
                Requirement::Conditional(expr) => {
                    let mut cond_vars = var_map.clone();
                    let field_name = field_infos[idx].0.to_string();
                    cond_vars.remove(&field_name);
                    let needed = eval_expr(expr, &cond_vars);
                    let filled = (mask & (1 << idx)) != 0;
                    if needed && !filled {
                        ok = false;
                        break;
                    }
                }
                Requirement::Optional => {}
            }
        }
        if !ok {
            continue;
        }

        let build_fields = field_infos.iter().map(|(ident, _ty, req)| match req {
            Requirement::Always => {
                quote! { #ident : self.#ident.unwrap() }
            }
            Requirement::Conditional(_) => {
                quote! { #ident : self.#ident }
            }
            Requirement::Optional => quote! { #ident : self.#ident },
        });

        build_impls.push(quote! {
            impl #builder_name < #( #generics ),* > {
                pub fn build(self) -> #name {
                    #name {
                        #( #build_fields, )*
                    }
                }
            }
        });
    }

    let expanded = quote! {
        pub struct #builder_name < #( #type_params ),* > {
            #( #builder_fields )*
        }

        impl #builder_name < #( #default_generics ),* > {
            #[inline]
            pub fn new() -> Self {
                Self { #( #default_inits )* }
             }
        }

        impl < #( #type_params ),* > #builder_name < #( #type_params ),* > {
            #( #setter_methods )*
        }

        #( #build_impls )*
    };

    TokenStream::from(expanded)
}

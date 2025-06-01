use darling::{util::Flag, FromField};
use syn::{Expr, Ident, Type};

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
pub struct BuilderField {
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

impl BuilderField {
    pub fn ident(&self) -> Option<&Ident> {
        self.ident.as_ref()
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
    
    pub fn requirement(&self) -> syn::Result<Requirement> {
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

pub enum Requirement {
    Always,
    Optional,
    Conditional(Expr),
}

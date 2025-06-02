use darling::{FromDeriveInput, FromField, util::Flag};
use syn::{Expr, Generics, Ident, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
pub struct Input {
    ident: Ident,
    generics: Generics,
    data: darling::ast::Data<(), InputField>,
}

impl Input {
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn generics(&self) -> &Generics {
        &self.generics
    }

    pub fn data(&self) -> darling::ast::Data<&(), &InputField> {
        self.data.as_ref()
    }
}

#[derive(Debug, FromField)]
#[darling(attributes(builder))]
pub struct InputField {
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

    /// #[builder(optional_if"...")]
    #[darling(rename = "optional_if", default)]
    optional_if: Option<String>,
}

impl InputField {
    pub fn ident(&self) -> Option<&Ident> {
        self.ident.as_ref()
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn requirement(&self) -> syn::Result<Requirement> {
        let attribute_count = self.optional_flag.is_present() as u8
            + self.required_flag.is_present() as u8
            + self.required_if.is_some() as u8
            + self.optional_if.is_some() as u8;

        if attribute_count > 1 {
            return Err(syn::Error::new(
                self.ident.as_ref().unwrap().span(),
                "Multiple builder attributes specified. Only one of required, optional, required_if, optional_if is allowed",
            ));
        }

        if let Some(opt_if) = &self.optional_if {
            let expr_result: Result<Expr, _> = syn::parse_str(opt_if);
            let expr: Expr = expr_result?;
            Ok(Requirement::OptionalIf(expr))
        } else if let Some(req_if) = &self.required_if {
            let expr_result: Result<Expr, _> = syn::parse_str(req_if);
            let expr: Expr = expr_result?;
            Ok(Requirement::Conditional(expr))
        } else if self.required_flag.is_present() {
            Ok(Requirement::Always)
        } else if self.optional_flag.is_present() {
            Ok(Requirement::Optional)
        } else {
            Err(syn::Error::new(
                self.ident.as_ref().unwrap().span(),
                "missing required field",
            ))
        }
    }
}

pub enum Requirement {
    Always,
    Optional,
    Conditional(Expr),
    OptionalIf(Expr),
}

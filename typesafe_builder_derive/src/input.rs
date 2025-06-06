use darling::{FromDeriveInput, FromField, util::Flag};
use syn::{Expr, Generics, Ident, Type};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder), supports(struct_named))]
pub struct Input {
    ident: Ident,
    generics: Generics,
    data: darling::ast::Data<(), InputField>,
    #[darling(rename = "name")]
    builder_name: Option<String>,
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

    pub fn builder_name(&self) -> String {
        self.builder_name
            .clone()
            .unwrap_or_else(|| format!("{}Builder", self.ident))
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

    /// #[builder(default = "...")]
    #[darling(rename = "default", default)]
    default: Option<Expr>,
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
            + self.optional_if.is_some() as u8
            + self.default.is_some() as u8;

        if attribute_count > 1 {
            return Err(syn::Error::new(
                self.ident.as_ref().unwrap().span(),
                "Multiple builder attributes specified. Only one of required, optional, required_if, optional_if, default is allowed",
            ));
        }

        if self.default.is_some() {
            Ok(Requirement::Default)
        } else if let Some(opt_if) = &self.optional_if {
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

    pub fn default(&self) -> Option<&Expr> {
        self.default.as_ref()
    }
}

pub enum Requirement {
    Always,
    Optional,
    Conditional(Expr),
    OptionalIf(Expr),
    Default,
}

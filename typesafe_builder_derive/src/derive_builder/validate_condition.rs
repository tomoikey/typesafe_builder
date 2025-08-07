use syn::{Expr, ExprBinary, ExprPath, ExprUnary};

pub fn validate_condition_fields(expr: &Expr, available_fields: &[String]) -> Result<(), String> {
    match expr {
        Expr::Path(ExprPath { path, .. }) => {
            let key = path.segments.last().unwrap().ident.to_string();
            if !available_fields.contains(&key) {
                return Err(format!("Field '{key}' used in condition does not exist"));
            }
            Ok(())
        }
        Expr::Paren(expr_paren) => validate_condition_fields(&expr_paren.expr, available_fields),
        Expr::Unary(ExprUnary { expr, .. }) => validate_condition_fields(expr, available_fields),
        Expr::Binary(ExprBinary { left, right, .. }) => {
            validate_condition_fields(left, available_fields)?;
            validate_condition_fields(right, available_fields)
        }
        _ => Ok(()),
    }
}

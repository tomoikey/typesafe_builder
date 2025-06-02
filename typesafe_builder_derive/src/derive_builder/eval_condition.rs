use std::collections::HashMap;
use syn::{BinOp, Expr, ExprBinary, ExprPath, ExprUnary, UnOp};

pub fn eval_condition(expr: &Expr, vars: &HashMap<String, bool>) -> bool {
    match expr {
        Expr::Path(ExprPath { path, .. }) => {
            let key = path.segments.last().unwrap().ident.to_string();

            vars.get(&key).copied().unwrap_or(false)
        }
        Expr::Paren(expr_paren) => eval_condition(&expr_paren.expr, vars),
        Expr::Unary(ExprUnary {
            op: UnOp::Not(_),
            expr,
            ..
        }) => !eval_condition(expr, vars),
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => {
            let l = eval_condition(left, vars);
            let r = eval_condition(right, vars);

            match op {
                BinOp::And(_) => l && r,
                BinOp::Or(_) => l || r,
                _ => false,
            }
        }
        _ => false,
    }
}

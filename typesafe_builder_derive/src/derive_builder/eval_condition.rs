use std::collections::HashMap;
use syn::{BinOp, Expr, ExprBinary, ExprPath, ExprUnary, UnOp};

pub fn eval_condition(expr: &Expr, vars: &HashMap<String, bool>) -> bool {
    match expr {
        Expr::Path(ExprPath { path, .. }) => {
            let key = path.segments.last().unwrap().ident.to_string();
            let val = vars.get(&key).copied().unwrap_or(false);
            val
        }
        Expr::Paren(expr_paren) => eval_condition(&expr_paren.expr, vars),
        Expr::Unary(ExprUnary {
            op: UnOp::Not(_),
            expr,
            ..
        }) => {
            let v = !eval_condition(expr, vars);
            v
        }
        Expr::Binary(ExprBinary {
            left, op, right, ..
        }) => {
            let l = eval_condition(left, vars);
            let r = eval_condition(right, vars);
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

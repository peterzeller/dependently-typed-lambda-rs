use crate::abstract_syntax::{Expr, Neutral, Value};
use std::borrow::Borrow;
use std::rc::Rc;
use Expr::*;

/// reduces the given expression to a value
/// implemented using simple reduction rules and substitution,
/// so not very efficient
pub fn eval(e: Rc<Expr>) -> Rc<Value> {
    match e.borrow() {
        Annotated { expr: e, .. } => eval(e.clone()),
        Var { name: n } => Rc::new(Value::Neutral(Neutral::Var { name: n.clone() })),
        App { func: f, arg: a } => match eval(f.clone()).borrow() {
            Value::Lambda {
                var_name: x,
                body: b,
            } => eval(subst_value(b, x, a.clone())),
            Value::Neutral(n) => Rc::new(Value::Neutral(Neutral::App {
                func: Rc::new(n.clone()),
                arg: eval(a.clone()),
            })),
        },
        Lambda {
            var_name: v,
            body: b,
        } => Rc::new(Value::Lambda {
            var_name: v.clone(),
            body: eval(b.clone()),
        }),
    }
}

/// substitutes variable v by replacement in the given value
fn subst_value(value: &Value, v: &String, replacement: Rc<Expr>) -> Rc<Expr> {
    match value {
        Value::Neutral(n) => subst_neutral(n, v, replacement),
        Value::Lambda {
            var_name: lv,
            body: b,
        } => {
            if v == lv {
                // if the variables are the same, the variable cannot appear free in the body,
                // so we don't substitute in the inner part

                // TODO replace dummy value "" with new function
                let vv = v.clone();
                let l = Lambda {
                    var_name: vv,
                    body: subst_value(b, &String::from(""), replacement),
                };
                Rc::new(l)
            } else {
                // otherwise, we need to make sure that the new variable
                // does not appear free in the replacement.
                // if that is the case, we have to rename v to avoid conflicts
                let mut new_lv = lv.clone();
                let mut i: i32 = 0;
                while is_free_var(replacement.borrow(), lv) {
                    i += 1;
                    new_lv = format!("{}_{}", lv, i)
                }

                Rc::new(Lambda {
                    var_name: new_lv.clone(),
                    body: subst_value(b, v, replacement),
                })
            }
        }
    }
}

/// substitutes variable v by replacement in the given neutral
fn subst_neutral(neutral: &Neutral, v: &String, replacement: Rc<Expr>) -> Rc<Expr> {
    match neutral {
        Neutral::Var { name: n } => {
            if n == v {
                replacement.clone()
            } else {
                Rc::new(Var { name: n.clone() })
            }
        }
        Neutral::App { func: f, arg: a } => Rc::new(App {
            func: subst_neutral(f, v, replacement.clone()),
            arg: subst_value(a, v, replacement.clone()),
        }),
    }
}

/// checks if v is a free variable in e
fn is_free_var(e: &Expr, v: &String) -> bool {
    match e {
        Expr::Var { name: x } => x == v,
        Expr::App { func: f, arg: a } => is_free_var(f, v) || is_free_var(a, v),
        Expr::Annotated { expr: e, .. } => is_free_var(e, v),
        Expr::Lambda {
            var_name: x,
            body: b,
        } => x != v && is_free_var(b, v),
    }
}

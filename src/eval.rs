use crate::abstract_syntax::Name;
use crate::abstract_syntax::{CheckableTerm, InferableTerm};
use std::borrow::Borrow;
use std::rc::Rc;
use InferableTerm::*;

/// reduces the given expression to a value
/// implemented using simple reduction rules and substitution,
/// so not very efficient
pub fn eval(e: &InferableTerm) -> Rc<Value> {
    eval_it(e, &Rc::new(Env::Empty))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Env {
    Empty,
    Cons(Rc<Value>, Rc<Env>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Neutral(Rc<Neutral>),
    Closure {
        env: Rc<Env>,
        body: Rc<CheckableTerm>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Neutral {
    Free { name: Name },
    App { func: Rc<Neutral>, arg: Rc<Value> },
}

impl Env {
    fn lookup(&self, index: i32) -> Option<Rc<Value>> {
        let mut c = self;
        for i in 0..index + 1 {
            match c {
                Env::Empty => return None,
                Env::Cons(v, rest) => {
                    if i == index {
                        return Some(v.clone());
                    } else {
                        c = rest
                    }
                }
            }
        }
        return None;
    }
}

fn eval_it(e: &InferableTerm, env: &Rc<Env>) -> Rc<Value> {
    match e.borrow() {
        Annotated { expr: e, .. } => eval_ct(&e, env),
        Free { name: n } => vfree(n),
        Bound { index: i } => match env.lookup(*i) {
            Some(v) => v,
            None => panic!("Could not find {:#?} in {:#?}", i, env),
        },
        App { func: f, arg: a } => {
            let fe = eval_it(f, env);
            let ae = eval_ct(a, env);
            vapp(&fe, &ae)
        }
    }
}

fn vapp(f: &Value, a: &Rc<Value>) -> Rc<Value> {
    match f {
        Value::Closure {
            body: b, env: e2, ..
        } => eval_ct(b, &Rc::new(Env::Cons(a.clone(), e2.clone()))),
        Value::Neutral(n) => Rc::new(Value::Neutral(Rc::new(Neutral::App {
            func: Rc::clone(n),
            arg: a.clone(),
        }))),
    }
}

fn eval_ct(e: &Rc<CheckableTerm>, env: &Rc<Env>) -> Rc<Value> {
    match e.borrow() {
        CheckableTerm::Inf(c) => eval_it(c, env),
        CheckableTerm::Lambda { body: b, .. } => Rc::new(Value::Closure {
            body: b.clone(),
            env: env.clone(),
        }),
    }
}

fn vfree(n: &Name) -> Rc<Value> {
    Rc::new(Value::Neutral(Rc::new(Neutral::Free { name: n.clone() })))
}

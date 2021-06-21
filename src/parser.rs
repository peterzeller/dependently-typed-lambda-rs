use crate::abstract_syntax::Name::Global;
use crate::abstract_syntax::Type;
use crate::abstract_syntax::{CheckableTerm, InferableTerm};
use crate::parser::InferableTerm::Bound;
use crate::parser::InferableTerm::Free;
use pest::Parser;
use std::rc::Rc;

#[derive(Parser)]
#[grammar = "lambda.pest"]
struct LambdaParser;

pub fn parse_string(input: &str) -> Result<Rc<InferableTerm>, String> {
    match LambdaParser::parse(Rule::start, input) {
        Err(e) => Err(format!("parse failed {:?}", e)),
        Ok(ast) => match transform_ast(ast).first() {
            None => Err(String::from("Could not extract AST")),
            Some(r) => Ok(r.clone()),
        },
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Env {
    Empty,
    Cons(String, Rc<Env>),
}

impl Env {
    fn lookup(&self, name: &String) -> Option<i32> {
        let mut c = self;
        let mut i = 0;
        loop {
            match c {
                Env::Empty => return None,
                Env::Cons(n, rest) => {
                    if *n == *name {
                        return Some(i);
                    } else {
                        i += 1;
                        c = rest
                    }
                }
            }
        }
    }
}

fn transform_ast(input: pest::iterators::Pairs<Rule>) -> Vec<Rc<InferableTerm>> {
    input
        .map(|p| transform_it(p, &Rc::new(Env::Empty)))
        .collect()
}

fn transform_it(p: pest::iterators::Pair<Rule>, env: &Rc<Env>) -> Rc<InferableTerm> {
    match p.as_rule() {
        Rule::start | Rule::expr | Rule::expr2 | Rule::expr3 | Rule::expr4 => {
            transform_it(p.into_inner().next().unwrap(), env)
        }
        Rule::parenExpr => transform_it(p.into_inner().next().unwrap(), env),
        Rule::app => {
            let mut inner = p.into_inner();
            let f = transform_it(inner.next().unwrap(), env);
            let x = transform_ct(inner.next().unwrap(), env);
            Rc::new(InferableTerm::App { func: f, arg: x })
        }
        Rule::var => {
            let name = String::from(p.as_str());
            match env.lookup(&name) {
                Some(i) => Rc::new(Bound { index: i }),
                None => Rc::new(Free { name: Global(name) }),
            }
        }
        Rule::annotated => {
            let mut inner = p.into_inner();
            Rc::new(InferableTerm::Annotated {
                expr: transform_ct(inner.next().unwrap(), env),
                typ: transform_type(inner.next().unwrap()),
            })
        }
        x => panic!("TODO {:#?}", x),
        //panic!("TODO {:#?}", p),
    }
}

fn transform_ct(p: pest::iterators::Pair<Rule>, env: &Rc<Env>) -> Rc<CheckableTerm> {
    match p.as_rule() {
        Rule::start | Rule::expr | Rule::expr2 | Rule::expr3 | Rule::expr4 => {
            transform_ct(p.into_inner().next().unwrap(), env)
        }
        Rule::parenExpr => transform_ct(p.into_inner().next().unwrap(), env),
        Rule::lambda => {
            let mut inner = p.into_inner();
            let v = inner.next().unwrap();
            let x = String::from(v.as_str());
            let b = transform_ct(
                inner.next().unwrap(),
                &Rc::new(Env::Cons(x.clone(), env.clone())),
            );
            Rc::new(CheckableTerm::Lambda {
                var_name: x,
                body: b,
            })
        }
        _ => {
            let it = transform_it(p, env);
            Rc::new(CheckableTerm::Inf(it))
        }
    }
}

fn transform_type(p: pest::iterators::Pair<Rule>) -> Rc<Type> {
    match p.as_rule() {
        Rule::typ | Rule::typ2 | Rule::paren_type => transform_type(p.into_inner().next().unwrap()),
        Rule::function_type => {
            let mut inner = p.into_inner();
            Rc::new(Type::Function {
                arg_type: transform_type(inner.next().unwrap()),
                res_type: transform_type(inner.next().unwrap()),
            })
        }
        Rule::base_type => {
            let x = String::from(p.as_str());
            Rc::new(Type::TFree(Global(x)))
        }
        x => panic!("TODO {:#?}", x),
    }
}

// enum Tok {
//     Eof,
//     Invalid(String),
//     Var(String),
//     ParenLeft,
//     ParenRight,
//     Lambda,
//     Colon,
//     Arrow,
//     Dot,
// }

// pub trait Stream<T> {
//     fn next(&mut self) -> Option<T>;
// }

// pub struct Lexer<'a, In: Stream<char>> {
//     input: &'a mut In,
// }

// impl<'a, In: Stream<char>> Stream<Tok> for Lexer<'a, In> {
//     fn next(&mut self) -> Option<Tok> {
//         match self.input.next() {
//             Option::None => Option::None,
//             Option::Some(c) => match c {
//                 '(' => Option::Some(Tok::ParenLeft),
//                 ')' => Option::Some(Tok::ParenRight),
//                 '.' => Option::Some(Tok::Dot),
//                 ':' => Option::Some(Tok::Colon),

//             },
//         }
//     }
// }

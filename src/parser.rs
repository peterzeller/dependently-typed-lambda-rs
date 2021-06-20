use crate::abstract_syntax::Expr;
use crate::abstract_syntax::Type;
use pest::Parser;
use std::rc::Rc;

#[derive(Parser)]
#[grammar = "lambda.pest"]
struct LambdaParser;

pub fn parse_string(input: &str) -> Result<Rc<Expr>, String> {
    match LambdaParser::parse(Rule::start, input) {
        Err(e) => Err(format!("parse failed {:?}", e)),
        Ok(ast) => match transform_ast(ast).first() {
            None => Err(String::from("Could not extract AST")),
            Some(r) => Ok(r.clone()),
        },
    }
}

fn transform_ast(input: pest::iterators::Pairs<Rule>) -> Vec<Rc<Expr>> {
    input.map(|p| transform_expr(p)).collect()
}

fn transform_expr(p: pest::iterators::Pair<Rule>) -> Rc<Expr> {
    match p.as_rule() {
        Rule::start | Rule::expr | Rule::expr2 | Rule::expr3 | Rule::expr4 => {
            transform_expr(p.into_inner().next().unwrap())
        }
        Rule::app => {
            let mut inner = p.into_inner();
            let f = transform_expr(inner.next().unwrap());
            let x = transform_expr(inner.next().unwrap());
            Rc::new(Expr::App { func: f, arg: x })
        }
        Rule::lambda => {
            let mut inner = p.into_inner();
            let v = inner.next().unwrap();
            let f = String::from(v.as_str());
            let b = transform_expr(inner.next().unwrap());
            Rc::new(Expr::Lambda {
                var_name: f,
                body: b,
            })
        }
        Rule::var => Rc::new(Expr::Var {
            name: String::from(p.as_str()),
        }),
        Rule::parenExpr => transform_expr(p.into_inner().next().unwrap()),
        Rule::annotated => {
            let mut inner = p.into_inner();
            Rc::new(Expr::Annotated {
                expr: transform_expr(inner.next().unwrap()),
                typ: transform_type(inner.next().unwrap()),
            })
        }
        x => panic!("TODO {:#?}", x),
        //panic!("TODO {:#?}", p),
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
            Rc::new(Type::Base)
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

use std::rc::Rc;
use crate::abstract_syntax::Value;
use crate::abstract_syntax::Neutral::Var;
use crate::abstract_syntax::Value::Neutral;
use crate::eval;
use crate::parse_string;

fn test_eval(input: &str, expected: Value) {
    let ast = parse_string(input).unwrap();
    let v = eval(ast);
    assert_eq!(v, Rc::new(expected))
}

#[test]
fn test_identity_func() {
    test_eval("(%x. x) y", Neutral(Var { name: String::from("y") }))
}


#[test]
fn test_annotated() {
    test_eval("((%x. x): 'a -> ('a)) y", Neutral(Var { name: String::from("y") }))
}

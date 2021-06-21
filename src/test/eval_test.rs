use crate::abstract_syntax::Name;
use crate::eval;
use crate::eval::Neutral::Free;
use crate::eval::Value;
use crate::parse_string;
use crate::test::eval_test::Value::Neutral;
use std::rc::Rc;

fn test_eval(input: &str, expected: Value) {
    let ast = parse_string(input).unwrap();
    let v = eval(&ast);
    assert_eq!(v, Rc::new(expected))
}

// #[test]
// fn test_identity_func() {
//     test_eval(
//         "((%x. x): 'a -> 'a) y",
//         Neutral(Rc::new(Free {
//             name: Name::Global(String::from("y")),
//         })),
//     )
// }

#[test]
fn test_annotated() {
    test_eval(
        "((%x. x): 'a -> ('a)) y",
        Neutral(Rc::new(Free {
            name: Name::Global(String::from("y")),
        })),
    )
}

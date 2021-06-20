use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Base,
    Function {
        arg_type: Rc<Type>,
        res_type: Rc<Type>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    Annotated { expr: Rc<Expr>, typ: Rc<Type> },
    Var { name: String },
    App { func: Rc<Expr>, arg: Rc<Expr> },
    Lambda { var_name: String, body: Rc<Expr> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Neutral(Neutral),
    Lambda { var_name: String, body: Rc<Value> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Neutral {
    Var { name: String },
    App { func: Rc<Neutral>, arg: Rc<Value> },
}

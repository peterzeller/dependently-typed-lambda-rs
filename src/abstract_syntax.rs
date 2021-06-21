use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    TFree(Name),
    Function {
        arg_type: Rc<Type>,
        res_type: Rc<Type>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferableTerm {
    Annotated {
        expr: Rc<CheckableTerm>,
        typ: Rc<Type>,
    },
    Bound {
        index: i32,
    },
    Free {
        name: Name,
    },
    App {
        func: Rc<InferableTerm>,
        arg: Rc<CheckableTerm>,
    },
    // TODO add Lambda with explicitly typed parameter
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckableTerm {
    Inf(Rc<InferableTerm>),
    Lambda {
        var_name: String,
        body: Rc<CheckableTerm>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Name {
    Global(String),
    Local(i32),
    Quote(i32),
}

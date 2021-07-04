use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InferableTerm {
    Annotated {
        expr: Rc<CheckableTerm>,
        typ: Rc<CheckableTerm>,
    },
    Star,
    Pi { var_name: String, arg_type: Rc<CheckableTerm>, result_type: Rc<CheckableTerm> },
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

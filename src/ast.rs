use crate::common::{Constant, Pattern, Ref, Symbol};

pub struct Fact(pub Symbol, pub Vec<Constant>);

pub struct Rule(pub Symbol, pub Vec<(Pattern, Query)>);

pub enum Value {
    Const(Constant),
    Ref(Ref),
}

pub enum Query {
    Apply(Symbol, Vec<Value>),
    // Let(Ref, Value),
}

pub enum TopLevel {
    Fact(Fact),
    Rule(Rule),
    Query(Query),
}

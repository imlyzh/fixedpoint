#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum Pattern {
    Const(Constant),
    Elim(Symbol, Vec<Pattern>),
    Ref(Ref),
    Ignore,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ref(pub Symbol);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(pub String);

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub enum Constant {
    None,
    Bool(bool),
    I64(i64),
    U64(u64),
    Atom(Symbol),
    // String(String),
    Construct(Symbol, Vec<Constant>),
}

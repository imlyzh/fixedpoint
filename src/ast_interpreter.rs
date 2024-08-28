use std::collections::HashMap;

use crate::{
    ast::{Query, Rule},
    common::{Constant, Ref, Symbol},
};

pub struct Context {
    pub facts: HashMap<(Symbol, usize), Constant>,
    pub rules: HashMap<(Symbol, usize), Rule>,
}

impl Context {
    pub fn query(self, q: Query) -> Option<Vec<Ref>> {
        match q {
            Query::Apply(sym, body) => {
                if let Some(fact) = self.facts.get(&(sym.clone(), body.len())) {
                    todo!()
                } else if let Some(rule) = self.rules.get(&(sym, body.len())) {
                    todo!()
                } else {
                    todo!()
                }
            } // Query::Let(sym, _) => todo!(),
        }
    }
}

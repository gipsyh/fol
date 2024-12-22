// use crate::{BvConst, ConstTerm, Term};
// use std::collections::HashMap;

// impl BvConst {
//     pub fn bitblast(&self) -> Vec<Term> {
//         self.c.iter().map(|c| Term::bool_const(*c)).collect()
//     }
// }

// impl ConstTerm {
//     pub fn bitblast(&self) -> Vec<Term> {
//         match self {
//             ConstTerm::BV(bv_const) => bv_const.bitblast(),
//             ConstTerm::Array(array_const) => todo!(),
//         }
//     }
// }

// impl Term {
//     pub fn bitblast(&self, map: &mut HashMap<Term, Vec<Term>>) -> Vec<Term> {
//         match self {
//             crate::TermInner::Const(const_term) => todo!(),
//             crate::TermInner::Var(var_term) => todo!(),
//             crate::TermInner::Op(op_term) => todo!(),
//         }
//     }
// }

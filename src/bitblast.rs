use crate::{BvConst, ConstTerm, Sort, Term, TermInner, TermManager, TermVec, VarTerm};
use logic_form::{DagCnfBuilder, Lit};
use std::{collections::HashMap, iter::repeat_with, ops::Deref};

impl BvConst {
    #[inline]
    pub fn bitblast(&self, tm: &mut TermManager) -> TermVec {
        self.c.iter().map(|c| tm.bool_const(*c)).collect()
    }

    #[inline]
    pub fn cnf_encode(&self) -> Lit {
        debug_assert!(self.bv_len() == 1);
        Lit::constant(self.c[0])
    }
}

impl ConstTerm {
    #[inline]
    pub fn bitblast(&self, tm: &mut TermManager) -> TermVec {
        match self {
            ConstTerm::BV(bv_const) => bv_const.bitblast(tm),
            ConstTerm::Array(_) => todo!(),
        }
    }

    #[inline]
    pub fn cnf_encode(&self) -> Lit {
        match self {
            ConstTerm::BV(bv_const) => bv_const.cnf_encode(),
            ConstTerm::Array(_) => todo!(),
        }
    }
}

impl VarTerm {
    #[inline]
    pub fn bitblast(&self, tm: &mut TermManager) -> TermVec {
        repeat_with(|| tm.new_var(Sort::bool_sort()))
            .take(self.sort.bv_len())
            .collect()
    }
}

impl Term {
    pub fn bitblast(&self, tm: &mut TermManager, map: &mut HashMap<Term, TermVec>) -> TermVec {
        if let Some(res) = map.get(self) {
            return res.clone();
        }
        let blast = match self.deref() {
            TermInner::Const(const_term) => const_term.bitblast(tm),
            TermInner::Var(var_term) => var_term.bitblast(tm),
            TermInner::Op(op_term) => {
                let terms: Vec<TermVec> =
                    op_term.terms.iter().map(|s| s.bitblast(tm, map)).collect();
                op_term.op.bitblast(tm, &terms)
            }
        };
        map.insert(self.clone(), blast.clone());
        map.get(self).unwrap().clone()
    }

    pub fn cnf_encode(&self, cb: &mut DagCnfBuilder, map: &mut HashMap<Term, Lit>) -> Lit {
        if let Some(res) = map.get(self) {
            return res.clone();
        }
        let blast = match self.deref() {
            TermInner::Const(const_term) => const_term.cnf_encode(),
            TermInner::Var(_) => cb.new_var().lit(),
            TermInner::Op(op_term) => {
                let terms: Vec<Lit> = op_term
                    .terms
                    .iter()
                    .map(|s| s.cnf_encode(cb, map))
                    .collect();
                op_term.op.cnf_encode(cb, &terms)
            }
        };
        map.insert(self.clone(), blast);
        *map.get(self).unwrap()
    }
}

pub fn bitblast_terms<'a, I: IntoIterator<Item = &'a Term>>(
    terms: I,
    tm: &mut TermManager,
    map: &mut HashMap<Term, TermVec>,
) -> impl Iterator<Item = TermVec> {
    terms.into_iter().map(|t| t.bitblast(tm, map))
}

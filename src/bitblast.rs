use crate::{BvConst, ConstTerm, Sort, Term, TermInner, TermManager, VarTerm};
use std::{collections::HashMap, iter::repeat_with, ops::Deref};

impl BvConst {
    #[inline]
    pub fn bitblast(&self, tm: &mut TermManager) -> Vec<Term> {
        self.c.iter().map(|c| tm.bool_const(*c)).collect()
    }
}

impl ConstTerm {
    #[inline]
    pub fn bitblast(&self, tm: &mut TermManager) -> Vec<Term> {
        match self {
            ConstTerm::BV(bv_const) => bv_const.bitblast(tm),
            ConstTerm::Array(_) => todo!(),
        }
    }
}

impl VarTerm {
    #[inline]
    pub fn bitblast(&self, tm: &mut TermManager) -> Vec<Term> {
        repeat_with(|| tm.new_var(Sort::bool_sort()))
            .take(self.sort.bv_len())
            .collect()
    }
}

impl Term {
    pub fn bitblast(&self, tm: &mut TermManager, map: &mut HashMap<Term, Vec<Term>>) -> Vec<Term> {
        if let Some(res) = map.get(self) {
            return res.clone();
        }
        let blast = match self.deref() {
            TermInner::Const(const_term) => const_term.bitblast(tm),
            TermInner::Var(vat_term) => vat_term.bitblast(tm),
            TermInner::Op(op_term) => {
                let terms: Vec<Vec<Term>> =
                    op_term.terms.iter().map(|s| s.bitblast(tm, map)).collect();
                op_term.op.bitblast(tm, &terms)
            }
        };
        map.insert(self.clone(), blast.clone());
        map.get(self).unwrap().clone()
    }
}

pub fn bitblast_terms<'a, I: IntoIterator<Item = &'a Term>>(
    terms: I,
    tm: &mut TermManager,
    map: &mut HashMap<Term, Vec<Term>>,
) -> impl Iterator<Item = Vec<Term>> {
    terms.into_iter().map(|t| t.bitblast(tm, map))
}

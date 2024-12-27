use super::define::define_non_core_op;
use super::{Concat, Eq, Ult};
use crate::{Term, TermManager};
use std::ops::Not as _;

define_non_core_op!(Neq, 2, neq_normalize);
fn neq_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    tm.new_op_term(Eq, terms).not()
}

define_non_core_op!(Redor, 1, redor_normalize);
fn redor_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    let len = terms[0].sort().bv_len();
    let zero = tm.bv_const_zero(len);
    neq_normalize(tm, &[terms[0].clone(), zero])
}

define_non_core_op!(Uext, 2, uext_normalize);
fn uext_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    tm.new_op_term(Concat, &[terms[1].clone(), terms[0].clone()])
}

define_non_core_op!(Ugte, 2, ugte_normalize);
fn ugte_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    !tm.new_op_term(Ult, terms)
}

// define_op!(Inc, 1, todo_bitblast);

// define_op!(Sub, 2, todo_bitblast);

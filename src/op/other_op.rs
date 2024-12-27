use super::define::define_non_core_op;
use super::{Concat, Eq, Ult, Xor};
use crate::{Term, TermManager};

define_non_core_op!(Neg, 1, neg_normalize);
fn neg_normalize(_tm: &mut TermManager, terms: &[Term]) -> Term {
    let term = &terms[0];
    !term + term.bv_const_one()
}

define_non_core_op!(Inc, 1, inc_normalize);
fn inc_normalize(_tm: &mut TermManager, terms: &[Term]) -> Term {
    &terms[0] + terms[0].bv_const_one()
}

define_non_core_op!(Dec, 1, dec_normalize);
fn dec_normalize(_tm: &mut TermManager, terms: &[Term]) -> Term {
    &terms[0] - terms[0].bv_const_one()
}

define_non_core_op!(Redand, 1, redand_normalize);
fn redand_normalize(_tm: &mut TermManager, terms: &[Term]) -> Term {
    let ones = terms[0].bv_const_ones();
    terms[0].op1(Eq, &ones)
}

define_non_core_op!(Redor, 1, redor_normalize);
fn redor_normalize(_tm: &mut TermManager, terms: &[Term]) -> Term {
    let zero = terms[0].bv_const_zero();
    terms[0].op1(Neq, &zero)
}

define_non_core_op!(Neq, 2, neq_normalize);
fn neq_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    !tm.new_op_term(Eq, terms)
}

define_non_core_op!(Xnor, 2, xnor_normalize);
fn xnor_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    !tm.new_op_term(Xor, terms)
}

define_non_core_op!(Uext, 2, uext_normalize);
fn uext_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    tm.new_op_term(Concat, &[terms[1].clone(), terms[0].clone()])
}

define_non_core_op!(Ugt, 2, ugt_normalize);
fn ugt_normalize(_tm: &mut TermManager, terms: &[Term]) -> Term {
    terms[1].op1(Ult, &terms[0])
}

define_non_core_op!(Ulte, 2, ulte_normalize);
fn ulte_normalize(_tm: &mut TermManager, terms: &[Term]) -> Term {
    !terms[1].op1(Ult, &terms[0])
}

define_non_core_op!(Ugte, 2, ugte_normalize);
fn ugte_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    !tm.new_op_term(Ult, terms)
}

define_non_core_op!(Sub, 2, sub_normalize);
fn sub_normalize(_tm: &mut TermManager, terms: &[Term]) -> Term {
    &terms[0] + -&terms[1]
}

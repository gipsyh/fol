use super::core_op;
use super::define::define_non_core_op;
use crate::{Term, TermManager};

define_non_core_op!(Neq, 2, neq_normalize);
fn neq_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    let eq = tm.new_op_term(core_op::Eq, terms);
    tm.new_op_term(core_op::Not, [&eq])
}

define_non_core_op!(Redor, 2, redor_normalize);
fn redor_normalize(tm: &mut TermManager, terms: &[Term]) -> Term {
    todo!()
}

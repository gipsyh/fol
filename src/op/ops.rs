use super::define::define_non_core_op;
use crate::{TermManager, TermVec, op::define::todo_bitblast};
use logic_form::{DagCnf, Lit};



define_op!(Inc, 1, todo_bitblast);

fn redor_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    TermVec::from([tm.new_op_terms_fold(Or, &terms[0])])
}
define_op!(Redor, 1, redor_bitblast);



define_op!(Ugte, 2, todo_bitblast);


fn uext_bitblast(_tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut res = terms[0].clone();
    res.extend_from_slice(&terms[1]);
    res
}
define_op!(Uext, 2, uext_bitblast);

define_op!(Sub, 2, todo_bitblast);

define_op!(Concat, 2, todo_bitblast);



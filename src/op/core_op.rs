use super::define::define_core_op;
use crate::{Sort, Term, TermManager, TermVec};
use logic_form::{DagCnf, Lit};

#[inline]
fn bool_sort(_terms: &[Term]) -> Sort {
    Sort::Bv(1)
}

define_core_op!(Not, 1, bitblast: not_bitblast, cnf_encode: not_cnf_encode);
fn not_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    terms[0].iter().map(|t| tm.new_op_term(Not, [t])).collect()
}
fn not_cnf_encode(_dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    !terms[0]
}

define_core_op!(And, 2, bitblast: and_bitblast, cnf_encode: and_cnf_encode);
fn and_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    tm.new_op_terms_elementwise(And, &terms[0], &terms[1])
}
fn and_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_and_rel(l, terms[0], terms[1]);
    l
}

define_core_op!(Or, 2, bitblast: or_bitblast, cnf_encode: or_cnf_encode);
fn or_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    tm.new_op_terms_elementwise(Or, &terms[0], &terms[1])
}
fn or_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_or_rel(l, terms[0], terms[1]);
    l
}

define_core_op!(Xor, 2, bitblast: xor_bitblast, cnf_encode: xor_cnf_encode);
fn xor_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    tm.new_op_terms_elementwise(Xor, &terms[0], &terms[1])
}
fn xor_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_xor_rel(l, terms[0], terms[1]);
    l
}

define_core_op!(Eq, 2, sort: bool_sort, bitblast: eq_bitblast, cnf_encode: eq_cnf_encode);
fn eq_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let neqs = tm.new_op_terms_elementwise(Eq, &terms[0], &terms[1]);
    TermVec::from([tm.new_op_terms_fold(And, &neqs)])
}
fn eq_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_xnor_rel(l, terms[0], terms[1]);
    l
}

define_core_op!(Ult, 2, sort: bool_sort, bitblast: ult_bitblast);
fn ult_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut res = tm.bool_const(false);
    for (x, y) in terms[0].iter().zip(terms[1].iter()) {
        res = (!x & y) | ((!x | y) & res)
    }
    TermVec::from([res])
}

define_core_op!(Ite, 3, sort: bool_sort, bitblast: ite_bitblast, cnf_encode: ite_cnf_encode);
fn ite_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut res = TermVec::new();
    for (x, y) in terms[1].iter().zip(terms[2].iter()) {
        res.push(tm.new_op_term(Ite, [&terms[0][0], x, y]));
    }
    res
}
fn ite_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_ite_rel(l, terms[0], terms[1], terms[2]);
    l
}

define_core_op!(Concat, 2, sort: concat_sort, bitblast: concat_bitblast);
fn concat_sort(terms: &[Term]) -> Sort {
    Sort::Bv(terms[0].bv_len() + terms[1].bv_len())
}
fn concat_bitblast(_tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut res = terms[1].clone();
    res.extend_from_slice(&terms[0]);
    res
}

define_core_op!(Add, 2, bitblast: add_bitblast);
fn add_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut c = tm.bool_const(false);
    let mut res = TermVec::new();
    for (x, y) in terms[0].iter().zip(terms[1].iter()) {
        res.push(tm.new_op_terms_fold(Xor, [x, y, &c]));
        let xy = tm.new_op_term(And, [x, y]);
        let xc = tm.new_op_term(And, [y, &c]);
        let yc = tm.new_op_term(And, [x, &c]);
        c = tm.new_op_terms_fold(Or, [&xy, &xc, &yc]);
    }
    res
}

use super::define::define_core_op;
use crate::{BvConst, Sort, Term, TermManager, TermResult, TermVec};
use logic_form::{DagCnf, Lit};

#[inline]
fn bool_sort(_terms: &[Term]) -> Sort {
    Sort::Bv(1)
}

define_core_op!(Not, 1, bitblast: not_bitblast, cnf_encode: not_cnf_encode, simplify: not_simplify);
fn not_simplify(tm: &mut TermManager, terms: &[Term]) -> TermResult {
    let x = &terms[0];
    if let Some(op) = x.op_term() {
        if op.op == Not {
            dbg!("not1");
            return TermResult::Some(op.terms[0].clone());
        }
    }
    if let Some(xc) = x.bv_const() {
        dbg!("not2");
        return TermResult::Some(tm.mk_bv_const(!xc));
    }
    TermResult::None
}
fn not_bitblast(_tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    terms[0].iter().map(|t| !t).collect()
}
fn not_cnf_encode(_dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    !terms[0]
}

define_core_op!(And, 2, bitblast: and_bitblast, cnf_encode: and_cnf_encode, simplify: and_simplify);
fn and_simplify(_tm: &mut TermManager, terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    let simp = |a: &Term, b: &Term| {
        if let Some(ac) = a.bv_const() {
            if ac.is_ones() {
                dbg!("and1");
                return TermResult::Some(b.clone());
            }
            if ac.is_zero() {
                dbg!("and2");
                return TermResult::Some(a.clone());
            }
        }
        if a == b {
            dbg!("and3");
            return TermResult::Some(a.clone());
        }
        if a == &!b {
            dbg!("and4");
            return TermResult::Some(a.mk_bv_const_zero());
        }
        TermResult::None
    };
    simp(x, y)?;
    simp(y, x)
}
fn and_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    tm.new_op_terms_elementwise(And, &terms[0], &terms[1])
}
fn and_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_and_rel(l, terms[0], terms[1]);
    l
}

define_core_op!(Or, 2, bitblast: or_bitblast, cnf_encode: or_cnf_encode, simplify: or_simplify);
fn or_simplify(_tm: &mut TermManager, terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    let simp = |a: &Term, b: &Term| {
        if let Some(ac) = a.bv_const() {
            if ac.is_ones() {
                dbg!("or1");
                return TermResult::Some(a.clone());
            }
            if ac.is_zero() {
                dbg!("or2");
                return TermResult::Some(b.clone());
            }
        }
        if a == b {
            dbg!("or3");
            return TermResult::Some(a.clone());
        }
        if a == &!b {
            dbg!("or4");
            return TermResult::Some(a.mk_bv_const_ones());
        }
        TermResult::None
    };
    simp(x, y)?;
    simp(y, x)
}
fn or_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    tm.new_op_terms_elementwise(Or, &terms[0], &terms[1])
}
fn or_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_or_rel(l, terms[0], terms[1]);
    l
}

define_core_op!(Xor, 2, bitblast: xor_bitblast, cnf_encode: xor_cnf_encode, simplify: xor_simplify);
fn xor_simplify(_tm: &mut TermManager, terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    let simp = |a: &Term, b: &Term| {
        if let Some(ac) = a.bv_const() {
            if ac.is_ones() {
                dbg!("xor1");
                return TermResult::Some(!b.clone());
            }
            if ac.is_zero() {
                dbg!("xor2");
                return TermResult::Some(b.clone());
            }
        }
        if a == b {
            dbg!("xor3");
            return TermResult::Some(a.mk_bv_const_zero());
        }
        if a == &!b {
            dbg!("xor4");
            return TermResult::Some(a.mk_bv_const_ones());
        }
        TermResult::None
    };
    simp(x, y)?;
    simp(y, x)
}
fn xor_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    tm.new_op_terms_elementwise(Xor, &terms[0], &terms[1])
}
fn xor_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_xor_rel(l, terms[0], terms[1]);
    l
}

define_core_op!(Eq, 2, sort: bool_sort, bitblast: eq_bitblast, cnf_encode: eq_cnf_encode, simplify: eq_simplify);
fn eq_simplify(tm: &mut TermManager, terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    let mut simp = |a: &Term, b: &Term| {
        if a.bv_len() == 1 {
            if let TermResult::Some(s) = xor_simplify(tm, terms) {
                dbg!("eq1");
                return TermResult::Some(!s);
            }
        }
        if a == b {
            dbg!("eq2");
            return TermResult::Some(tm.bool_const(true));
        }
        if a == &!b {
            dbg!("eq3");
            return TermResult::Some(tm.bool_const(false));
        }
        TermResult::None
    };
    simp(x, y)?;
    simp(y, x)
}
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

define_core_op!(Slt, 2, sort: bool_sort, bitblast: slt_bitblast);
fn slt_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let x = &terms[0];
    let y = &terms[1];
    let len = x.len();
    let (xr, xs) = (&x[..len - 1], &x[len - 1]);
    let (yr, ys) = (&y[..len - 1], &y[len - 1]);
    let ls = xs & !ys;
    let eqs = xs.op1(Eq, ys);
    let mut el = tm.bool_const(false);
    for (x, y) in xr.iter().zip(yr.iter()) {
        el = (!x & y) | ((!x | y) & el)
    }
    TermVec::from([ls | (eqs & el)])
}

fn get_shift_size(x: usize) -> usize {
    let mut pow2 = 1;
    let mut res = 0;
    while pow2 < x {
        pow2 *= 2;
        res += 1;
    }
    res
}

define_core_op!(Sll, 2, bitblast: sll_bitblast);
fn sll_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let (x, y) = (&terms[0], &terms[1]);
    assert!(x.len() == y.len());
    if terms[0].len() == 1 {
        return TermVec::from([&x[0] & !&y[0]]);
    }
    let width = x.len();
    let shift_size = get_shift_size(width);
    let mut res = x.clone();
    for shift_bit in 0..shift_size {
        let shift_step = 1 << shift_bit;
        let shift = &y[shift_bit];
        for j in 0..shift_step {
            res[j] = &!shift & &res[j];
        }
        for j in shift_step..width {
            res[j] = tm.new_op_term(Ite, [shift, &res[j - shift_step], &res[j]]);
        }
    }
    let width_bv = tm
        .bv_const_from_usize(width, width)
        .bv_const()
        .unwrap()
        .clone();
    let width_bv = width_bv.bitblast(tm);
    let less = &ult_bitblast(tm, &[terms[1].clone(), width_bv])[0];
    let f = tm.bool_const(false);
    for r in res.iter_mut() {
        *r = tm.new_op_term(Ite, [less, r, &f]);
    }
    res
}

define_core_op!(Srl, 2, bitblast: srl_bitblast);
fn srl_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let (x, y) = (&terms[0], &terms[1]);
    assert!(x.len() == y.len());
    if terms[0].len() == 1 {
        return TermVec::from([&x[0] & !&y[0]]);
    }
    let width = x.len();
    let shift_size = get_shift_size(width);
    let mut res = x.clone();
    for shift_bit in 0..shift_size {
        let shift_step = 1 << shift_bit;
        let shift = &y[shift_bit];
        for j in 0..width - shift_step {
            res[j] = tm.new_op_term(Ite, [shift, &res[j + shift_step], &res[j]]);
        }
        for j in width - shift_step..width {
            res[j] = &!shift & &res[j];
        }
    }
    let width_bv = tm
        .bv_const_from_usize(width, width)
        .bv_const()
        .unwrap()
        .clone();
    let width_bv = width_bv.bitblast(tm);
    let less = &ult_bitblast(tm, &[terms[1].clone(), width_bv])[0];
    let f = tm.bool_const(false);
    for r in res.iter_mut() {
        *r = tm.new_op_term(Ite, [less, r, &f]);
    }
    res
}

define_core_op!(Sra, 2, bitblast: sra_bitblast);
fn sra_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let (x, y) = (&terms[0], &terms[1]);
    assert!(x.len() == y.len());
    if terms[0].len() == 1 {
        return x.clone();
    }
    let width = x.len();
    let shift_size = get_shift_size(width);
    let mut res = x.clone();
    for shift_bit in 0..shift_size {
        let shift_step = 1 << shift_bit;
        let shift = &y[shift_bit];
        for j in 0..width - shift_step {
            res[j] = tm.new_op_term(Ite, [shift, &res[j + shift_step], &res[j]]);
        }
        for j in width - shift_step..width {
            res[j] = tm.new_op_term(Ite, [shift, &res[width - 1], &res[j]]);
        }
    }
    let width_bv = tm
        .bv_const_from_usize(width, width)
        .bv_const()
        .unwrap()
        .clone();
    let width_bv = width_bv.bitblast(tm);
    let less = &ult_bitblast(tm, &[terms[1].clone(), width_bv])[0];
    let sign = &x[width - 1];
    for r in res.iter_mut() {
        *r = tm.new_op_term(Ite, [less, r, &sign]);
    }
    res
}

define_core_op!(Ite, 3, sort: ite_sort, bitblast: ite_bitblast, cnf_encode: ite_cnf_encode, simplify: ite_simplify);
fn ite_sort(terms: &[Term]) -> Sort {
    terms[1].sort()
}
fn ite_simplify(_tm: &mut TermManager, terms: &[Term]) -> TermResult {
    let (c, t, e) = (&terms[0], &terms[1], &terms[2]);
    if let Some(cc) = c.bv_const() {
        dbg!("ite1");
        if cc.is_ones() {
            return TermResult::Some(t.clone());
        } else {
            return TermResult::Some(e.clone());
        }
    }
    if t == e {
        dbg!("ite2");
        return TermResult::Some(t.clone());
    }
    if t.bv_len() == 1 {
        if let Some(ec) = e.bv_const() {
            if ec.is_zero() {
                dbg!("ite3");
                return TermResult::Some(c & t);
            }
            if ec.is_ones() {
                dbg!("ite4");
                return TermResult::Some(!c | t);
            }
        }
        if let Some(tc) = t.bv_const() {
            if tc.is_zero() {
                dbg!("ite5");
                return TermResult::Some(!c & e);
            }
            if tc.is_ones() {
                dbg!("ite6");
                return TermResult::Some(c | e);
            }
        }
    }
    TermResult::None
}
fn ite_bitblast(_tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut res = TermVec::new();
    for (x, y) in terms[1].iter().zip(terms[2].iter()) {
        res.push(terms[0][0].op2(Ite, x, y));
    }
    res
}
fn ite_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_ite_rel(l, terms[0], terms[1], terms[2]);
    l
}

define_core_op!(Concat, 2, sort: concat_sort, bitblast: concat_bitblast, simplify: concat_simplify);
fn concat_simplify(tm: &mut TermManager, terms: &[Term]) -> TermResult {
    let x = &terms[0];
    let y = &terms[1];
    if let (Some(xc), Some(yc)) = (x.bv_const(), y.bv_const()) {
        let mut c = yc.c.clone();
        c.extend_from_slice(&xc.c);
        return TermResult::Some(tm.mk_bv_const(BvConst::new(&c)));
    }
    TermResult::None
}
fn concat_sort(terms: &[Term]) -> Sort {
    Sort::Bv(terms[0].bv_len() + terms[1].bv_len())
}
fn concat_bitblast(_tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut res = terms[1].clone();
    res.extend_from_slice(&terms[0]);
    res
}

define_core_op!(Sext, 2, sort: sext_sort, bitblast: sext_bitblast);
fn sext_sort(terms: &[Term]) -> Sort {
    Sort::Bv(terms[0].bv_len() + terms[1].bv_len())
}
fn sext_bitblast(_tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let x = &terms[0];
    let mut res = x.clone();
    let ext = vec![x[x.len() - 1].clone(); terms[1].len()];
    res.extend(ext);
    res
}

define_core_op!(Slice, 3, sort: slice_sort, bitblast: slice_bitblast);
fn slice_sort(terms: &[Term]) -> Sort {
    Sort::Bv(terms[1].bv_len() - terms[2].bv_len() + 1)
}
fn slice_bitblast(_tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let l = terms[2].len();
    let h = terms[1].len();
    terms[0][l..=h].iter().cloned().collect()
}

define_core_op!(Redxor, 1, sort: bool_sort, bitblast: redxor_bitblast);
fn redxor_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    TermVec::from([tm.new_op_terms_fold(Xor, terms[0].iter())])
}

#[inline]
fn full_adder(tm: &mut TermManager, x: &Term, y: &Term, c: &Term) -> (Term, Term) {
    let r = tm.new_op_terms_fold(Xor, [x, y, c]);
    let xy = x & y;
    let xc = x & c;
    let yc = y & c;
    let c = tm.new_op_terms_fold(Or, [&xy, &xc, &yc]);
    (r, c)
}

define_core_op!(Add, 2, bitblast: add_bitblast);
fn add_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut r;
    let mut c = tm.bool_const(false);
    let mut res = TermVec::new();
    for (x, y) in terms[0].iter().zip(terms[1].iter()) {
        (r, c) = full_adder(tm, x, y, &c);
        res.push(r);
    }
    res
}

define_core_op!(Mul, 2, bitblast: mul_bitblast);
fn mul_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let x = &terms[0];
    let y = &terms[1];
    assert!(x.len() == y.len());
    let len = x.len();
    let mut res: TermVec = x.iter().map(|t| t & &y[0]).collect();
    for i in 1..len {
        let mut c = tm.bool_const(false);
        for j in i..len {
            let add = &y[i] & &x[j - i];
            (res[j], c) = full_adder(tm, &res[j], &add, &c);
        }
    }
    res
}

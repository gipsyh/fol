use super::term::Term;
use crate::{TermManager, TermVec};
use lazy_static::lazy_static;
use logic_form::{DagCnf, Lit};
use std::collections::HashMap;
use std::{
    any::{TypeId, type_name},
    borrow::Borrow,
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

pub trait Op: Debug + 'static {
    #[inline]
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    #[inline]
    fn name(&self) -> &str {
        type_name::<Self>().split("::").last().unwrap()
    }

    fn num_operand(&self) -> usize;

    fn op(&self, _tm: &mut TermManager, _terms: &[Term]) -> Term {
        todo!()
    }

    fn bitblast(&self, _tm: &mut TermManager, _terms: &[TermVec]) -> TermVec {
        todo!()
    }

    fn cnf_encode(&self, _dc: &mut DagCnf, _terms: &[Lit]) -> Lit {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct DynOp {
    op: Rc<dyn Op>,
}

impl DynOp {
    #[inline]
    pub fn new(op: impl Op) -> Self {
        Self { op: Rc::new(op) }
    }
}

impl<T: Op> From<T> for DynOp {
    fn from(op: T) -> Self {
        Self::new(op)
    }
}

impl Deref for DynOp {
    type Target = dyn Op;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.op.borrow()
    }
}

impl Hash for DynOp {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.op.type_id().hash(state);
    }
}

impl PartialEq for DynOp {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.op.type_id() == other.op.type_id()
    }
}

impl Eq for DynOp {}

impl<O: Op> PartialEq<O> for DynOp {
    #[inline]
    fn eq(&self, other: &O) -> bool {
        self.op.type_id() == other.type_id()
    }
}

unsafe impl Send for DynOp {}

unsafe impl Sync for DynOp {}

macro_rules! define_op {
    ($name:ident, $num_operand:expr, $bitblast:expr, $cnf_encode:expr) => {
        #[derive(Hash, Debug, PartialEq, Clone, Copy)]
        pub struct $name;
        impl Op for $name {
            #[inline]
            fn num_operand(&self) -> usize {
                $num_operand
            }

            #[inline]
            fn bitblast(&self, tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
                debug_assert!(self.num_operand() == terms.len());
                $bitblast(tm, terms)
            }

            #[inline]
            fn cnf_encode(&self, dc: &mut DagCnf, terms: &[Lit]) -> Lit {
                dbg!(self);
                debug_assert!(self.num_operand() == terms.len());
                $cnf_encode(dc, terms)
            }
        }
    };
}

fn todo_bitblast(_tm: &mut TermManager, _terms: &[TermVec]) -> TermVec {
    todo!()
}
fn todo_cnf_encode(_dc: &mut DagCnf, _terms: &[Lit]) -> Lit {
    todo!()
}

fn not_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    terms[0].iter().map(|t| tm.new_op_term(Not, [t])).collect()
}
fn not_cnf_encode(_dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    !terms[0]
}
define_op!(Not, 1, not_bitblast, not_cnf_encode);
define_op!(Inc, 1, todo_bitblast, todo_cnf_encode);

fn neq_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let neqs = tm.new_op_terms_elementwise(Neq, &terms[0], &terms[1]);
    TermVec::from([tm.new_op_terms_fold(Or, &neqs)])
}
fn neq_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_xor_rel(l, terms[0], terms[1]);
    l
}
define_op!(Neq, 2, neq_bitblast, neq_cnf_encode);

fn or_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    tm.new_op_terms_elementwise(Or, &terms[0], &terms[1])
}
fn or_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_or_rel(l, terms[0], terms[1]);
    l
}
define_op!(Or, 2, or_bitblast, or_cnf_encode);

fn and_bitblast(tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    tm.new_op_terms_elementwise(And, &terms[0], &terms[1])
}
fn and_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_and_rel(l, terms[0], terms[1]);
    l
}
define_op!(And, 2, and_bitblast, and_cnf_encode);

fn uext_bitblast(_tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
    let mut res = terms[0].clone();
    res.extend_from_slice(&terms[1]);
    res
}
define_op!(Uext, 2, uext_bitblast, todo_cnf_encode);

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
define_op!(Add, 2, add_bitblast, todo_cnf_encode);

fn xor_cnf_encode(dc: &mut DagCnf, terms: &[Lit]) -> Lit {
    let l = dc.new_var().lit();
    dc.add_xor_rel(l, terms[0], terms[1]);
    l
}
define_op!(Xor, 2, todo_bitblast, xor_cnf_encode);

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
define_op!(Ite, 3, ite_bitblast, ite_cnf_encode);

macro_rules! insert_op {
    ($map:expr, $($type:tt),*) => {
        $(
            let op = DynOp::new($type);
            $map.insert(
                op.name().to_lowercase(),
                op,
            );
        )*
    };
}

lazy_static! {
    static ref OP_MAP: HashMap<String, DynOp> = {
        let mut m = HashMap::new();
        insert_op!(m, Not, Inc, Or, Neq, And, Uext, Add, Ite, Xor);
        m
    };
}

impl From<&str> for DynOp {
    #[inline]
    fn from(value: &str) -> Self {
        OP_MAP.get(&value.to_lowercase()).unwrap().clone()
    }
}

// #[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
// #[strum(serialize_all = "lowercase")]
// pub enum UniOpType {
//     Dec,
//     Neg,
//     Redand,
//     Redor,
//     Redxor,
// }

// #[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
// #[strum(serialize_all = "lowercase")]
// pub enum BiOpType {
//     Iff,
//     Implies,
//     Eq,
//     Neq,
//     Sgt,
//     Ugt,
//     Sgte,
//     Ugte,
//     Slt,
//     Ult,
//     Slte,
//     Ulte,
//     And,
//     Nand,
//     Nor,
//     Or,
//     Xnor,
//     Rol,
//     Ror,
//     Sll,
//     Sra,
//     Srl,
//     Add,
//     Mul,
//     Sdiv,
//     Udiv,
//     Smod,
//     Srem,
//     Urem,
//     Sub,
//     Saddo,
//     Uaddo,
//     Sdivo,
//     Udivo,
//     Smulo,
//     Umulo,
//     Ssubo,
//     Usubo,
//     Concat,
//     Read,
// }

// #[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
// #[strum(serialize_all = "lowercase")]
// pub enum TriOpType {
//     Ite,
//     Write,
// }

// #[derive(Debug, Copy, Clone, strum::EnumString, strum::Display, PartialEq, Eq, Hash)]
// #[strum(serialize_all = "lowercase")]
// pub enum ExtOpType {
//     Sext,
//     Uext,
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct SliceOp {
//     pub a: Term,
//     pub upper: u32,
//     pub lower: u32,
// }

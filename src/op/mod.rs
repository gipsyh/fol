mod core_op;
mod define;
mod other_op;

use super::term::Term;
use crate::{Sort, TermManager, TermResult, TermVec};
pub use core_op::*;
use lazy_static::lazy_static;
use logic_form::{DagCnf, Lit};
pub use other_op::*;
use std::collections::HashMap;
use std::fmt;
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

    #[inline]
    fn is_core(&self) -> bool {
        false
    }

    fn num_operand(&self) -> usize;

    #[inline]
    fn sort(&self, terms: &[Term]) -> Sort {
        terms[0].sort()
    }

    fn normalize(&self, _tm: &mut TermManager, _terms: &[Term]) -> Term {
        panic!("{:?} not support normalize", self);
    }

    fn simplify(&self, _tm: &mut TermManager, _terms: &[Term]) -> TermResult {
        TermResult::None
    }

    fn bitblast(&self, _tm: &mut TermManager, _terms: &[TermVec]) -> TermVec {
        panic!("{:?} not support biblast", self);
    }

    fn cnf_encode(&self, _dc: &mut DagCnf, _terms: &[Lit]) -> Lit {
        panic!("{:?} not support cnf_encode", self);
    }
}

#[derive(Clone)]
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
    #[inline]
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

impl Debug for DynOp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.op.fmt(f)
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

impl std::cmp::Eq for DynOp {}

impl<O: Op> PartialEq<O> for DynOp {
    #[inline]
    fn eq(&self, other: &O) -> bool {
        self.op.type_id() == other.type_id()
    }
}

unsafe impl Send for DynOp {}

unsafe impl Sync for DynOp {}

struct DynOpCollect(fn() -> DynOp);

inventory::collect!(DynOpCollect);

lazy_static! {
    static ref OP_MAP: HashMap<String, DynOp> = {
        let mut m = HashMap::new();
        for op in inventory::iter::<DynOpCollect> {
            let op = op.0();
            m.insert(op.name().to_lowercase(), op);
        }
        m
    };
}

impl From<&str> for DynOp {
    #[inline]
    fn from(value: &str) -> Self {
        OP_MAP
            .get(&value.to_lowercase())
            .unwrap_or_else(|| panic!("unsupport {value} op!"))
            .clone()
    }
}

// pub enum BiOpType {
//     Iff,
//     Nand,
//     Nor,
//     Rol,
//     Ror,
//     Sdiv,
//     Udiv,
//     Smod,
//     Srem,
//     Urem,
//     Saddo,
//     Uaddo,
//     Sdivo,
//     Udivo,
//     Smulo,
//     Umulo,
//     Ssubo,
//     Usubo,
//     Read,
// }

// pub enum TriOpType {
//     Write,
// }

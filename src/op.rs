use super::term::Term;
use lazy_static::lazy_static;
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

    #[inline]
    fn op(&self, _terms: &[Term]) -> Term {
        todo!()
    }

    fn bitblast(&self, _terms: &[Vec<Term>]) -> Vec<Term> {
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
    ($name:ident, $num_operand:expr, $bitblast:expr) => {
        #[derive(Hash, Debug, PartialEq, Clone, Copy)]
        pub struct $name;
        impl Op for $name {
            #[inline]
            fn num_operand(&self) -> usize {
                $num_operand
            }

            #[inline]
            fn bitblast(&self, terms: &[Vec<Term>]) -> Vec<Term> {
                $bitblast(terms)
            }
        }
    };
}

fn todo_bitblast(_terms: &[Vec<Term>]) -> Vec<Term> {
    todo!()
}

define_op!(Not, 1, todo_bitblast);
define_op!(Inc, 1, todo_bitblast);

fn neq_bitblast(terms: &[Vec<Term>]) -> Vec<Term> {
    let mut neqs = Vec::new();
    for i in 0..terms[0].len() {
        neqs.push(Term::new_op_term(Neq, &[
            terms[0][i].clone(),
            terms[1][i].clone(),
        ]))
    }
    let res = neqs[1..].iter().cloned().fold(neqs[0].clone(), |acc, neq| {
        Term::new_op_term(Or, &[acc, neq])
    });
    vec![res]
}

define_op!(Neq, 2, neq_bitblast);
define_op!(Or, 2, todo_bitblast);
define_op!(And, 2, todo_bitblast);
define_op!(Uext, 2, todo_bitblast);
define_op!(Add, 2, todo_bitblast);
define_op!(Ite, 3, todo_bitblast);

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
        insert_op!(m, Not, Inc, Or, Neq, And, Uext, Add, Ite);
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
//     Not,
//     Inc,
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
//     Xor,
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

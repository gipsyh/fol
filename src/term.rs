use super::{op::DynOp, sort::Sort};
use crate::TermVec;
use crate::op::{Add, And, Neg, Not, Or, Sub, Xor};
use giputils::grc::Grc;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash;
use std::ops::DerefMut;
use std::{hash::Hash, ops::Deref};

#[derive(Clone)]
pub struct Term {
    tm: TermManager,
    pub(crate) inner: Grc<TermInner>,
}

impl Term {
    #[inline]
    pub fn get_manager(&self) -> TermManager {
        self.tm.clone()
    }

    #[inline]
    pub fn sort(&self) -> Sort {
        self.inner.sort()
    }

    #[inline]
    pub fn bv_len(&self) -> usize {
        self.sort().bv_len()
    }

    #[inline]
    pub fn bv_const_zero(&self) -> Term {
        let mut tm = self.get_manager();
        tm.bv_const_zero(self.bv_len())
    }

    #[inline]
    pub fn bv_const_one(&self) -> Term {
        let mut tm = self.get_manager();
        tm.bv_const_one(self.bv_len())
    }

    #[inline]
    pub fn bv_const_ones(&self) -> Term {
        let mut tm = self.get_manager();
        tm.bv_const_ones(self.bv_len())
    }

    #[inline]
    pub fn op<'a>(
        &'a self,
        op: impl Into<DynOp>,
        terms: impl IntoIterator<Item = &'a Term>,
    ) -> Term {
        let mut tm = self.get_manager();
        tm.new_op_term(op.into(), [self].into_iter().chain(terms))
    }

    #[inline]
    pub fn op0(&self, op: impl Into<DynOp>) -> Term {
        let mut tm = self.get_manager();
        tm.new_op_term(op.into(), [self])
    }

    #[inline]
    pub fn op1(&self, op: impl Into<DynOp>, x: &Term) -> Term {
        let mut tm = self.get_manager();
        tm.new_op_term(op.into(), [self, x])
    }

    #[inline]
    pub fn op2(&self, op: impl Into<DynOp>, x: &Term, y: &Term) -> Term {
        let mut tm = self.get_manager();
        tm.new_op_term(op.into(), [self, x, y])
    }
}

impl Deref for Term {
    type Target = TermType;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner.ty
    }
}

impl Hash for Term {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl Debug for Term {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.deref().fmt(f)
    }
}

impl PartialEq for Term {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        debug_assert!(self.tm == other.tm);
        self.inner == other.inner
    }
}

impl Eq for Term {}

impl AsRef<Term> for Term {
    #[inline]
    fn as_ref(&self) -> &Term {
        self
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        let g = self.clone();
        self.tm.tgc.collect(g);
    }
}

macro_rules! impl_unary_ops {
    ($trait:ident, $method:ident, $op:expr) => {
        impl std::ops::$trait for Term {
            type Output = Term;

            #[inline]
            fn $method(self) -> Self::Output {
                self.op0($op)
            }
        }

        impl std::ops::$trait for &Term {
            type Output = Term;

            #[inline]
            fn $method(self) -> Self::Output {
                self.op0($op)
            }
        }
    };
}

impl_unary_ops!(Not, not, Not);
impl_unary_ops!(Neg, neg, Neg);

macro_rules! impl_biops {
    ($trait:ident, $method:ident, $op:expr) => {
        impl<T: AsRef<Term>> std::ops::$trait<T> for Term {
            type Output = Term;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                self.op1($op, rhs.as_ref())
            }
        }

        impl<T: AsRef<Term>> std::ops::$trait<T> for &Term {
            type Output = Term;

            #[inline]
            fn $method(self, rhs: T) -> Self::Output {
                self.op1($op, rhs.as_ref())
            }
        }
    };
}

impl_biops!(BitAnd, bitand, And);
impl_biops!(BitOr, bitor, Or);
impl_biops!(BitXor, bitxor, Xor);
impl_biops!(Add, add, Add);
impl_biops!(Sub, sub, Sub);

pub struct TermInner {
    sort: Sort,
    ty: TermType,
}

impl TermInner {
    #[inline]
    pub fn sort(&self) -> Sort {
        self.sort
    }
}

impl Debug for TermInner {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ty.fmt(f)
    }
}

impl Deref for TermInner {
    type Target = TermType;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TermType {
    Const(ConstTerm),
    Var(u32),
    Op(OpTerm),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BvConst {
    pub(crate) c: Vec<bool>,
}

impl BvConst {
    #[inline]
    pub fn new(c: &[bool]) -> Self {
        Self { c: c.to_vec() }
    }

    #[inline]
    pub fn bv_len(&self) -> usize {
        self.c.len()
    }
}

impl Debug for BvConst {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BvConst").field(&self.c).finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayConst {
    c: Vec<BvConst>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstTerm {
    BV(BvConst),
    Array(ArrayConst),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpTerm {
    pub op: DynOp,
    pub terms: Vec<Term>,
}

impl OpTerm {
    #[inline]
    fn new(op: impl Into<DynOp>, terms: Vec<Term>) -> Self {
        Self {
            op: op.into(),
            terms: terms.to_vec(),
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TermGC {
    garbage: Grc<Vec<Term>>,
}

impl TermGC {
    #[inline]
    pub fn collect(&mut self, term: Term) {
        self.garbage.push(term);
    }
}

#[derive(Default, Debug)]
pub struct TermManagerInner {
    tgc: TermGC,
    num_var: u32,
    map: HashMap<TermType, Term>,
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct TermManager {
    inner: Grc<TermManagerInner>,
}

impl TermManager {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn new_term(&mut self, ty: TermType, sort: Sort) -> Term {
        match self.map.get(&ty) {
            Some(term) => term.clone(),
            None => {
                let term = Term {
                    tm: self.clone(),
                    inner: Grc::new(TermInner {
                        sort,
                        ty: ty.clone(),
                    }),
                };
                self.map.insert(ty, term.clone());
                term
            }
        }
    }

    #[inline]
    pub fn bool_const(&mut self, c: bool) -> Term {
        let term = TermType::Const(ConstTerm::BV(BvConst::new(&[c])));
        self.new_term(term, Sort::Bv(1))
    }

    #[inline]
    pub fn bv_const(&mut self, c: &[bool]) -> Term {
        let term = TermType::Const(ConstTerm::BV(BvConst::new(c)));
        self.new_term(term, Sort::Bv(c.len()))
    }

    #[inline]
    pub fn bv_const_zero(&mut self, len: usize) -> Term {
        let c = vec![false; len];
        let term = TermType::Const(ConstTerm::BV(BvConst::new(&c)));
        self.new_term(term, Sort::Bv(len))
    }

    #[inline]
    pub fn bv_const_one(&mut self, len: usize) -> Term {
        let mut c = vec![false; len];
        c[0] = true;
        let term = TermType::Const(ConstTerm::BV(BvConst::new(&c)));
        self.new_term(term, Sort::Bv(len))
    }

    #[inline]
    pub fn bv_const_ones(&mut self, len: usize) -> Term {
        let c = vec![true; len];
        let term = TermType::Const(ConstTerm::BV(BvConst::new(&c)));
        self.new_term(term, Sort::Bv(len))
    }

    #[inline]
    pub fn new_op_term<'a>(
        &mut self,
        op: impl Into<DynOp>,
        terms: impl IntoIterator<Item = &'a Term>,
    ) -> Term {
        let op: DynOp = op.into();
        let terms: Vec<Term> = terms.into_iter().map(|t| (*t).clone()).collect();
        if !op.is_core() {
            return op.normalize(self, &terms);
        }
        let sort = op.sort(&terms);
        let term = TermType::Op(OpTerm::new(op, terms));
        self.new_term(term, sort)
    }

    #[inline]
    pub fn new_op_terms_fold<'a>(
        &mut self,
        op: impl Into<DynOp> + Copy,
        terms: impl IntoIterator<Item = &'a Term>,
    ) -> Term {
        let mut terms = terms.into_iter();
        let acc = terms.next().unwrap().clone();
        terms.fold(acc, |acc, x| self.new_op_term(op, &[acc, x.clone()]))
    }

    #[inline]
    pub fn new_op_terms_elementwise<'a>(
        &mut self,
        op: impl Into<DynOp> + Copy,
        x: impl IntoIterator<Item = &'a Term>,
        y: impl IntoIterator<Item = &'a Term>,
    ) -> TermVec {
        x.into_iter()
            .zip(y)
            .map(|(x, y)| self.new_op_term(op, [x, y]))
            .collect()
    }

    #[inline]
    pub fn new_var(&mut self, sort: Sort) -> Term {
        let id = self.num_var;
        self.num_var += 1;
        let term = TermType::Var(id);
        self.new_term(term, sort)
    }

    #[inline]
    pub fn garbage_collect(&mut self) {}

    #[inline]
    pub fn size(&self) -> usize {
        self.map.len()
    }
}

impl Deref for TermManager {
    type Target = TermManagerInner;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for TermManager {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Debug for TermManager {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TermManager")
            .field("size", &self.size())
            .finish()
    }
}

// impl PartialOrd for TermCube {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for TermCube {
//     #[inline]
//     fn cmp(&self, other: &Self) -> Ordering {
//         debug_assert!(self.is_sorted());
//         debug_assert!(other.is_sorted());
//         let min_index = self.len().min(other.len());
//         for i in 0..min_index {
//             match self[i].cmp(&other[i]) {
//                 Ordering::Less => return Ordering::Less,
//                 Ordering::Equal => {}
//                 Ordering::Greater => return Ordering::Greater,
//             }
//         }
//         self.len().cmp(&other.len())
//     }
// }

// impl FromIterator<Term> for TermCube {
//     #[inline]
//     fn from_iter<T: IntoIterator<Item = Term>>(iter: T) -> Self {
//         Self {
//             cube: Vec::from_iter(iter),
//         }
//     }
// }

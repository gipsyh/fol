use super::{op::DynOp, sort::Sort};
use giputils::grc::Grc;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash;
use std::{hash::Hash, ops::Deref};

#[derive(Clone)]
pub struct Term {
    tgc: TermGC,
    pub(crate) inner: Grc<TermInner>,
}

impl Deref for Term {
    type Target = TermInner;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
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
        f.debug_struct("Term").field("inner", &self.inner).finish()
    }
}

impl PartialEq for Term {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        debug_assert!(self.tgc == other.tgc);
        self.inner == other.inner
    }
}

impl Eq for Term {}

impl Drop for Term {
    fn drop(&mut self) {
        self.tgc.collect(self.clone());
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum TermInner {
    Const(ConstTerm),
    Var(VarTerm),
    Op(OpTerm),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BvConst {
    pub(crate) c: Vec<bool>,
}

impl BvConst {
    #[inline]
    pub fn new(c: &[bool]) -> Self {
        Self { c: c.to_vec() }
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
pub struct VarTerm {
    vid: u32,
    sort: Sort,
}

impl VarTerm {
    pub fn new(vid: u32, sort: Sort) -> Self {
        Self { vid, sort }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpTerm {
    op: DynOp,
    terms: Vec<Term>,
}

impl OpTerm {
    #[inline]
    fn new(op: impl Into<DynOp>, terms: &[Term]) -> Self {
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
pub struct TermManager {
    tgc: TermGC,
    num_var: u32,
    map: HashMap<TermInner, Term>,
}

impl TermManager {
    #[inline]
    pub fn new_term(&mut self, inner: TermInner) -> Term {
        match self.map.get(&inner) {
            Some(term) => term.clone(),
            None => {
                let term = Term {
                    tgc: self.tgc.clone(),
                    inner: Grc::new(inner.clone()),
                };
                self.map.insert(inner, term.clone());
                term
            }
        }
    }

    pub fn garbage_collect(&mut self) {}

    #[inline]
    pub fn bool_const(&mut self, c: bool) -> Term {
        let term = TermInner::Const(ConstTerm::BV(BvConst::new(&[c])));
        self.new_term(term)
    }

    pub fn bv_const(&mut self, c: &[bool]) -> Term {
        let term = TermInner::Const(ConstTerm::BV(BvConst::new(c)));
        self.new_term(term)
    }

    #[inline]
    pub fn new_op_term(&mut self, op: impl Into<DynOp>, terms: &[Term]) -> Term {
        let term = TermInner::Op(OpTerm::new(op, terms));
        self.new_term(term)
    }

    #[inline]
    pub fn new_var(&mut self, sort: Sort) -> Term {
        let id = self.num_var;
        self.num_var += 1;
        let term = TermInner::Var(VarTerm::new(id, sort));
        self.new_term(term)
    }
}

// impl Term {
//     #[inline]
//     fn new(sort: Sort, term: TermInner) -> Self {
//         if let Some(inner) = TERMMAP.get(&term) {
//             return Self { inner };
//         }
//         let inner = Grc::new(term.clone());
//         TERMMAP.insert(term, &inner, sort);
//         Self { inner }
//     }

//     #[inline]
//     pub fn term_id(&self) -> usize {
//         self.inner.as_ptr() as _
//     }

//     #[inline]
//     pub fn sort(&self) -> Sort {
//         TERMMAP.sort(self)
//     }

//     #[inline]
//     pub fn bool_const(v: bool) -> Self {
//         let term = TermInner::Const(Const::Bool(v));
//         Self::new(Sort::Bool, term)
//     }

//     #[inline]
//     pub fn bv_const(bv: &[bool]) -> Self {
//         if bv.len() == 1 {
//             return Self::bool_const(bv[0]);
//         }
//         let term = TermInner::Const(Const::BV(bv.to_vec()));
//         Self::new(Sort::BV(bv.len() as u32), term)
//     }

//     #[inline]
//     pub fn new_var(mut sort: Sort, id: usize) -> Self {
//         if let Sort::BV(w) = sort {
//             assert!(w > 0);
//             if w == 1 {
//                 sort = Sort::Bool;
//             }
//         }
//         let term = TermInner::Var(unsafe { NUM_VAR });
//         unsafe { NUM_VAR += 1 };
//         Self::new(sort, term)
//     }
// }

// impl Term {
//     #[inline]
//     pub fn uniop(&self, op: UniOpType) -> Self {
//         let op = UniOp {
//             ty: op,
//             a: self.clone(),
//         };
//         let sort = op.sort();
//         let term = TermInner::UniOp(op);
//         Self::new(sort, term)
//     }

//     #[inline]
//     pub fn not(&self) -> Self {
//         self.uniop(UniOpType::Not)
//     }

//     #[inline]
//     pub fn biop(&self, other: &Self, op: BiOpType) -> Self {
//         let op = BiOp {
//             ty: op,
//             a: self.clone(),
//             b: other.clone(),
//         };
//         let sort = op.sort();
//         let term = TermInner::BiOp(op);
//         Self::new(sort, term)
//     }

//     #[inline]
//     pub fn equal(&self, other: &Self) -> Self {
//         self.biop(other, BiOpType::Eq)
//     }

//     #[inline]
//     pub fn not_equal(&self, other: &Self) -> Self {
//         self.biop(other, BiOpType::Neq)
//     }

//     #[inline]
//     pub fn and(&self, other: &Self) -> Self {
//         self.biop(other, BiOpType::And)
//     }

//     #[inline]
//     pub fn or(&self, other: &Self) -> Self {
//         self.biop(other, BiOpType::Or)
//     }

//     #[inline]
//     pub fn add(&self, other: &Self) -> Self {
//         self.biop(other, BiOpType::Add)
//     }

//     #[inline]
//     pub fn triop(&self, x: &Self, y: &Self, op: TriOpType) -> Self {
//         let op = TriOp {
//             ty: op,
//             a: self.clone(),
//             b: x.clone(),
//             c: y.clone(),
//         };
//         let sort = op.sort();
//         let term = TermInner::TriOp(op);
//         Self::new(sort, term)
//     }

//     #[inline]
//     pub fn extop(&self, op: ExtOpType, length: u32) -> Self {
//         let op = ExtOp {
//             ty: op,
//             a: self.clone(),
//             length,
//         };
//         let sort = op.sort();
//         let term = TermInner::ExtOp(op);
//         Self::new(sort, term)
//     }

//     #[inline]
//     pub fn slice(&self, upper: u32, lower: u32) -> Self {
//         let op = SliceOp {
//             a: self.clone(),
//             upper,
//             lower,
//         };
//         let sort = op.sort();
//         let term = TermInner::SliceOp(op);
//         Self::new(sort, term)
//     }
// }

// impl Deref for Term {
//     type Target = TermInner;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         self.inner.deref()
//     }
// }

// impl Drop for Term {
//     #[inline]
//     fn drop(&mut self) {
//         if self.inner.count() == 1 {
//             self.inner.increment_count();
//             // TERMMAP.remove(&self.inner);
//         }
//     }
// }

// impl PartialOrd for Term {
//     #[inline]
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for Term {
//     #[inline]
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.term_id().cmp(&other.term_id())
//     }
// }

// unsafe impl Sync for Term {}

// unsafe impl Send for Term {}

// #[derive(Debug, PartialEq, Eq, Clone, Hash)]
// pub enum TermInner {
//     Const(Const),
//     Var(u32),
//     UniOp(UniOp),
//     BiOp(BiOp),
//     TriOp(TriOp),
//     ExtOp(ExtOp),
//     SliceOp(SliceOp),
// }

// unsafe impl Sync for TermInner {}

// unsafe impl Send for TermInner {}

// #[derive(Clone, Default, PartialEq, Eq)]
// pub struct TermCube {
//     cube: Vec<Term>,
// }

// impl Debug for TermCube {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.cube.fmt(f)
//     }
// }

// impl TermCube {
//     #[inline]
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn term(&self) -> Term {
//         let mut term = Term::bool_const(true);
//         for l in self.iter() {
//             term = term.and(l);
//         }
//         term
//     }

//     #[inline]
//     pub fn ordered_subsume(&self, cube: &TermCube) -> bool {
//         debug_assert!(self.is_sorted());
//         debug_assert!(cube.is_sorted());
//         if self.len() > cube.len() {
//             return false;
//         }
//         let mut j = 0;
//         for i in 0..self.len() {
//             while j < cube.len() && self[i] > cube[j] {
//                 j += 1;
//             }
//             if j == cube.len() || self[i] != cube[j] {
//                 return false;
//             }
//         }
//         true
//     }
// }

// impl Deref for TermCube {
//     type Target = Vec<Term>;

//     #[inline]
//     fn deref(&self) -> &Self::Target {
//         &self.cube
//     }
// }

// impl DerefMut for TermCube {
//     #[inline]
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.cube
//     }
// }

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

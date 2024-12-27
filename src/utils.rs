use crate::{Term, TermManager, op::DynOp};
use core::range::Range;
use std::{
    ops::{Deref, DerefMut, Index, IndexMut, RangeInclusive, RangeTo},
    slice, vec,
};

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TermVec {
    data: Vec<Term>,
}

impl TermVec {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn get_manager(&self) -> TermManager {
        self[0].get_manager()
    }

    #[inline]
    pub fn item(mut self) -> Term {
        self.data.pop().unwrap()
    }

    #[inline]
    pub fn fold(&self, op: impl Into<DynOp> + Copy) -> Term {
        let mut tm = self.get_manager();
        tm.new_op_terms_fold(op, self.iter())
    }

    // #[inline]
    // pub fn ordered_subsume(&self, cube: &TermCube) -> bool {
    //     debug_assert!(self.is_sorted());
    //     debug_assert!(cube.is_sorted());
    //     if self.len() > cube.len() {
    //         return false;
    //     }
    //     let mut j = 0;
    //     for i in 0..self.len() {
    //         while j < cube.len() && self[i] > cube[j] {
    //             j += 1;
    //         }
    //         if j == cube.len() || self[i] != cube[j] {
    //             return false;
    //         }
    //     }
    //     true
    // }
}

impl Deref for TermVec {
    type Target = Vec<Term>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for TermVec {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Index<usize> for TermVec {
    type Output = Term;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for TermVec {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<Range<usize>> for TermVec {
    type Output = [Term];

    #[inline]
    fn index(&self, index: Range<usize>) -> &Self::Output {
        self.data.index(index)
    }
}

impl Index<RangeTo<usize>> for TermVec {
    type Output = [Term];

    #[inline]
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        self.data.index(index)
    }
}

impl Index<RangeInclusive<usize>> for TermVec {
    type Output = [Term];

    #[inline]
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        self.data.index(index)
    }
}

impl FromIterator<Term> for TermVec {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Term>>(iter: T) -> Self {
        Self {
            data: Vec::from_iter(iter),
        }
    }
}

impl IntoIterator for TermVec {
    type Item = Term;
    type IntoIter = vec::IntoIter<Term>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a> IntoIterator for &'a TermVec {
    type Item = &'a Term;
    type IntoIter = slice::Iter<'a, Term>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<const N: usize> From<[Term; N]> for TermVec {
    #[inline]
    fn from(value: [Term; N]) -> Self {
        Self {
            data: Vec::from(value),
        }
    }
}

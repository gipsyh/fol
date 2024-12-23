use crate::Term;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct TermCube {
    cube: Vec<Term>,
}

impl TermCube {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fold_term(&self) -> Term {
        todo!()
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

impl Deref for TermCube {
    type Target = Vec<Term>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.cube
    }
}

impl DerefMut for TermCube {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cube
    }
}

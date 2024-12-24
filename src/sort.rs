use std::fmt::{self, Debug};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sort {
    Bv(usize),
    Array(usize, usize),
}

impl Sort {
    #[inline]
    pub fn bool_sort() -> Self {
        Sort::Bv(1)
    }

    #[inline]
    pub fn bv_len(&self) -> usize {
        if let Sort::Bv(w) = self { *w } else { panic!() }
    }
}

impl Debug for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sort::Bv(w) => write!(f, "Bv{}", w),
            Sort::Array(w, d) => write!(f, "Array{},{}", w, d),
        }
    }
}

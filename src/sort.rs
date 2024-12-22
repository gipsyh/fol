#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

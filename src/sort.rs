#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sort {
    Bv(u32),
    Array(u32, u32),
}

impl Sort {
    #[inline]
    pub fn bv_len(&self) -> u32 {
        if let Sort::Bv(w) = self { *w } else { panic!() }
    }
}

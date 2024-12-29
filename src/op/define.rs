macro_rules! op_trait_impl {
    (sort $impl:expr) => {
        #[inline]
        fn sort(&self, terms: &[crate::Term]) -> crate::Sort {
            debug_assert!(self.num_operand() == terms.len());
            $impl(terms)
        }
    };
    (normalize $impl:expr) => {
        #[inline]
        fn normalize(&self, tm: &mut crate::TermManager, terms: &[crate::Term]) -> crate::Term {
            debug_assert!(self.num_operand() == terms.len());
            $impl(tm, terms)
        }
    };
    (simplify $impl:expr) => {
        #[inline]
        fn simplify(
            &self,
            tm: &mut crate::TermManager,
            terms: &[crate::Term],
        ) -> crate::TermResult {
            debug_assert!(self.num_operand() == terms.len());
            $impl(tm, terms)
        }
    };
    (bitblast $impl:expr) => {
        #[inline]
        fn bitblast(
            &self,
            tm: &mut crate::TermManager,
            terms: &[crate::TermVec],
        ) -> crate::TermVec {
            debug_assert!(self.num_operand() == terms.len());
            $impl(tm, terms)
        }
    };
    (cnf_encode $impl:expr) => {
        #[inline]
        fn cnf_encode(
            &self,
            dc: &mut logic_form::DagCnf,
            terms: &[logic_form::Lit],
        ) -> logic_form::Lit {
            debug_assert!(self.num_operand() == terms.len());
            $impl(dc, terms)
        }
    };
}

macro_rules! define_core_op {
    ($name:ident, $num_operand:expr, $($be_impl:ident: $impl:expr),*) => {
        #[derive(Hash, Debug, PartialEq, Clone, Copy)]
        pub struct $name;
        inventory::submit! {crate::op::DynOpCollect(|| crate::op::DynOp::new($name))}
        impl crate::op::Op for $name {
            #[inline]
            fn num_operand(&self) -> usize {
                $num_operand
            }

            #[inline]
            fn is_core(&self) -> bool {
                true
            }

            #[inline]
            fn normalize(&self, _tm: &mut crate::TermManager, _terms: &[crate::Term]) -> crate::Term {
                panic!("{:?} not support normalize", self);
            }

            $(
                crate::op::define::op_trait_impl!($be_impl $impl);
            )*
        }
    };
}

macro_rules! define_non_core_op {
    ($name:ident, $num_operand:expr) => {
        #[derive(Hash, Debug, PartialEq, Clone, Copy)]
        pub struct $name;
        inventory::submit! {crate::op::DynOpCollect(|| crate::op::DynOp::new($name))}
        impl crate::op::Op for $name {
            #[inline]
            fn num_operand(&self) -> usize {
                $num_operand
            }
        }
    };
    ($name:ident, $num_operand:expr, $normalize:expr) => {
        #[derive(Hash, Debug, PartialEq, Clone, Copy)]
        pub struct $name;
        inventory::submit! {crate::op::DynOpCollect(|| crate::op::DynOp::new($name))}
        impl crate::op::Op for $name {
            #[inline]
            fn num_operand(&self) -> usize {
                $num_operand
            }

            #[inline]
            fn sort(&self, _terms: &[crate::Term]) -> crate::Sort {
                panic!("{:?} not support sort", self);
            }

            #[inline]
            fn normalize(&self, tm: &mut crate::TermManager, terms: &[crate::Term]) -> crate::Term {
                debug_assert!(self.num_operand() == terms.len());
                $normalize(tm, terms)
            }
        }
    };
}

pub(crate) use define_core_op;
pub(crate) use define_non_core_op;
pub(crate) use op_trait_impl;

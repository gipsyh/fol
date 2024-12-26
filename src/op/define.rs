macro_rules! define_core_op {
    ($name:ident, $num_operand:expr, $bitblast:expr, $cnf_encode:expr) => {
        #[derive(Hash, Debug, PartialEq, Clone, Copy)]
        pub struct $name;
        inventory::submit! {crate::op::DynOpCollect(|| crate::op::DynOp::new($name))}
        impl crate::op::Op for $name {
            #[inline]
            fn num_operand(&self) -> usize {
                $num_operand
            }

            #[inline]
            fn normalize(&self, tm: &mut crate::TermManager, terms: &[crate::Term]) -> crate::Term {
                debug_assert!(self.num_operand() == terms.len());
                tm.new_op_term($name, terms)
            }

            #[inline]
            fn bitblast(
                &self,
                tm: &mut crate::TermManager,
                terms: &[crate::TermVec],
            ) -> crate::TermVec {
                debug_assert!(self.num_operand() == terms.len());
                $bitblast(tm, terms)
            }

            #[inline]
            fn cnf_encode(
                &self,
                dc: &mut logic_form::DagCnf,
                terms: &[logic_form::Lit],
            ) -> logic_form::Lit {
                debug_assert!(self.num_operand() == terms.len());
                $cnf_encode(dc, terms)
            }
        }
    };
    ($name:ident, $num_operand:expr, $bitblast:expr) => {
        #[derive(Hash, Debug, PartialEq, Clone, Copy)]
        pub struct $name;
        inventory::submit! {crate::op::DynOpCollect(|| crate::op::DynOp::new($name))}
        impl crate::op::Op for $name {
            #[inline]
            fn num_operand(&self) -> usize {
                $num_operand
            }

            #[inline]
            fn normalize(&self, tm: &mut crate::TermManager, terms: &[crate::Term]) -> crate::Term {
                debug_assert!(self.num_operand() == terms.len());
                tm.new_op_term($name, terms)
            }

            #[inline]
            fn bitblast(
                &self,
                tm: &mut crate::TermManager,
                terms: &[crate::TermVec],
            ) -> crate::TermVec {
                debug_assert!(self.num_operand() == terms.len());
                $bitblast(tm, terms)
            }
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
            fn normalize(&self, tm: &mut crate::TermManager, terms: &[crate::Term]) -> crate::Term {
                debug_assert!(self.num_operand() == terms.len());
                $normalize(tm, terms)
            }
        }
    };
}

pub(crate) use define_core_op;
pub(crate) use define_non_core_op;

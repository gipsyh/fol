use crate::{TermManager, TermVec};
use logic_form::{DagCnf, Lit};

macro_rules! define_op {
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
            fn bitblast(&self, tm: &mut TermManager, terms: &[TermVec]) -> TermVec {
                debug_assert!(self.num_operand() == terms.len());
                $bitblast(tm, terms)
            }

            #[inline]
            fn cnf_encode(&self, dc: &mut DagCnf, terms: &[Lit]) -> Lit {
                debug_assert!(self.num_operand() == terms.len());
                $cnf_encode(dc, terms)
            }
        }
    };
}

pub fn todo_bitblast(_tm: &mut TermManager, _terms: &[TermVec]) -> TermVec {
    todo!()
}
pub fn todo_cnf_encode(_dc: &mut DagCnf, _terms: &[Lit]) -> Lit {
    todo!()
}

pub(crate) use define_op;

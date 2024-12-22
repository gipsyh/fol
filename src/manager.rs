use crate::{Term, TermInner};
use giputils::grc::Grc;
use std::{cell::RefCell, collections::HashMap};

lazy_static::lazy_static! {
    pub static ref GLOBAL_TERM_MANAGER: Grc<RefCell<TermManager>> = Grc::new(RefCell::new(TermManager::default()));
}

#[derive(Default)]
pub struct TermManager {
    map: HashMap<TermInner, Term>,
}

impl TermManager {
    pub fn new_term(&mut self, inner: TermInner) -> Term {
        match self.map.get(&inner) {
            Some(term) => term.clone(),
            None => {
                let term = Term {
                    inner: Grc::new(inner.clone()),
                };
                self.map.insert(inner, term.clone());
                term
            }
        }
    }

    pub fn garbage_collect(&mut self) {}
}

pub fn gtm_new_term(inner: TermInner) -> Term {
    GLOBAL_TERM_MANAGER.borrow_mut().new_term(inner)
}


use bitvec::{vec::BitVec};

use yew::prelude::*;

use crate::{load, store, url::parse_url};

#[derive(Debug, Default, Properties, PartialEq, Eq, Clone)]
pub struct Collection {
    pub contains: BitVec,
}

#[derive(Debug)]
pub enum CollectionAction {
    Toggle(usize),
}

impl Collection {
    pub fn new() -> Self {
        let collection = parse_url().2;
        
        if let Some(contains) = collection {
            Self { contains }
        } else if let Some(contains) = load("collection") {
            Self { contains }
        } else {
            Self::default()
        }
    }

    pub fn toggle(&mut self, card_id: usize) {
        let contains = &mut self.contains;
        while contains.len() <= card_id {
            contains.push(false);
        }
        let Some(mut contain) = contains.get_mut(card_id) else {
                    return ; 
                };
        *contain = !(contain.as_ref());
    }

    pub fn get(&self, card_id: usize) -> Option<bool> {
        self.contains.get(card_id).map(|v| *v.as_ref()).to_owned()
    }
}

impl Reducible for Collection {
    type Action = CollectionAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            CollectionAction::Toggle(card_id) => {
                let mut contains = (*self).clone();
                contains.toggle(card_id);
                store("collection", &contains.contains);

                contains.into()
            }
        }
    }
}


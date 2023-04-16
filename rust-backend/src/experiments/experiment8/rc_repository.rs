use std::{collections::HashSet, hash::Hash, rc::Rc};

pub struct RcRepository<Item>(HashSet<Rc<Item>>);

impl<Item: Eq + Hash> RcRepository<Item> {
    pub fn new() -> Self {
        RcRepository(HashSet::new())
    }
    pub fn get_or_create(&mut self, item: Item) -> Rc<Item> {
        if let Some(existing) = self.0.get(&item) {
            existing.clone()
        } else {
            let new = Rc::new(item);
            self.0.insert(new.clone());
            new
        }
    }
}

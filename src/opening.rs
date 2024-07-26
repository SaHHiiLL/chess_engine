// PLAN:
// A datastrcuture that had "X" amounts of Starting Nodes -
//

use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
struct Node<T: Copy + Clone + Eq + Hash> {
    children: HashMap<T, Node<T>>,
    // refers to end of opening
    is_end: bool,
}

impl<T: Copy + Clone + Eq + Hash> Default for Node<T> {
    fn default() -> Self {
        Self {
            children: Default::default(),
            is_end: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct OpeningDatabase<T: Copy + Eq + Clone + Hash> {
    root: Node<T>,
}

impl<T: Copy + Clone + Eq + Hash> OpeningDatabase<T> {
    fn insert(&mut self, items: &[T]) {
        let mut curr = &mut self.root;
        for i in items.iter() {
            curr = self.root.children.entry(*i).or_default();
        }
        curr.is_end = true;
    }

    fn is_child(&mut self, needle: &T) -> bool {
        self.root.children.contains_key(needle)
    }

    fn change_root(&mut self, new_root_value: T) -> bool {
        if let Some(new_root) = self.root.children.get(&new_root_value) {
            self.root = new_root.clone();
            true
        } else {
            false
        }
    }
}

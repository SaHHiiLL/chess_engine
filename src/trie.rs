use std::{collections::HashMap, fmt::Display, hash::Hash};

#[derive(Debug, Clone)]
pub struct Node<T: Copy + Clone + Eq + Hash> {
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

impl<T: Copy + Clone + Hash + Eq + Display> Node<T> {
    fn print(&self, prefix: &str) {
        for (ch, node) in &self.children {
            let new_prefix = format!("{}{}", prefix, ch);
            println!("a{}", new_prefix);
        }
    }

    fn pprint(&self, prefix: &str, level: usize) {
        let indent = "  ".repeat(level);
        println!("{}{}", indent, prefix);
        if self.is_end {
            println!("{}[END]", indent);
        }
        for (ch, node) in &self.children {
            node.pprint(&ch.to_string(), level + 1);
        }
    }

    pub fn childern(&self) -> &HashMap<T, Node<T>> {
        &self.children
    }
}

#[derive(Debug, Clone)]
pub struct Trie<T: Copy + Eq + Clone + Hash> {
    root: Node<T>,
}

impl<T: Clone + Copy + Eq + Hash> Default for Trie<T> {
    fn default() -> Self {
        Self {
            root: Default::default(),
        }
    }
}

impl<T: Copy + Clone + Eq + Hash> Trie<T> {
    pub fn is_end(&self) -> bool {
        self.root.is_end
    }
    pub fn insert(&mut self, items: &[T]) {
        let mut curr = &mut self.root;
        for i in items.iter() {
            curr = curr.children.entry(*i).or_default();
            curr.is_end = false;
        }
        curr.is_end = true;
    }

    pub fn child_exits(&mut self, needle: &T) -> bool {
        self.root.children.contains_key(needle)
    }

    pub fn change_root(&mut self, new_root_value: T) -> bool {
        if let Some(new_root) = self.root.children.get(&new_root_value) {
            self.root = new_root.clone();
            true
        } else {
            false
        }
    }

    fn search(&self, items: &[T]) -> bool {
        let mut current = &self.root;
        for i in items.iter() {
            match current.children.get(i) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_end
    }

    pub fn root(&self) -> &Node<T> {
        &self.root
    }
}

impl<T: Copy + Clone + Hash + Eq + Display> Trie<T> {
    pub fn print(&self) {
        self.root.pprint("", 0);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use chess::*;

    #[test]
    fn db_test() {
        let mut db = Trie::default();
        let chess_line_1 = vec![
            ChessMove::from_str("d2d4").unwrap(),
            ChessMove::from_str("g8f6").unwrap(),
            ChessMove::from_str("c2c4").unwrap(),
            ChessMove::from_str("e7e6").unwrap(),
        ];
        let chess_line_2 = vec![
            ChessMove::from_str("d2d4").unwrap(),
            ChessMove::from_str("d7d5").unwrap(),
            ChessMove::from_str("c2c4").unwrap(),
            ChessMove::from_str("e7e6").unwrap(),
        ];

        let chess_line_3 = vec![
            ChessMove::from_str("d2d4").unwrap(),
            ChessMove::from_str("d7d5").unwrap(),
            ChessMove::from_str("c2c4").unwrap(),
            ChessMove::from_str("b8c6").unwrap(),
        ];

        db.insert(&chess_line_1);
        db.insert(&chess_line_2);
        db.insert(&chess_line_3);

        assert!(db.change_root(ChessMove::from_str("d2d4").unwrap()));
        assert!(db.change_root(ChessMove::from_str("d7d5").unwrap()));
        assert!(db.change_root(ChessMove::from_str("c2c4").unwrap()));
    }
}

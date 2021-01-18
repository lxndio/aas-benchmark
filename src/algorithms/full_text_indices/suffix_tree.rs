use std::cell::RefCell;
use std::rc::Rc;

pub struct SuffixTree {
    nodes: Vec<SuffixTreeNode>,
    text: Vec<u8>,
}

pub struct SuffixTreeNode {
    id: usize,
    parent: Option<usize>,
    childs: Vec<usize>,
    interval: (usize, Option<usize>),
}

struct ActivePosition {
    pub node: usize,
    pub letter: u8,
    pub depth: usize,
}

impl SuffixTree {
    pub fn new(text: &[u8]) -> Self {
        Self {
            nodes: Vec::new(),
            text: text.to_vec(),
        }
    }

    pub fn new_node(&mut self, parent: Option<usize>) -> usize {
        let id = self.nodes.len();

        self.nodes.push(SuffixTreeNode::new(id).parent(parent));

        // If there is a parent, add the new node as its child
        if let Some(parent_id) = parent {
            self.nodes
                .get_mut(parent_id)
                .expect("Trying to add a child to a node which doesn't exist")
                .childs
                .push(id);
        }

        id
    }

    pub fn node(&self, id: usize) -> &SuffixTreeNode {
        self.nodes
            .get(id)
            .expect("Trying to get node with ID that doesn't exist")
    }

    pub fn node_mut(&mut self, id: usize) -> &mut SuffixTreeNode {
        self.nodes
            .get_mut(id)
            .expect("Trying to get node with ID that doesn't exist")
    }

    pub fn child_at(&self, node_id: usize, letter: u8) -> Option<usize> {
        self.node(node_id)
            .childs
            .iter()
            .filter(|x| self.text[self.node(**x).interval.0] == letter)
            .copied()
            .next()
    }

    pub fn add_child(&mut self, node_id: usize, child_id: usize) {
        self.node_mut(node_id).childs.push(child_id);
    }

    pub fn remove_child(&mut self, node_id: usize, child_id: usize) {
        self.node_mut(node_id).childs.retain(|x| *x != child_id);
    }

    pub fn ukkonen(&mut self) {
        let root = self.new_node(None);
        let mut active = ActivePosition::new();
        let mut l: isize = -1;

        // At the beginning, active position is the root
        active.node = root;

        for phase in 0..self.text.len() {
            // Check if current character can already be read from current position
            let readable;
            if active.is_node() {
                readable = self.child_at(active.node, self.text[phase]).is_some();
            } else {
                let child = self.node(self.child_at(active.node, active.letter).unwrap());
                readable = self.text[child.interval.0 + active.depth + 1] == self.text[phase];
            }

            if readable {
                // Move active position by read character
                if active.is_node() {
                    active.letter = self.text[phase];
                    active.depth = 1;
                } else {
                    active.depth += 1;
                }

                // TODO Check if new position is a node
                let child = self.child_at(active.node, self.text[phase]).unwrap();

                if self.node(child).interval.1.is_some()
                    && self.node(child).interval.0 + active.depth
                        >= self.node(child).interval.1.unwrap()
                {
                    active.node = child;
                    active.depth = 0;
                }

                continue;
            } else {
                l += 1;

                if active.is_node() {
                    let new_leaf = self.new_node(Some(active.node));
                    self.node_mut(new_leaf).interval = (phase, None);
                } else {
                    let child = self.child_at(active.node, active.letter).unwrap();
                    let (a, b) = self.node(child).interval;
                    let new_node = self.new_node(Some(active.node));

                    self.remove_child(active.node, child);
                    self.add_child(active.node, new_node);
                    self.node_mut(new_node).interval = (a, Some(a + active.depth - 1));
                    self.node_mut(child).interval = (a + active.depth, b);
                }
            }
        }
    }
}

impl SuffixTreeNode {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            parent: None,
            childs: Vec::new(),
            interval: (0, None),
        }
    }

    pub fn parent(mut self, parent: Option<usize>) -> Self {
        self.parent = parent;

        self
    }
}

impl ActivePosition {
    pub fn new() -> Self {
        Self {
            node: 0,
            letter: 0,
            depth: 0,
        }
    }

    pub fn is_node(&self) -> bool {
        self.depth == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

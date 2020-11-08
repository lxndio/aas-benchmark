use std::collections::{HashMap, VecDeque};

pub struct ACNode<'a> {
    targets: HashMap<u8, ACNode<'a>>,
    lps: Option<&'a ACNode<'a>>,
    parent: Option<&'a ACNode<'a>>,
    letter: u8,
    label: Vec<u8>,
    out: Vec<u8>,
}

impl<'a> ACNode<'a> {
    pub fn new(parent: Option<&'a ACNode>, letter: u8, depth: usize, label: Vec<u8>) -> Self {
        let mut depth = depth;
        let mut label = label;

        if parent.is_some() {
            depth += 1;
            label.push(letter);
        }

        Self {
            targets: HashMap::new(),
            lps: None,
            parent,
            letter,
            label,
            out: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self::new(None, 0, 0, Vec::new())
    }

    /// A DFA delta function for transition upon processing a given character.
    ///
    /// It takes a character `c` and returns the new active `ACNode`
    /// after transitioning.
    pub fn delta(&self, c: u8) -> &ACNode {
        let mut q = self;

        while q.lps.is_some() && !q.targets.contains_key(&c) {
            q = q.lps.unwrap();
        }

        if q.targets.contains_key(&c) {
            q = &q.targets[&c];
        }

        q
    }

    /// Returns a vector of each node below and including self in BFS order.
    pub fn bfs(&self) -> Vec<&ACNode> {
        let mut queue: VecDeque<&ACNode> = VecDeque::new();
        queue.push_back(self);

        let mut ordered: Vec<&ACNode> = Vec::new();

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            ordered.push(node);
            queue.extend(node.targets.values());
        }

        ordered
    }
}

pub fn build_ac(patterns: Vec<&[u8]>) -> ACNode {
    unimplemented!();

    /*let mut root: ACNode = ACNode::empty();
    let mut node: ACNode;
    let mut newnode: ACNode;

    for (i, pattern) in patterns.iter().enumerate() {
        node = root;

        for c in pattern.iter() {
            if node.targets.contains_key(c) {
                node = *node.targets.get(c).unwrap();
            } else {
                newnode = ACNode::new(Some(node), *c, 0, Vec::new());
                node.targets.insert(*c, newnode);
            }
        }
    }

    root*/
}

pub fn aho_corasick(patterns: Vec<&[u8]>, text: &[u8]) -> Vec<Vec<usize>> {
    unimplemented!();
}

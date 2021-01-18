use std::collections::VecDeque;

pub struct ACTrie {
    nodes: Vec<ACNode>,
}

pub struct ACNode {
    id: usize,

    targets: Vec<usize>,
    lps: Option<usize>,
    parent: Option<usize>,
    letter: Option<u8>,
    label: Vec<u8>,
    depth: usize,
    out: Vec<usize>,
}

impl ACTrie {
    pub fn new(patterns: &[&[u8]]) -> Self {
        let mut ac_trie = Self { nodes: Vec::new() };
        let root = ac_trie.new_node(None, None, 0, &[]);
        let mut node;

        for (i, pattern) in patterns.iter().enumerate() {
            node = root;

            for c in *pattern {
                if let Some(target) = ac_trie.target_with(node, *c) {
                    node = target;
                } else {
                    node = ac_trie.new_node(Some(node), Some(*c), 0, &[]);
                }
            }

            ac_trie.node_mut(node).out.push(i);
        }

        // Build lps by iterating through trie in BFS-order
        for node in ac_trie.bfs(root) {
            if ac_trie.node(node).parent.is_none() {
                continue;
            }

            ac_trie.node_mut(node).lps = Some(if ac_trie.node(node).depth > 1 {
                ac_trie.delta(
                    ac_trie
                        .node(ac_trie.node(node).parent.unwrap())
                        .lps
                        .unwrap(),
                    ac_trie.node(node).letter.unwrap(),
                )
            } else {
                root
            });

            let lps = ac_trie.node(node).lps.unwrap();
            let out = ac_trie.node(lps).out.clone();
            ac_trie.node_mut(node).out.extend(out.iter());
        }

        ac_trie
    }

    fn new_node(
        &mut self,
        parent: Option<usize>,
        letter: Option<u8>,
        depth: usize,
        label: &[u8],
    ) -> usize {
        let id = self.nodes.len();
        let mut node = ACNode::new(id).parent(parent).letter(letter);

        if let Some(parent) = parent {
            node.depth = self.node(parent).depth + 1;

            let mut label = self.node(parent).label.clone();
            label.push(letter.expect("No letter given though required"));
            node.label = label;
        } else {
            node.depth = depth;
            node.label = label.to_vec();
        }

        self.nodes.push(node);

        // If there is a parent, add the new node as its child
        if let Some(parent_id) = parent {
            self.nodes
                .get_mut(parent_id)
                .expect("Trying to add a child to a node which doesn't exist")
                .targets
                .push(id);
        }

        id
    }

    fn node(&self, id: usize) -> &ACNode {
        &self.nodes[id]
    }

    fn node_mut(&mut self, id: usize) -> &mut ACNode {
        &mut self.nodes[id]
    }

    fn root(&self) -> &ACNode {
        &self.nodes[0]
    }

    fn target_with(&self, node: usize, letter: u8) -> Option<usize> {
        self.node(node)
            .targets
            .iter()
            .map(|x| (self.node(*x).letter, *x))
            .filter(|(c, _)| *c == Some(letter))
            .map(|(_, x)| x)
            .next()
    }

    fn has_target_with(&self, node: usize, letter: u8) -> bool {
        self.target_with(node, letter).is_some()
    }

    fn delta(&self, node: usize, c: u8) -> usize {
        let mut q = self.node(node);

        while q.lps.is_some() && !self.has_target_with(q.id, c) {
            q = self.node(q.lps.unwrap());
        }

        if let Some(target) = self.target_with(q.id, c) {
            q = self.node(target);
        }

        q.id
    }

    fn bfs(&self, node: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let mut queue = VecDeque::from(vec![node]);
        let mut node;

        while !queue.is_empty() {
            node = queue.pop_front().unwrap();

            res.push(node);
            queue.extend(self.node(node).targets.iter());
        }

        res
    }

    fn ac_with_automaton(&self, patterns: &[&[u8]], text: &[u8]) -> Vec<Vec<usize>> {
        let mut res = vec![Vec::new(); patterns.len()];
        let mut q = self.root().id;

        for (i, c) in text.iter().enumerate() {
            q = self.delta(q, *c);

            for x in self.node(q).out.iter() {
                //res[*x].push(i - patterns[*x].len() + 1);
                res[*x].push(i);
            }
        }

        res
    }
}

impl ACNode {
    pub fn new(id: usize) -> Self {
        Self {
            id,

            targets: Vec::new(),
            lps: None,
            parent: None,
            letter: None,
            label: Vec::new(),
            depth: 0,
            out: Vec::new(),
        }
    }

    pub fn parent(mut self, parent: Option<usize>) -> Self {
        self.parent = parent;

        self
    }

    pub fn letter(mut self, letter: Option<u8>) -> Self {
        self.letter = letter;

        self
    }
}

pub fn aho_corasick(patterns: &[&[u8]], text: &[u8]) -> Vec<Vec<usize>> {
    let ac_trie = ACTrie::new(patterns);

    ac_trie.ac_with_automaton(patterns, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
        let text = b"gccttaacattattacgccta";
        let patterns: &[&[u8]] = &[b"tta", b"catta", b"gcct", b"abc"];

        let matches = aho_corasick(patterns, text);

        let matches_correct = vec![vec![3, 9, 12], vec![7], vec![0, 16], vec![]];

        assert_eq!(matches, matches_correct);
    }
}

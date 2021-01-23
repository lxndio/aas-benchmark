use std::collections::VecDeque;

/// A struct that manages all nodes in a Trie used by the Aho-Corasick algorithm.
struct ACTrie {
    nodes: Vec<ACNode>,
}

/// A node of a Trie used by the Aho-Corasick algorithm.
struct ACNode {
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
    /// Creates an empty Trie and builds it up according to the Aho-Corasick algorithm.
    fn new(patterns: &Vec<Vec<u8>>) -> Self {
        let mut ac_trie = Self { nodes: Vec::new() };
        let root = ac_trie.new_node(None, None, 0, &[]);
        let mut node;

        // Build the Trie for each pattern by iterating over the pattern's characters
        // and checking whether there are already "paths" in the Trie to take or not,
        // adding new nodes along the way if necessary
        for (i, pattern) in patterns.iter().enumerate() {
            node = root;

            for c in pattern.iter() {
                // Is there already a "path" to take given the already read characters?
                if let Some(target) = ac_trie.target_with(node, *c) {
                    node = target;
                } else {
                    node = ac_trie.new_node(Some(node), Some(*c), 0, &[]);
                }
            }

            ac_trie.node_mut(node).out.push(i);
        }

        // Next, the lps-links have to be built. This is done by iterating
        // through the previously built Trie in BFS-order
        for node in ac_trie.bfs(root) {
            // If there is no parent node, i. e. the current node is the root,
            // there can't be an lps-link to anywhere, so this iteration will
            // be skipped
            if ac_trie.node(node).parent.is_none() {
                continue;
            }

            // Find the lps-link by using a delta transition
            ac_trie.node_mut(node).lps = Some(if ac_trie.node(node).depth > 1 {
                ac_trie.delta(
                    ac_trie
                        .node(ac_trie.node(node).parent.unwrap())
                        .lps
                        .unwrap(),
                    ac_trie.node(node).letter.unwrap(),
                )
            } else {
                // For all nodes with depth 1, the lps-link is the root node
                root
            });

            // Extend the output function of the current node by the values
            // of the lps-link's output function
            let lps = ac_trie.node(node).lps.unwrap();
            let out = ac_trie.node(lps).out.clone();
            ac_trie.node_mut(node).out.extend(out.iter());
        }

        ac_trie
    }

    /// Creates a new node, setting its label and depth according to its parent.
    fn new_node(
        &mut self,
        parent: Option<usize>,
        letter: Option<u8>,
        depth: usize,
        label: &[u8],
    ) -> usize {
        let id = self.nodes.len();
        let mut node = ACNode::new(id, parent, letter);

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

    /// Returns a reference to a node in the Trie given its ID.
    fn node(&self, id: usize) -> &ACNode {
        &self.nodes[id]
    }

    /// Returns a mutable reference to a node in the Trie given its ID.
    fn node_mut(&mut self, id: usize) -> &mut ACNode {
        &mut self.nodes[id]
    }

    /// Returns a reference to the Trie's root node.
    fn root(&self) -> &ACNode {
        &self.nodes[0]
    }

    /// Returns a node from a given node's target list which is reachable from
    /// the given node by reading a given letter.
    ///
    /// Returns `None` if there is no such node.
    fn target_with(&self, node: usize, letter: u8) -> Option<usize> {
        self.node(node)
            .targets
            .iter()
            .map(|x| (self.node(*x).letter, *x))
            .filter(|(c, _)| *c == Some(letter))
            .map(|(_, x)| x)
            .next()
    }

    /// Returns whether there is a node in a given node's target list which is
    /// reachable from the given node by reading a given letter.
    fn has_target_with(&self, node: usize, letter: u8) -> bool {
        self.target_with(node, letter).is_some()
    }

    /// Simulates the Trie's delta function.
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

    /// Returns a vector containing the IDs of all nodes from the subtree at
    /// a given node, sorted in BFS-order.
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

    /// Runs the Aho-Corasick algorithm given a list of patterns and a text.
    ///
    /// The Trie must have already been built.
    fn ac_with_automaton(&self, patterns: &Vec<Vec<u8>>, text: &[u8]) -> Vec<Vec<usize>> {
        let mut res = vec![Vec::new(); patterns.len()];
        let mut q = self.root().id;

        for (i, c) in text.iter().enumerate() {
            q = self.delta(q, *c);

            for x in self.node(q).out.iter() {
                res[*x].push((i as isize - patterns[*x].len() as isize + 1) as usize);
            }
        }

        res
    }
}

impl ACNode {
    /// Creates a new node.
    fn new(id: usize, parent: Option<usize>, letter: Option<u8>) -> Self {
        Self {
            id,

            targets: Vec::new(),
            lps: None,
            parent,
            letter,
            label: Vec::new(),
            depth: 0,
            out: Vec::new(),
        }
    }
}

/// Returns occurrences of given patterns in a text.
///
/// Takes multiple patterns and a text, returning a vector containing
/// vectors with the positions of the found occurrences for each pattern.
///
/// It uses the Aho-Corasick algorithm to first build a Trie with lps-links and
/// then find the occurrences of the given patterns in the text.
pub fn aho_corasick(patterns: &Vec<Vec<u8>>, text: &[u8]) -> Vec<Vec<usize>> {
    let ac_trie = ACTrie::new(patterns);

    ac_trie.ac_with_automaton(patterns, text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick() {
        let text = b"gccttaacattattacgccta";
        let patterns: Vec<Vec<u8>> = vec![
            b"tta".to_vec(),
            b"catta".to_vec(),
            b"gcct".to_vec(),
            b"abc".to_vec(),
        ];

        let matches = aho_corasick(&patterns, text);

        let matches_correct = vec![vec![3, 9, 12], vec![7], vec![0, 16], vec![]];

        assert_eq!(matches, matches_correct);
    }
}

/*use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct SuffixTree {
    childs: Vec<(usize, isize, Rc<Self>)>,
}

impl SuffixTree {
    pub fn new() -> Self {
        Self {
            childs: Vec::new(),
        }
    }

    pub fn child(self, child: SuffixTree, edge_l: usize, edge_r: isize) -> Self {
        self.childs.push((edge_l, edge_r, Rc::new(child)));

        self
    }

    pub fn childs(&self) -> &Vec<(usize, isize, Rc<Self>)> {
        &self.childs
    }
}

pub fn ukkonen(text: &[u8]) {
    let root: SuffixTree = SuffixTree::new();

    let mut i: usize = 0;
    let mut l: isize = -1;
    let mut active_node: &SuffixTree = &root;
    let mut active_char: u8 = text[0];
    let mut active_depth: usize = 0;

    loop {
        // Is the active position a node?
        if active_depth == 0 {
            for (edge_l, edge_r, child) in active_node.childs().iter() {
                if text[*edge_l] == text[i] {
                    // If there is only one char on the edge, set the child
                    // node as the next active node, otherwise set a
                    // position on the selected edge as active
                    if *edge_r == (edge_l + 1) as isize {
                        active_node = child;
                    } else {
                        active_char = text[i];
                        active_depth = 1;
                    }

                    i += 1;
                    continue;
                }
            }
        } else {
            // TODO
        }

        l += 1;

        active_node = active_node.child(SuffixTree::new(), i, -1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}*/

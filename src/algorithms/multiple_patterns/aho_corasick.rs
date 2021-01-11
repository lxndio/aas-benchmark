/*use std::rc::Rc;

pub struct ACNode {
    childs: Vec<Rc<ACNode>>,
    lps: Option<Box<ACNode>>,
    parent: Option<Rc<ACNode>>,
    letter: Option<u8>,
    label: Vec<u8>,
    depth: usize,
}

impl ACNode {
    pub fn new(parent: Option<Rc<ACNode>>, letter: Option<u8>, mut depth: usize, mut label: Vec<u8>) -> Self {
        if parent.is_some() {
            depth = parent.unwrap().depth + 1;
            label.extend(parent.unwrap().label());
            label.push(letter.expect("Letter has to be set"));
        }

        Self {
            childs: Vec::new(),
            lps: None,
            parent,
            letter,
            label,
            depth,
        }
    }

    fn label(&self) -> Vec<u8> {
        self.label.clone()
    }
}
*/

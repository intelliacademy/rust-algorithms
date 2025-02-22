
// Simple inbalanced BST

pub type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T: Ord + Copy> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct BST<T: Ord + Copy> {
    root: Link<T>,
}

impl <T: Ord + Copy> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, value: T) {
        self.root = self.insert_node(self.root.take(), value);
    }

    fn insert_node(&mut self, node: Link<T>, value: T) -> Link<T> {
        match node {
            None => Some(Box::new(Node {
                value,
                left: None,
                right: None,
            })),
            Some(mut node) => {
                if value <= node.value {
                    node.left = self.insert_node(node.left.take(), value);
                } else {
                    node.right = self.insert_node(node.right.take(), value);
                }
                Some(node)
            }
        }
    }

    pub fn search(&self, value: T) -> bool {
        let mut node = &self.root;
        self.search_node(node, value)
    }

    fn search_node(&self, mut node: &Link<T>, value: T) -> bool {
        match node {
            None => false,
            Some(node) => {
                if value == node.value {
                    true
                } else if value < node.value {
                    self.search_node(&node.left, value)
                } else {
                    self.search_node(&node.right, value)
                }
            }
        }
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        bst.insert(6);

        assert_eq!(bst.search(5), true);
    }
}
type Link<T> = Option<Box<Node<T>>>;


pub struct List<T> {
    head: Link<T>,
}

impl <T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push<T>(self,elem: T) -> List<T> {
        let new_node = Box::new(Node {
            data: elem,
            next: self.head,
        });

        List { head: Some(new_node) }
    }

    pub fn pop<T>(self) -> (Option<T>, List<T>) {
        match self.head {
            None => (None, self),
            Some(node) => {
                (Some(node.data), List { head: node.next })
            }
        }
    }
}


#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}
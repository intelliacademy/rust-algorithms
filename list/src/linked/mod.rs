type Link<T> = Option<Box<Node<T>>>;


pub struct List<T> {
    head: Link<T>,
}



impl <T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self,elem: T){
        let new_node = Box::new(Node {
            data: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}


#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_linked_list_push_and_pop(){
        let arr = vec![1,2,3,4,5];
        let mut list = List::new();
        for i in arr.iter() {
            list.push(i);
        }
        assert_eq!(list.pop(), Some(&5));
    }

}
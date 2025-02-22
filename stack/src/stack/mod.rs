use std::collections::LinkedList;

pub struct Queue<T> {
    list: LinkedList<T>,
}

impl <T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            list: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.list.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.list.front()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_queue_push() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_queue_peek(){
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(*queue.peek().unwrap(), 1);
    }


    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue().unwrap(), 1);
        assert_eq!(queue.dequeue().unwrap(), 2);
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.len(), 0);
    }


    #[test]
    fn test_is_empty() {
        let mut queue = Queue::new();
        assert_eq!(queue.is_empty(), true);
        queue.enqueue(1);
        assert_eq!(queue.is_empty(), false);
    }
}
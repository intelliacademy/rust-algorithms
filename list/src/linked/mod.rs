type Link<T> = Option<Box<Node<T>>>;

pub struct List<T: std::cmp::PartialEq + Copy> {
    head: Link<T>,
}

pub struct IntoIter<T: std::cmp::PartialEq + Copy>(List<T>);
pub struct Iter<'a, T: std::cmp::PartialEq + Copy> {
    next: Option<&'a Node<T>>,
}
pub struct IterMut<'a, T: std::cmp::PartialEq + Copy> {
    next: Option<&'a mut Node<T>>,
}

impl<T: std::cmp::PartialEq + Copy> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            data: elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn remove(&mut self, elem: T) -> Option<T> {
        let mut current = self.head.as_mut();
        match current {
            None => None,
            Some(ref mut node) => {
                if node.data == elem {
                    self.pop()
                } else {
                    while let Some(ref mut n) = current {
                        if n.data == elem {
                            let next = n.next.take();
                            let data = n.data;
                            n.next = next;
                            return Some(data);
                        }
                        current = n.next.as_mut();
                    }
                    None
                }
            }
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: Option::Some(self.head.as_deref_mut().unwrap()),
        }
    }
}

impl<T: std::cmp::PartialEq + Copy> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T: std::cmp::PartialEq + Copy> Iterator for IterMut<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            node.data
        })
    }
}

impl<'a, T: std::cmp::PartialEq + Copy> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next
            .map(|node| {
                self.next = node.next.as_deref();
                &node.data
            })
            .copied()
    }
}

impl<T: std::cmp::PartialEq + Copy> Drop for List<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
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
    use std::ops::{Deref, DerefMut};

    #[test]
    fn test_linked_list_push_and_pop() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut list = List::new();
        for i in arr.iter() {
            list.push(i);
        }
        assert_eq!(list.pop(), Some(&5));
    }

    #[test]
    fn test_linked_list_remove_contains_element() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut list = List::new();
        for i in arr.iter() {
            list.push(i);
        }
        assert_eq!(list.remove(&3), Some(&3));
    }

    #[test]
    fn test_linked_list_remove_does_not_contain_element() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut list = List::new();
        for i in arr.iter() {
            list.push(i);
        }
        assert_eq!(list.remove(&6), None);
    }

    #[test]
    fn test_linked_list_peek() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut list = List::new();
        for i in arr.iter() {
            list.push(i);
        }
        let elem = list.peek().unwrap().deref();
        assert_eq!(elem, &5);
    }

    #[test]
    fn test_linked_list_peek_mut() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut list = List::new();
        for i in arr.iter() {
            list.push(i);
        }
        let elem = list.peek_mut().unwrap().deref_mut();
        *elem = &6;
        assert_eq!(list.peek().unwrap().deref(), &6);
    }

    #[test]
    fn test_linked_list_into_iter() {
        let arr = vec![1, 2, 3, 4, 5];
        let mut list = List::new();
        for i in arr.iter() {
            list.push(i);
        }
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(&5));
    }
}

// Simple inbalanced BST

use std::cmp::Ordering;

#[derive(Debug)]
pub struct SBST<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<SBST<T>>>,
    right: Option<Box<SBST<T>>>,
}

impl<T> Default for SBST<T>
where
    T: Ord,
{
    fn default() -> Self {
        SBST::new()
    }
}

impl<T> SBST<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        SBST {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value)
        } else {
            match &self.value {
                None => (),
                Some(key) => {
                    let target_node = if value < *key {
                        &mut self.left
                    } else {
                        &mut self.right
                    };
                    match target_node {
                        Some(node) => node.insert(value),
                        None => {
                            let mut node = SBST::default();
                            node.insert(value);
                            *target_node = Some(Box::new(node))
                        }
                    }
                }
            }
        }
    }

    pub fn search(&self, value: T) -> bool
    where
        T: Ord,
    {
        match &self.value {
            Some(key) => match key.cmp(&value) {
                Ordering::Equal => true,
                Ordering::Greater => match &self.left {
                    None => false,
                    Some(node) => node.search(value),
                },
                Ordering::Less => match &self.right {
                    None => false,
                    Some(node) => node.search(value),
                },
            },
            None => false,
        }
    }

    pub fn min(&self) -> Option<&T>
    where
        T: Ord,
    {
        match &self.left {
            None => self.value.as_ref(),
            Some(node) => node.min(),
        }
    }

    pub fn max(&self) -> Option<&T>
    where
        T: Ord,
    {
        match &self.right {
            None => self.value.as_ref(),
            Some(node) => node.max(),
        }
    }

    pub fn ceil(&self, value: &T) -> Option<&T>
    where
        T: Ord,
    {
        None
    }

    pub fn floor(&self, value: &T) -> Option<&T>
    where
        T: Ord,
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_bst() -> SBST<i32> {
        let mut sbst = SBST::default();
        let arr = [4, 6, 7, 1, 8, 2];
        for i in 0..arr.len() {
            sbst.insert(arr[i])
        }
        sbst
    }

    #[test]
    fn test_bst() {
        let mut sbst = new_bst();
        println!("{:?}", sbst)
    }

    #[test]
    fn test_search() {
        let mut sbst = new_bst();
        let has_five = sbst.search(5);
        assert_eq!(has_five, false)
    }

    #[test]
    fn test_min() {
        let mut sbst = new_bst();
        let minimum_value = *sbst.min().unwrap();
        assert_eq!(minimum_value, 1)
    }

    #[test]
    fn test_max() {
        let mut sbst = new_bst();
        let max_value = *sbst.max().unwrap();
        assert_eq!(max_value, 8)
    }
}

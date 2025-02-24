// Simple inbalanced BST

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
                        Some( node) => node.insert(value),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut sbst = SBST::default();
        let arr = [4,6,7,1,8,2];
        for i in 0..arr.len() {
            sbst.insert(arr[i])
        }
        println!("{:?}", sbst)
    }
}

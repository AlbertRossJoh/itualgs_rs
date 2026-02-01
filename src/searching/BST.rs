use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

struct Value<T, K>(T, K);

type Edge<T, K> = Option<Box<Node<T, K>>>;

struct Node<T, K>(Value<T, K>, Edge<T, K>, Edge<T, K>);

impl<T, K> Node<T, K>
where
    T: Ord,
{
    fn new(key: T, val: K) -> Self {
        Node(Value(key, val), None, None)
    }

    fn cmp_to_key(&self, key: &T) -> Ordering {
        key.cmp(&self.0 .0)
    }
}

#[derive(Default)]
pub struct BST2<T, K> {
    root: Edge<T, K>,
}

impl<'a, T, K> BST2<T, K>
where
    T: Ord,
{
    pub fn new() -> BST2<T, K> {
        BST2 { root: None }
    }

    fn _put(key: T, val: K, curr: &mut Edge<T, K>) {
        let mut current = curr;
        while let Some(ref mut node) = *current {
            match node.cmp_to_key(&key) {
                Less => current = &mut node.1,
                Greater => current = &mut node.2,
                Equal => {
                    node.0 .1 = val;
                    return;
                }
            }
        }
        current.replace(Box::new(Node::new(key, val)));
    }

    pub fn put(&mut self, key: T, val: K) {
        Self::_put(key, val, &mut self.root);
    }

    fn _get(key: &'a T, curr: &'a Edge<T, K>) -> Option<&'a K> {
        let mut current = curr;
        while let Some(ref node) = *current {
            match node.cmp_to_key(key) {
                Less => current = &node.1,
                Greater => current = &node.2,
                Equal => return Some(&node.0 .1),
            }
        }

        None
    }

    fn _get_mut(key: T, curr: &mut Edge<T, K>) -> Option<&mut K> {
        if let Some(ref mut node) = *curr {
            return match node.cmp_to_key(&key) {
                Less => Self::_get_mut(key, &mut node.1),
                Greater => Self::_get_mut(key, &mut node.2),
                Equal => Some(&mut node.0 .1),
            };
        }
        None
    }

    pub fn get_root(&'a self) -> Option<&'a K> {
        match &self.root {
            Some(n) => self.get(&n.0 .0),
            None => None,
        }
    }

    pub fn get(&'a self, key: &'a T) -> Option<&'a K> {
        Self::_get(key, &self.root)
    }

    pub fn get_mut(&mut self, key: T) -> Option<&mut K> {
        Self::_get_mut(key, &mut self.root)
    }

    pub fn remove(&mut self, key: &T) -> Option<K> {
        Self::_remove(key, &mut self.root).map(|res| res.0 .1)
    }

    fn _remove(key: &T, curr: &mut Edge<T, K>) -> Option<Box<Node<T, K>>> {
        if let Some(mut node) = curr.take() {
            return match node.cmp_to_key(key) {
                res @ (Less | Greater) => {
                    curr.replace(node);
                    let next = curr.as_mut().unwrap();
                    Self::_remove(
                        key,
                        if res == Less {
                            &mut next.1
                        } else {
                            &mut next.2
                        },
                    )
                }
                Equal => {
                    match (node.1.take(), node.2.take()) {
                        (Some(left), Some(right)) => {
                            let mut right_tree = Some(right);
                            let mut successor = Self::_take_min(&mut right_tree).unwrap();
                            successor.1 = Some(left);
                            successor.2 = right_tree;
                            Some(successor)
                        }
                        (Some(left), None) => Some(left),
                        (None, Some(right)) => Some(right),
                        (None, None) => None,
                    }
                    .map(|v| curr.replace(v));

                    Some(node)
                }
            };
        }
        None
    }

    fn _take_min(curr: &mut Edge<T, K>) -> Edge<T, K> {
        let mut current = curr;
        while current.as_ref()?.1.is_some() {
            current = &mut current.as_mut().unwrap().1;
        }
        current.take()
    }

    fn _take_max(curr: &mut Edge<T, K>) -> Edge<T, K> {
        let mut current = curr;
        while current.as_ref()?.2.is_some() {
            current = &mut current.as_mut().unwrap().2;
        }
        current.take()
    }

    pub fn take_min(&mut self) -> Option<K> {
        Self::_take_min(&mut self.root).map(|x| x.0 .1)
    }

    pub fn take_max(&mut self) -> Option<K> {
        Self::_take_max(&mut self.root).map(|x| x.0 .1)
    }
}
#[cfg(test)]
mod tests {
    use super::BST2;

    #[test]
    fn test_deletion() {
        let mut bst: BST2<i32, &str> = BST2::new();
        bst.put(24, "Ferris");
        bst.put(20, "John");
        bst.put(25, "Jane");
        assert_eq!(bst.get_root().unwrap(), &"Ferris");
        assert_eq!(bst.get(&20).unwrap(), &"John");
        assert_eq!(bst.get(&25).unwrap(), &"Jane");
        bst.remove(&25);
        assert!(bst.get(&25).is_none());
        assert_eq!(bst.get_root().unwrap(), &"Ferris");
        assert_eq!(bst.get(&20).unwrap(), &"John");
    }

    #[test]
    fn test_deletion_2() {
        let mut bst: BST2<i32, &str> = BST2::new();
        bst.put(23, "Ferris");
        bst.put(20, "John");
        bst.put(25, "Jane");
        bst.put(30, "Doe");
        bst.put(24, "wut");
        assert_eq!(bst.get_root().unwrap(), &"Ferris");
        assert_eq!(bst.get(&20).unwrap(), &"John");
        assert_eq!(bst.get(&25).unwrap(), &"Jane");
        bst.remove(&25);
        assert!(bst.get(&25).is_none());
        assert_eq!(bst.get_root().unwrap(), &"Ferris");
        assert_eq!(bst.get(&20).unwrap(), &"John");
        assert_eq!(bst.get(&30).unwrap(), &"Doe");
    }
    //
    #[test]
    fn test_get() {
        let mut bst: BST2<u8, &str> = BST2::new();
        bst.put(4, "val4");
        bst.put(10, "val10");
        bst.put(2, "val2");
        bst.put(3, "val3");
        bst.put(11, "val11");
        let val = bst.get(&4).unwrap();
        assert_eq!(val, &"val4");
        let val2 = bst.get(&11).unwrap();
        assert_eq!(val2, &"val11");
    }

    #[test]
    fn test_put() {
        let mut bst: BST2<u8, &str> = BST2::new();
        bst.put(4, "val4");
        bst.put(10, "val10");
        bst.put(2, "val2");
        bst.put(3, "val3");
        bst.put(11, "val11");

        let val = bst.get_root().unwrap();
        assert_eq!(val, &"val4");
    }

    #[test]
    fn test_delete_min() {
        let mut bst: BST2<u8, &str> = BST2::new();
        bst.put(4, "val4");
        bst.put(10, "val10");
        bst.put(2, "val2");
        bst.put(3, "val3");
        bst.put(11, "val11");

        assert_eq!(bst.get(&2).unwrap(), &"val2");
        bst.take_min();
        assert!(bst.get(&2).is_none());
    }

    #[test]
    fn test_delete_max() {
        let mut bst: BST2<u8, &str> = BST2::new();
        bst.put(4, "val4");
        bst.put(10, "val10");
        bst.put(2, "val2");
        bst.put(3, "val3");
        bst.put(11, "val11");

        assert_eq!(bst.get(&11).unwrap(), &"val11");
        bst.take_max();
        assert!(bst.get(&11).is_none());
        assert!(bst.get(&3).is_some());
    }

    #[test]
    fn test_null_root() {
        let mut bst: BST2<u8, &str> = BST2::new();
        assert!(bst.take_max().is_none());
    }
}

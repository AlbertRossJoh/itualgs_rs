/// A bag is an unorderd data strructure of items which can be iterated over.
///
/// it supports the operations `add`, `iterator`, `size`, `new_from_vec` and `clone`.
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::fundamentals::bag::Bag;
///
///
/// let list = vec!['f', 'e', 'r', 'r', 'i', 's'];
/// let bag = Bag::new_from_vec_clone(&list);
/// let to_list = bag.as_vec();
/// assert_eq!(&to_list, &list);///
/// ```
#[derive(Default, Clone)]
pub struct Bag<T> {
    elements: Vec<T>,
    size: usize,
}

impl<T> Bag<T> {
    /// creates a new bag
    pub fn new() -> Bag<T> {
        Bag {
            elements: Vec::new(),
            size: 0,
        }
    }

    /// creates a new bag from a vector, while the vector maintains ownership
    pub fn new_from_vec(v: &Vec<T>) -> Bag<&T> {
        let mut b = Bag {
            elements: Vec::new(),
            size: 0,
        };
        for item in v {
            b.elements.push(item);
        }
        b
    }

    /// Creates a new bag from a vector, cloning all the elements of the vector
    pub fn new_from_vec_clone(v: &Vec<T>) -> Bag<T>
    where
        T: Clone,
    {
        let mut b = Bag {
            elements: Vec::new(),
            size: 0,
        };
        for item in v {
            b.elements.push(item.clone());
        }
        b
    }

    /// adds element to the bag
    pub fn add(&mut self, item: T) {
        self.elements.push(item);
        self.size += 1;
    }

    /// Returns true if bag is empty
    pub fn is_empty(&mut self) -> bool {
        self.elements.is_empty()
    }

    /// Size of the bag
    pub fn size(&mut self) -> usize {
        self.size
    }

    /// Gets an iterator of the bag
    pub fn iterator(&self) -> std::slice::Iter<'_, T> {
        self.elements.iter()
    }

    /// clones bag into vector
    pub fn as_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.iterator().cloned().collect()
    }
}

#[cfg(test)]
mod tests {

    use super::Bag;

    #[test]
    fn test_bag() {
        let list = vec!['f', 'e', 'r', 'r', 'i', 's'];
        let bag = Bag::new_from_vec_clone(&list);

        let to_list = bag.as_vec();
        assert_eq!(&to_list, &list);
    }
}

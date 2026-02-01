/// The fundamentals binary search represents a generic iterative binary search
/// it has the index_of method which returns the index of a searched for key.
/// This has a time complexity of *O*(log(n)) and has the recurrence relation:
/// T(n) = T(n/2) + 1
///
/// Author: cave
///
/// # Examples
/// ```
/// use itualgs_rs::fundamentals::binary_search;
///
/// let list = vec![0,1,2,3,4,5,6,7,8,9];
/// let index_of_three = binary_search::index_of(&list, 3);
/// assert!(index_of_three.unwrap() == 3)
/// ```  
pub fn index_of<T>(array: &[T], key: T) -> Option<usize>
where
    T: PartialOrd,
{
    // Get the middle element, high, and low begin here
    let mut hi = array.len() - 1;
    let mut lo = 0;

    // Get the value

    // While we haven't iterated through every element, continue searching
    while hi >= lo {
        let middle = lo + (hi - lo) / 2;
        let current_value = array.get(middle);

        // our current guess was too high
        if Some(&key) < current_value {
            hi = middle - 1;
        }
        // our current guess was too low
        else if Some(&key) > current_value {
            lo = middle + 1;
        }
        // we found the correct value from the key
        else {
            return Some(middle);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::index_of;
    #[test]
    fn test_search_last_index() {
        let list = vec![0, 3, 5, 7, 9, 10, 23, 31, 32, 94];
        let var = index_of(&list, 94);
        assert!(var.unwrap() == list.len() - 1);
    }

    #[test]
    fn test_search_first_index() {
        let list = vec![0, 3, 5, 7, 9, 10, 23, 31, 32, 94];
        let var = index_of(&list, 0);
        assert!(var.unwrap() == 0);
    }

    #[test]
    fn test_non_existent() {
        let list = vec![0, 3, 5, 7, 9, 10, 23, 31, 32, 94];
        let var = index_of(&list, 69);
        assert!(var.is_none() == true)
    }

    #[test]
    fn test_string_alphabetical() {
        let list = vec!["abc", "abcd", "abcde", "abcdef", "abcdefg", "abcdefgh"];
        let var = index_of(&list, "abcdefgh");
        assert!(var.unwrap() == 5)
    }
}


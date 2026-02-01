/// Selection sort is the simplest sorting algorithms. It simply gets the smallest element in the list and exchanges it with the first element.
/// This algorithm is not good, it will always run at *O(N^2)* time.
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::sorting::selection::sort;
///
/// let mut list = vec!['f','e','r','r','i','s'];
/// sort(&mut list);
///
/// assert_eq!(list, vec!['e','f','i','r','r','s']);
///
/// ```  

/// Sorts a given vector using selection sort
pub fn sort<T: Ord>(list: &mut Vec<T>) {
    for i in 0..list.len() {
        let mut min = i;
        for j in i + 1..list.len() {
            if less(&list[j], &list[min]) {
                min = j;
            }
        }
        exch(list, i, min);
    }
}

fn less<T: Ord>(v: &T, w: &T) -> bool {
    v.cmp(w).is_lt()
}

fn exch<T: Ord>(a: &mut Vec<T>, i: usize, j: usize) {
    if i != j {
        a.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn test_sort() {
        let mut list = vec![4, 2, 4, 1];

        sort(&mut list);

        assert_eq!(list, vec![1, 2, 4, 4]);
    }

    #[test]
    fn test_sort_char() {
        let mut list = vec!['f', 'e', 'r', 'r', 'i', 's'];

        sort(&mut list);

        assert_eq!(list, vec!['e', 'f', 'i', 'r', 'r', 's']);
    }

    #[test]
    fn test_sort_string() {
        let mut list = vec!["does", "the", "bed", "bugs", "bite", "?"];

        sort(&mut list);

        assert_eq!(list, vec!["?", "bed", "bite", "bugs", "does", "the"]);
    }
}

/// Insertion sort is one of the simplest sorting algorithms. It is sometimes also known as the brigde hand sorting algorithm.
/// This algorithm is particulary good at sorting partially sorted data, since everything behind index *i* is sorted.
/// This means that calling `sort` on a already sorted list, will result in *O(N)*. But the worst case is *1/2N^2=O(N^2)*.
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::sorting::insertion;
/// use itualgs_rs::sorting::insertion::index_sort;
/// use itualgs_rs::sorting::insertion::sort_slice;
/// use itualgs_rs::sorting::insertion::sort;
///
/// let mut list = vec!['f','e','r','r','i','s'];
/// sort(&mut list);
///
/// assert_eq!(list, vec!['e','f','i','r','r','s']);
///
/// let mut new_list = vec![9,8,7,6,5,4,3,2,1,0];
/// sort_slice(&mut new_list,3,8);
///
/// assert_eq!(new_list, vec![9,8,7,2,3,4,5,6,1,0]);
///
/// let another_list = vec![4,2,4,1];
/// let index = index_sort(&another_list);
///
/// assert_eq!(another_list, vec![4,2,4,1]);
/// assert_eq!(index, vec![1,2,4,4]);
/// ```  

/// Sorts a given vector by using insertion sort
pub fn sort<T: Ord>(list: &mut Vec<T>) {
    for i in 1..list.len() {
        let mut j = i;
        while j > 0 && less(&list[j], &list[j - 1]) {
            exch(list, j, j - 1);
            j = j - 1;
        }
    }
}

/// sorting a slice of a vector lo included hi excluded
pub fn sort_slice<T: Ord>(list: &mut Vec<T>, lo: i32, hi: i32) {
    let lo = lo as usize;
    let hi = hi as usize;
    for i in lo + 1..hi {
        let mut j = i;
        while j > lo && less(&list[j], &list[j - 1]) {
            exch(list, j, j - 1);
            j = j - 1;
        }
    }
}

/// Creates a copy of the list given and sorts it, the given list will remain intact
pub fn index_sort<T: Ord>(list: &Vec<T>) -> Vec<T>
where
    T: Copy,
{
    let mut index = Vec::with_capacity(list.len());
    for i in 0..list.len() {
        index.push(list[i]);
    }
    sort(&mut index);
    index
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
    use crate::sorting::insertion::{index_sort, sort_slice};

    use super::sort;

    #[test]
    fn test_sort() {
        let mut list = vec![4, 2, 4, 1];

        sort(&mut list);

        assert_eq!(list, vec![1, 2, 4, 4]);
    }

    #[test]
    fn test_sort_slice() {
        let mut list = vec![4, 2, 4, 1];

        sort_slice(&mut list, 0, 2);

        assert_eq!(list, vec![2, 4, 4, 1]);
    }

    #[test]
    fn test_sort_index() {
        let list = vec![4, 2, 4, 1];

        let index = index_sort(&list);

        assert_eq!(list, vec![4, 2, 4, 1]);
        assert_eq!(index, vec![1, 2, 4, 4]);
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

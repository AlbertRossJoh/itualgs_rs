/// Merge sort is a recusive algorithm that sorts an array by making sorting calls on smaller arrays.
/// After all sort call a number of merge calls are made to merge the sub arrays. The running time of this algorithm is *O(N log N)*.
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::sorting::merge::sort;
/// use itualgs_rs::sorting::merge::index_sort;
///
/// let mut list = vec!['f','e','r','r','i','s'];
/// sort(&mut list);
///
/// assert_eq!(list, vec!['e','f','i','r','r','s']);
///
/// let mut new_list = vec!["does","the","bed","bugs","bite","?"];
/// sort(&mut new_list);
///
/// assert_eq!(new_list, vec!["?","bed","bite","bugs","does","the"]);
///
/// let another_list = vec![4,2,4,1];
/// let index = index_sort(&another_list);
///
/// assert_eq!(another_list, vec![4,2,4,1]);
/// assert_eq!(index, vec![1,2,4,4]);
/// ```  
pub fn sort<T>(list: &mut Vec<T>)
where
    T: Copy,
    T: Ord,
{
    let mut aux: Vec<T> = list.clone();
    m_sort(list, &mut aux, 0, list.len() - 1);
}

/// Creates a copy of the list given and sorts it using merge sort, the given list will remain intact
pub fn index_sort<T>(list: &Vec<T>) -> Vec<T>
where
    T: Copy,
    T: Ord,
{
    let mut aux: Vec<T> = list.clone();
    let mut index = list.clone();
    m_sort(&mut index, &mut aux, 0, list.len() - 1);
    index
}

fn m_sort<T>(list: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, hi: usize)
where
    T: Copy,
    T: Ord,
{
    if hi <= lo {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    m_sort(list, aux, lo, mid);
    m_sort(list, aux, mid + 1, hi);
    merge(list, aux, lo, mid, hi);
}

fn merge<T>(list: &mut Vec<T>, aux: &mut Vec<T>, lo: usize, mid: usize, hi: usize)
where
    T: Copy,
    T: Ord,
{
    for i in lo..hi + 1 {
        aux[i] = list[i];
    }

    let mut i = lo;
    let mut j = mid + 1;
    for k in lo..hi + 1 {
        if i > mid {
            list[k] = aux[j];
            j += 1;
        } else if j > hi {
            list[k] = aux[i];
            i += 1;
        } else if less(&aux[j], &aux[i]) {
            list[k] = aux[j];
            j += 1;
        } else {
            list[k] = aux[i];
            i += 1;
        }
    }
}

fn less<T: Ord>(v: &T, w: &T) -> bool {
    v.cmp(w).is_lt()
}

#[cfg(test)]
mod tests {

    use super::index_sort;
    use super::sort;

    #[test]
    fn test_sort() {
        let mut list = vec![4, 2, 4, 1];

        sort(&mut list);

        assert_eq!(list, vec![1, 2, 4, 4]);
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

use std::cmp::Ordering;

pub struct IndexMinPQ<T> {
    max_n: usize,
    n: usize,
    pq: Vec<usize>,
    qp: Vec<Option<usize>>,
    keys: Vec<Option<T>>,
}

impl<T> IndexMinPQ<T>
where
    T: Ord,
{
    pub fn new(max_n: usize) -> IndexMinPQ<T> {
        let mut v = Vec::with_capacity(max_n + 1);
        for _ in 0..max_n + 1 {
            v.push(None);
        }

        IndexMinPQ {
            max_n,
            n: 0,
            pq: vec![0; max_n + 1],
            qp: vec![None; max_n + 1],
            keys: v,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn validate(&self, i: usize) {
        if i >= self.max_n {
            panic!("Index out of bounds!!")
        }
    }

    pub fn contains(&self, i: usize) -> bool {
        self.validate(i);
        self.qp[i].is_some()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn insert(&mut self, i: usize, key: T) {
        self.validate(i);
        self.n += 1;
        let tmp = self.n;
        self.qp[i].replace(tmp);
        self.pq[tmp] = i;
        self.keys[i].replace(key);
        self.swim(tmp);
    }

    pub fn min_index(&self) -> usize {
        self.pq[1]
    }

    pub fn min_key(&self) -> Option<&T> {
        if let Some(e) = &self.keys[1] {
            return Some(e);
        }
        None
    }

    pub fn delete_min(&mut self) -> usize {
        let min = self.min_index();
        let mut n = self.n;
        self.exch(1, n);
        n -= 1;
        self.sink(1);
        self.qp[min] = None;
        self.keys[min] = None;
        self.n = n;
        min
    }

    pub fn key_of(&self, i: usize) -> Option<&T> {
        self.validate(i);
        if let Some(e) = &self.keys[i] {
            return Some(e);
        }
        None
    }

    pub fn change_key(&mut self, i: usize, key: T) {
        self.validate(i);
        self.keys[i].replace(key);
        let elm = self.qp[i].unwrap();
        self.swim(elm);
        self.sink(elm);
    }

    pub fn decrease_key(&mut self, i: usize, key: T) {
        self.keys[i].replace(key);
        self.swim(self.qp[i].unwrap());
    }

    pub fn increase_key(&mut self, i: usize, key: T) {
        self.keys[i].replace(key);
        self.sink(self.qp[i].unwrap());
    }

    pub fn delete(&mut self, i: usize) {
        self.validate(i);
        let index = self.qp[i].unwrap();
        let tmp = self.n;
        self.exch(index, tmp);
        self.n -= 1;
        self.swim(index);
        self.sink(index);
        self.keys[i] = None;
        self.qp[i] = None;
    }

    fn greater(&self, i: usize, j: usize) -> bool {
        self.keys[self.pq[i]].cmp(&self.keys[self.pq[j]]) == Ordering::Greater
    }

    fn swim(&mut self, k: usize) {
        let mut tmp = k;
        while tmp > 1 && self.greater(tmp / 2, tmp) {
            self.exch(k, tmp / 2);
            tmp = tmp / 2;
        }
    }

    fn sink(&mut self, k: usize) {
        let mut tmp = k;
        while 2 * tmp <= self.n {
            let mut j = 2 * tmp;
            if j < self.n && self.greater(j, j + 1) {
                j += 1;
            }
            if !self.greater(tmp, j) {
                break;
            }
            self.exch(tmp, j);
            tmp = j;
        }
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
        self.qp.get(self.pq[i]).replace(&Some(i));
        self.qp.get(self.pq[j]).replace(&Some(j));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph() {
        let mut pq = IndexMinPQ::<&str>::new(1000);
        pq.insert(600, "hej");
        pq.insert(900, "hej");
        pq.insert(7, "hej");
    }
}

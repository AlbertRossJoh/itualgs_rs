use crate::fundamentals::bag::Bag;

#[derive(Clone)]
pub struct Graph {
    v: usize,
    e: usize,
    adj: Vec<Bag<usize>>,
}

impl Graph {
    /// Inits a new graph with V vertices
    pub fn new(v: usize) -> Graph {
        let mut g = Graph {
            v,
            e: 0,
            adj: Vec::new(),
        };
        for _ in 0..v {
            g.adj.push(Bag::<usize>::new());
        }
        g
    }

    /// Gets the degree of a given vertex
    pub fn degree(&mut self, v: usize) -> usize {
        self.validate(v);
        self.adj[v].size()
    }

    /// returns the vertices adjacent to v
    pub fn adj_vertices(&self, v: usize) -> std::slice::Iter<'_, usize> {
        self.validate(v);
        self.adj[v].iterator()
    }

    /// adds an edge between v and w
    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate(v);
        self.validate(w);
        self.e += 1;
        self.adj[v].add(w);
        self.adj[w].add(v);
    }

    fn validate(&self, p: usize) {
        if p >= self.v {
            panic!("The index is out of bounds!")
        }
    }

    pub fn get_v(&self) -> usize {
        self.v
    }

    pub fn get_e(&self) -> usize {
        self.e
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_create_graph() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        let mut it = g.adj_vertices(0);
        assert_eq!(*it.next().unwrap(), 1);
        assert_eq!(*it.next().unwrap(), 2);
    }

    #[test]
    fn test_create_empty_graph() {
        let g = Graph::new(4);
        let mut it = g.adj_vertices(0);
        assert_eq!(it.next(), None);
    }
}


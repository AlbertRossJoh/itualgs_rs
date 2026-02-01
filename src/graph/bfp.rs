use crate::fundamentals::{queue::Queue, stack::Stack};

use super::di_graph::Graph;

/// Breath first paths implmented for at graph. Breath first paths is a graph searching algorithm that finds which vertices are connected to the vertex v.
/// The `BFP::new()` has a running time of *O(V+E)* where *V* is the amount of vertices and *E* the amount of edges.
///
/// BFP finds a shortest path tree from a vertex V, in an undirected unweighted graph. Because is holds more data, it has a larger space complexity.
///
/// BFP has the functions `get_path_to` which runs linearly in the amount of edges between to vertices V W.
/// It also has the functions `get_dist_to` and `has_path_to` which are constant.
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::graph::bfp::BFP;
/// use itualgs_rs::graph::di_graph::Graph;
///
/// let mut g = Graph::new(4);
/// g.add_edge(0, 1);
/// g.add_edge(0, 2);
/// let paths = BFP::new(&mut g, 2);
/// let mut s = paths.get_path_to(1).unwrap();
/// assert_eq!(s.pop().unwrap(), 2 as usize);
/// assert_eq!(s.pop().unwrap(), 0 as usize);
/// assert_eq!(s.pop().unwrap(), 1 as usize);
/// assert_eq!(paths.has_path_to(1), true);
/// ```
pub struct BFP {
    pub marked: Vec<bool>,
    pub edge_to: Vec<usize>,
    pub dist_to: Vec<usize>,
}

impl BFP {
    /// creates a new marked list from a graph
    pub fn new(g: &mut Graph, s: usize) -> BFP {
        let tmp = Self::bfs(g, s);
        BFP {
            marked: tmp.0,
            edge_to: tmp.1,
            dist_to: tmp.2,
        }
    }

    fn infinity() -> usize {
        usize::MAX
    }

    fn validate(&self, p: usize) {
        if p >= self.marked.len() {
            panic!("Index out of bounds")
        }
    }

    /// Does the source node have a path to v
    pub fn has_path_to(&self, v: usize) -> bool {
        self.validate(v);
        self.marked[v]
    }

    /// Get dist from the source vertex to v
    pub fn get_dist_to(&self, v: usize) -> usize {
        self.validate(v);
        self.dist_to[v]
    }

    pub fn get_path_to(&self, v: usize) -> Option<Stack<usize>> {
        self.validate(v);
        if !self.has_path_to(v) {
            return None;
        }
        let mut x = v;
        let mut stack = Stack::<usize>::new();
        loop {
            if self.dist_to[x] == 0 {
                break;
            }
            stack.push(x);
            x = self.edge_to[x];
        }
        stack.push(x);

        Some(stack)
    }

    /// Runs bfs on the given graph
    fn bfs(g: &mut Graph, s: usize) -> (Vec<bool>, Vec<usize>, Vec<usize>) {
        let mut m: Vec<bool> = vec![false; g.get_v()];
        let mut edge_to: Vec<usize> = vec![0; g.get_v()];
        let mut dist_to: Vec<usize> = vec![Self::infinity(); g.get_v()];
        // let mut adj: Vec<Iter<usize>> = Vec::with_capacity(g.get_v());

        let mut q = Queue::<usize>::new();

        m[s] = true;
        dist_to[s] = 0;
        q.enqueue(s);
        while !q.is_empty() {
            let v = q.dequeue().unwrap();
            for vertex in g.adj_vertices(v) {
                if !m[*vertex] {
                    edge_to[*vertex] = v;
                    dist_to[*vertex] = dist_to[v] + 1;
                    m[*vertex] = true;
                    q.enqueue(*vertex);
                }
            }
        }
        (m, edge_to, dist_to)
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::di_graph::Graph;

    use super::BFP;

    #[test]
    fn test_create_graph() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        let paths = BFP::new(&mut g, 2);
        let mut s = paths.get_path_to(1).unwrap();
        assert_eq!(s.pop().unwrap(), 2 as usize);
        assert_eq!(s.pop().unwrap(), 0 as usize);
        assert_eq!(s.pop().unwrap(), 1 as usize);
        assert_eq!(paths.has_path_to(1), true);
    }
}

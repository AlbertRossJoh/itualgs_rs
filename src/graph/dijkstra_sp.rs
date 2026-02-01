use crate::{fundamentals::stack::Stack, sorting::index_min_pq::IndexMinPQ};

use super::{edge::Edge, edge_weighted_graph::EdgeWeightedGraph};

/// Dijkstras algorithm is for findin the shortest path between two points.
/// This is done by creating a shortest path tree which means, that we actually gets the shortest path to any point in the edgeweighted graph.
///
/// The `new` method runs in time *O(E log V)* for a graph with edges *E* and vertices *V*.
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::graph::dijkstra_sp::DijkstraSP;
/// use itualgs_rs::graph::edge::Edge;
/// use itualgs_rs::graph::edge_weighted_graph::EdgeWeightedGraph;
///
///
/// let mut g = EdgeWeightedGraph::new(4);
/// let list = vec![
///     Edge::new(0, 1, 10),
///     Edge::new(2, 1, 2),
///     Edge::new(2, 0, 20)];
///
/// for ele in list {
///     g.add_edge(ele);
/// }
/// let mut dijkstra_SP = DijkstraSP::new(g, 0);
/// let mut dist_2 = dijkstra_SP.get_distance_to(2);
/// assert_eq!(dist_2, 12);
///
/// let mut path = dijkstra_SP.path_to(2).unwrap();
///
/// assert_eq!(path.pop().unwrap().weight, 10);
/// assert_eq!(path.pop().unwrap().weight, 2);
/// assert_eq!(path.is_empty(), true);
/// ```
pub struct DijkstraSP {
    dist_to: Vec<u128>,
    edge_to: Vec<Option<Edge>>,
    pq: IndexMinPQ<u128>,
}

impl DijkstraSP {
    pub fn new(g: EdgeWeightedGraph, s: usize) -> Self {
        let mut dist_to = vec![u128::MAX; g.v];
        let edge_to: Vec<Option<Edge>> = vec![None; g.v];

        dist_to[s] = 0;
        let pq = IndexMinPQ::<u128>::new(g.v);

        let mut tmp = DijkstraSP {
            dist_to,
            edge_to,
            pq,
        };
        tmp.pq.insert(s, tmp.dist_to[s]);

        while !tmp.pq.is_empty() {
            let v = tmp.pq.delete_min();
            for e in g.adj(v) {
                tmp.relax(e, v);
            }
        }
        tmp
    }

    /// is constant time
    pub fn get_distance_to(&self, v: usize) -> u128 {
        self.dist_to[v]
    }

    /// is constant time
    pub fn has_path_to(&self, v: usize) -> bool {
        self.dist_to[v] < u128::MAX
    }

    /// Is $O(N)$ in the amount of $N$ vertices to v
    pub fn path_to(&self, v: usize) -> Option<Stack<Edge>> {
        if !self.has_path_to(v) {
            return None;
        }
        let mut s = Stack::<Edge>::new();
        let mut x = v;
        while let Some(e) = &self.edge_to[x] {
            s.push(e.clone());
            x = e.other(x);
        }

        Some(s)
    }

    fn relax(&mut self, e: &Edge, v: usize) {
        let w = e.other(v);
        let tmp = &mut self.dist_to;
        if tmp[w] > tmp[v] + e.weight {
            tmp[w] = tmp[v] + e.weight;
            self.edge_to[w] = Some(e.clone());
            if self.pq.contains(w) {
                self.pq.decrease_key(w, self.dist_to[w]);
            } else {
                self.pq.insert(w, self.dist_to[w]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::edge::Edge;

    use super::EdgeWeightedGraph;

    use super::DijkstraSP;

    #[test]
    fn test_create_graph() {
        let mut g = EdgeWeightedGraph::new(4);
        let list = vec![Edge::new(0, 1, 10), Edge::new(2, 1, 2), Edge::new(2, 0, 20)];

        for ele in list {
            g.add_edge(ele);
        }
        let k = DijkstraSP::new(g, 0);
        let dist_2 = k.get_distance_to(2);
        assert_eq!(dist_2, 12);

        let mut path = k.path_to(2).unwrap();

        assert_eq!(path.pop().unwrap().weight, 10);
        assert_eq!(path.pop().unwrap().weight, 2);
        assert_eq!(path.is_empty(), true);
    }
}

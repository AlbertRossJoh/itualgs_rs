use std::rc::Rc;

use crate::fundamentals::{queue::Queue, uf::WeightedQuickUnionUF};

use super::{edge::Edge, edge_weighted_graph::EdgeWeightedGraph};

/// Kruskals algorithm is created to find a minimum spanning tree over a weighted undirected graph.
/// It does this by, until every edge have been touched, continuously getting the edge with the lowest weight in the graph and adding it to the MST.
///
/// It has a running time of *O(E log E)* in the worst case, where *E* is the amount of edges
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::graph::kruskal_mst::KruskalMST;
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
///
/// let mut k = KruskalMST::new(g);
/// let mut edges = k.edges();
/// let w1 = edges.dequeue().unwrap();
/// let w2 = edges.dequeue().unwrap();
/// assert_eq!(w1.weight, 2);
/// assert_eq!(w2.weight, 10);
/// ```
pub struct KruskalMST {
    pub weight: u128,
    mst: Queue<Rc<Edge>>,
}

impl KruskalMST {
    pub fn new(g: EdgeWeightedGraph) -> Self {
        let mut kruskal = KruskalMST {
            weight: 0,
            mst: Queue::new(),
        };
        let mut edges: Vec<&Rc<Edge>> = Vec::new();
        let tmp = g.edges();
        for e in tmp.iterator() {
            edges.push(e);
        }

        edges.sort_by(|a, b| a.cmp(b));

        let mut uf = WeightedQuickUnionUF::new(g.v);

        let mut i: usize = 0;
        loop {
            if i >= g.e || kruskal.mst.size() > g.v - 1 {
                break;
            }
            let edge = edges[i];
            let v = edge.either();
            let w = edge.other(v);

            if uf.find(v) != uf.find(w) {
                uf.union(v, w);
                kruskal.mst.enqueue(edge.clone());
                kruskal.weight += edge.weight;
            }
            i += 1;
        }
        kruskal
    }

    pub fn edges(&mut self) -> &mut Queue<Rc<Edge>> {
        &mut self.mst
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::edge::Edge;

    use super::EdgeWeightedGraph;

    use super::KruskalMST;

    #[test]
    fn test_create_graph() {
        let mut g = EdgeWeightedGraph::new(4);
        let list = vec![Edge::new(0, 1, 10), Edge::new(2, 1, 2), Edge::new(2, 0, 20)];

        for ele in list {
            g.add_edge(ele);
        }
        let mut k = KruskalMST::new(g);
        let edges = k.edges();
        let w1 = edges.dequeue().unwrap();
        let w2 = edges.dequeue().unwrap();
        assert_eq!(w1.weight, 2);
        assert_eq!(w2.weight, 10);
    }
}

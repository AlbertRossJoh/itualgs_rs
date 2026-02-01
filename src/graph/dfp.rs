use crate::fundamentals::stack::Stack;

use super::di_graph::Graph;
use std::slice::Iter;

/// Depth first paths implmented for at graph. Depth first paths is a graph searching algorithm that finds which vertices are connected to the vertex v.
/// The `DFP::new()` has a running time of *O(V+E)* where *V* is the amount of vertices and *E* the amount of edges.
///
/// DFP contains a vector of booleans to check weather the vertices are connected or not, as well a vector of paths from the given vertex.
/// For this reason the DFP uses more space tan DFS.
///
/// It does not support any operations, since it is just a data struct.
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::graph::dfp::DFP;
/// use itualgs_rs::graph::di_graph::Graph;
///
/// let mut g = Graph::new(4);
/// g.add_edge(0, 1);
/// g.add_edge(0, 2);
/// let paths = DFP::new(&mut g, 2);
/// assert_eq!(paths.marked[1], true);
/// assert_eq!(paths.edge_to[2], 0);
/// assert_eq!(paths.marked[2], true);
/// assert_eq!(paths.marked[3], false);
/// ```
pub struct DFP {
    pub marked: Vec<bool>,
    pub edge_to: Vec<usize>,
}

impl DFP {
    /// creates a new marked list from a graph
    pub fn new(g: &mut Graph, s: usize) -> DFP {
        let tmp = Self::dfs(g, s);
        DFP {
            marked: tmp.0,
            edge_to: tmp.1,
        }
    }

    fn dfs(g: &mut Graph, s: usize) -> (Vec<bool>, Vec<usize>) {
        let mut m: Vec<bool> = vec![false; g.get_v()];
        let mut edge_to: Vec<usize> = vec![0; g.get_v()];
        let mut adj: Vec<Iter<usize>> = Vec::with_capacity(g.get_v());

        for v in 0..g.get_v() {
            adj.push(g.adj_vertices(v))
        }

        let mut stack = Stack::<usize>::new();
        m[s] = true;
        stack.push(s);
        while !stack.is_empty() {
            let v = stack.peek().unwrap();
            if let Some(w) = adj[*v].next() {
                if !m[*w] {
                    m[*w] = true;
                    edge_to[*w] = *v;
                    stack.push(*w);
                }
            } else {
                stack.pop();
            }
        }
        (m, edge_to)
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::di_graph::Graph;

    use super::DFP;

    #[test]
    fn test_create_graph() {
        let mut g = Graph::new(4);
        g.add_edge(0, 1);
        g.add_edge(0, 2);
        let paths = DFP::new(&mut g, 2);
        assert_eq!(paths.marked[1], true);
        assert_eq!(paths.edge_to[2], 0);
        assert_eq!(paths.marked[2], true);
        assert_eq!(paths.marked[3], false);
    }
}

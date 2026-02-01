use std::cmp::Ordering;

/// Edge is a data struct for representing an edge in a graph.
/// It has two methods for getting from to nodes which is `either` and `other`. The `either` method returns v, and the `other` method returns the vertex opposite the one you give as an argument.
/// The weight is represented as a u128.
///
/// Author: AlberRossJoh
///
/// # Examples
/// ```
/// use itualgs_rs::graph::edge::Edge;
///
/// let mut edge = Edge::new(1, 2, 3);
/// let either = edge.either();
/// let other = edge.other(either);
/// assert_eq!(either, 1);
/// assert_eq!(other, 2);
/// ```
#[derive(Clone)]
pub struct Edge {
    v: usize,
    w: usize,
    pub weight: u128,
}

impl Edge {
    pub fn new(v: usize, w: usize, weight: u128) -> Self {
        Edge { v, w, weight }
    }

    pub fn either(&self) -> usize {
        self.v
    }

    pub fn other(&self, vertex: usize) -> usize {
        if vertex == self.v {
            self.w
        } else if vertex == self.w {
            self.v
        } else {
            panic!("Non legal \"other\"");
        }
    }

    pub fn cmp(&self, that: &Edge) -> Ordering {
        self.weight.cmp(&that.weight)
    }

    pub fn clone(&self) -> Edge {
        Edge {
            v: self.v.clone(),
            w: self.w.clone(),
            weight: self.weight.clone(),
        }
    }
}


use std::rc::Rc;
use std::slice::Iter;

use crate::fundamentals::bag::Bag;

use super::edge::Edge;

pub struct EdgeWeightedGraph {
    pub v: usize,
    pub e: usize,
    adj: Vec<Bag<Rc<Edge>>>,
}

impl EdgeWeightedGraph {
    pub fn new(v: usize) -> Self {
        let mut tmp: Vec<Bag<Rc<Edge>>> = Vec::with_capacity(v);
        for _ in 0..v {
            tmp.push(Bag::<Rc<Edge>>::new());
        }
        EdgeWeightedGraph { v, e: 0, adj: tmp }
    }

    pub fn add_edge(&mut self, e: Edge) {
        let v = e.either();
        let w = e.other(v);
        self.validate(v);
        self.validate(w);
        self.adj[v].add(Rc::new(e.clone()));
        self.adj[w].add(Rc::new(e));
        self.e += 1;
    }

    pub fn adj(&self, v: usize) -> Iter<'_, Rc<Edge>> {
        self.validate(v);
        self.adj[v].iterator()
    }

    pub fn degree(&mut self, v: usize) -> usize {
        self.validate(v);
        self.adj[v].size()
    }

    pub fn edges(&self) -> Bag<Rc<Edge>> {
        let mut list = Bag::<Rc<Edge>>::new();
        for v in 0..self.v {
            let mut self_loops = 0;
            for e in self.adj(v) {
                if e.other(v) > v {
                    list.add(Rc::clone(e));
                } else if e.other(v) == v {
                    if self_loops % 2 == 0 {
                        list.add(Rc::clone(e))
                    }
                    self_loops += 1;
                }
            }
        }
        list
    }

    fn validate(&self, v: usize) {
        if v >= self.v {
            panic!("Out of bounds!!")
        }
    }
}


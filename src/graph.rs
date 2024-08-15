use std::collections::HashMap;


pub struct Graph {
    pub nodes: HashMap<usize, Vec<(usize, usize)>>
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: HashMap::new() }
    }

    pub fn add_edge(
        &mut self,
        u: usize, v: usize,
        weight: usize
    ) {
        self.nodes.entry(u).or_insert_with(Vec::new).push((v, weight));
        self.nodes.entry(v).or_insert_with(Vec::new).push((u, weight));
    }
}
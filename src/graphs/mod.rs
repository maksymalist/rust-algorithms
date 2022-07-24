use std::collections::HashMap;

mod dfs;
use dfs::main as _dfs;

mod bfs;
use bfs::main as _bfs;


#[derive(Debug)]
pub struct Graph {
    pub graph: HashMap<char, Vec<char>>
}


impl Graph {
    pub fn new() -> Self {
        Self { graph: HashMap::new() }
    }

    pub fn add_node(&mut self, node: char, edges: Vec<char>){
        self.graph.insert(node, edges);
    }

    pub fn get(&self) -> &Graph {
        return self;
    }
}

pub fn bfs () {
    _bfs();
} 

pub fn dfs () {
    _dfs();
}
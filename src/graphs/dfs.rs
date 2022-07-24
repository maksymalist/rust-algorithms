use std::collections::HashSet;
use crate::graphs::Graph;

fn dfs(mut visited: HashSet<char>, graph: &Graph, node: char, target: char) -> bool {
    let tmp = graph;
    let g = &graph.get().graph;
    if node == target{
        return true
    }

    if visited.contains(&node) == false{
        visited.insert(node);
        let neighbors = g.get(&node);

        for neighbor in neighbors.unwrap(){
            if dfs(visited.clone(), tmp, *neighbor, target){
                return true;
            }
        }

    }

    return false
}

pub fn main(){
    let mut g = Graph::new();
    let visited: HashSet<char> = HashSet::new();

    g.add_node('A', vec!['B', 'C']);
    g.add_node('B', vec!['D', 'E', 'A']);
    g.add_node('C', vec!['F']);
    g.add_node('D', vec!['B', 'C']);
    g.add_node('E', vec!['B', 'C']);
    g.add_node('F', vec!['A']);

    const NODE: char = 'A';
    const TARGET: char = 'C';

    let status: bool = dfs(visited, &g, NODE, TARGET);
    println!("Target found: {}", status)
}
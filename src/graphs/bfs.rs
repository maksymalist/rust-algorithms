use std::collections::HashSet;
use crate::graphs::Graph;

fn bfs(mut visited: HashSet<char>, mut queue: Vec<char>, graph: &Graph, node: char, target: char) -> bool {
    let g = &graph.get().graph;

    visited.insert(node);
    queue.push(node);

    while queue.len() > 0 {

        let n: char = queue.pop().unwrap();

        if n == target{
            return true
        }

        let neighbors = g.get(&n);

        for neighbor in neighbors.unwrap() {
            if visited.contains(&neighbor) == false{
                visited.insert(*neighbor);
                queue.push(*neighbor);

                if neighbor == &target{
                    return true;
                }
            }
        }
    }


    return false;

}

pub fn main(){
    let mut g = Graph::new();
    let visited: HashSet<char> = HashSet::new();
    let queue: Vec<char> = Vec::new();

    g.add_node('A', vec!['B', 'C']);
    g.add_node('B', vec!['D', 'E', 'A']);
    g.add_node('C', vec!['F']);
    g.add_node('D', vec!['B', 'C']);
    g.add_node('E', vec!['B', 'C']);
    g.add_node('F', vec!['A']);

    const NODE: char = 'A';
    const TARGET: char = 'K';

    let status: bool = bfs(visited, queue, &g, NODE, TARGET);
    println!("Target found: {}", status)
}
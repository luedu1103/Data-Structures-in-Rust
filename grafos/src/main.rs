use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
struct DirectedGraph {
    nodes: Vec<String>,
    edges: HashMap<usize, Vec<usize>>,
}

impl DirectedGraph {
    fn new() -> Self {
        DirectedGraph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: &str) -> usize {
        self.nodes.push(node.to_string());
        let index = self.nodes.len() - 1;
        self.edges.insert(index, Vec::new());
        index
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(neighbors) = self.edges.get_mut(&from) {
            neighbors.push(to);
        }
    }

    // fn print_graph(&self) {
    //     for (index, node) in self.nodes.iter().enumerate() {
    //         println!("Node {}: {}", index, node);
    //         if let Some(neighbors) = self.edges.get(&index) {
    //             for neighbor in neighbors {
    //                 println!("  -> Node {}", neighbor);
    //             }
    //         }
    //     }
    // }

    fn dfs_util(&self, current: usize, visited: &mut Vec<bool>, result: &mut Vec<usize>) {
        visited[current] = true;
        println!("Node: {}", self.nodes[current]);
        result.push(current);

        if let Some(neighbors) = self.edges.get(&current) {
            for &neighbor in neighbors {
                if !visited[neighbor] {
                    self.dfs_util(neighbor, visited, result);
                }
            }
        }
    }

    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.nodes.len()];
        let mut result = Vec::new();
        self.dfs_util(start, &mut visited, &mut result);
        result
    }
}

fn main() {
    let mut graph = DirectedGraph::new();

    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("F");
    let g = graph.add_node("G");

    // FIRST GRAPH
    graph.add_edge(a, b);
    graph.add_edge(b, a);
    graph.add_edge(a, c);
    graph.add_edge(c, a);
    graph.add_edge(a, d);
    graph.add_edge(d, a);
    graph.add_edge(d, c);
    graph.add_edge(c, d);
    graph.add_edge(c, e);
    graph.add_edge(e, c);

    // graph.print_graph();

    // SECOND GRAPH
    // graph.add_edge(a, b);
    // graph.add_edge(a, c);
    // graph.add_edge(a, d);
    // graph.add_edge(c, e);
    // graph.add_edge(b, c);

    // THIRD GRAPH
    // graph.add_edge(a, b);
    // graph.add_edge(a, c);
    // graph.add_edge(a, e);
    // graph.add_edge(b, d);
    // graph.add_edge(b, f);
    // graph.add_edge(f, e);
    // graph.add_edge(c, g);

    // FORTH GRAPH
    // graph.add_edge(a, b);
    // graph.add_edge(a, c);
    // graph.add_edge(b, c);
    // graph.add_edge(c, a);
    // graph.add_edge(c, d);

    let start = Instant::now();
    graph.dfs(2);
    let duration = start.elapsed();

    println!("Time elapsed in sort_dfs() is: {:?}", duration);
}

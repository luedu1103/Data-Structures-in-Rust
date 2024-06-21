use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

struct Graph {
    nodes: Vec<String>,
    edges: HashMap<usize, Vec<(usize, usize)>>
}

impl Graph {
    // Create a new, empty graph
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    // Add a new node to the graph
    fn add_node(&mut self, node: &str) -> usize {
        self.nodes.push(node.to_string());
        let index = self.nodes.len() - 1;
        self.edges.insert(index, Vec::new());
        index // Return the index of the new node
    }

    // Add a new edge to the graph
    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        if let Some(neighbors) = self.edges.get_mut(&from) {
            neighbors.push((to, weight));
        }
    }

    fn dijkstra(&self, start: usize) -> Vec<(usize, usize)> {
        let mut distances = vec![usize::MAX; self.nodes.len()];
        let mut heap = BinaryHeap::new();
        
        distances[start] = 0;
        heap.push(State { node: start, cost: 0 });
        
        while let Some(State { node, cost }) = heap.pop() {
            if cost > distances[node] {
                continue;
            }
            
            if let Some(neighbors) = self.edges.get(&node) {
                for &(neighbor, weight) in neighbors {
                    let next = State { node: neighbor, cost: cost + weight };
                    
                    if next.cost < distances[neighbor] {
                        heap.push(next);
                        distances[neighbor] = next.cost;
                    }
                }
            }
        }
        
        distances.into_iter().enumerate().collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut graph = Graph::new();

    // Add nodes
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");

    // Add edges
    graph.add_edge(a, b, 1);
    graph.add_edge(a, c, 4);
    graph.add_edge(a, d, 7);
    graph.add_edge(b, c, 1);
    graph.add_edge(b, e, 3);
    graph.add_edge(c, d, 2);
    graph.add_edge(c, e, 5);
    graph.add_edge(d, e, 1);

    let list = graph.dijkstra(0);

    println!("{:?}", list);
}


#[test]
fn simple_graph() {
    let mut graph = Graph::new();

    // Add nodes
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");

    graph.add_edge(a, b, 1);
    graph.add_edge(a, c, 4);
    graph.add_edge(b, c, 2);
    graph.add_edge(b, d, 5);
    graph.add_edge(c, d, 1);

    let list = graph.dijkstra(0);

    println!("{:?}", list);
}

#[test]
fn large_graph(){
    let mut graph = Graph::new();

    // Add nodes
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");
    let d = graph.add_node("D");
    let e = graph.add_node("E");
    let f = graph.add_node("G");
    let g = graph.add_node("G");
    let h = graph.add_node("H");
    let i = graph.add_node("I");
    let j = graph.add_node("J");
    let k = graph.add_node("K");
    let l = graph.add_node("L");
    let m = graph.add_node("M");
    let n = graph.add_node("N");
    let p = graph.add_node("P");

    graph.add_edge(a, b, 8);
    graph.add_edge(a, d, 5);
    graph.add_edge(a, e, 4);

    graph.add_edge(b, c, 3);
    graph.add_edge(b, f, 4);
    graph.add_edge(b, e, 12);

    graph.add_edge(c, g, 11);
    graph.add_edge(c, f, 9);

    graph.add_edge(d, e, 9);
    graph.add_edge(d, h, 6);

    graph.add_edge(e, f, 3);
    graph.add_edge(e, j, 5);
    graph.add_edge(e, i, 8);

    graph.add_edge(f, g, 1);
    graph.add_edge(f, k, 8);

    graph.add_edge(g, l, 7);
    graph.add_edge(g, k, 8);

    graph.add_edge(h, i, 2);
    graph.add_edge(h, m, 7);

    graph.add_edge(i, j, 10);
    graph.add_edge(i, m, 6);
    
    graph.add_edge(j, k, 6);
    graph.add_edge(j, n, 9);

    graph.add_edge(k, l, 5);
    graph.add_edge(k, p, 7);
    
    graph.add_edge(l, p, 6);
    
    graph.add_edge(m, n, 2);

    graph.add_edge(n, p, 12);

    let list = graph.dijkstra(0);

    println!("{:?}", list);
}
use std::collections::HashMap;

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

    fn dijsktra(&mut self, start: usize) -> Vec<(usize, usize)>{
        let mut visited = vec![false; self.nodes.len()];
        let mut dijsktra_list = Vec::new();

        // Llenamos la lista de los nodos con distancia 1000
        for i in 0..self.nodes.len() {
            if i == 0 {
                dijsktra_list.push((i, 0));
            }
            else {
                dijsktra_list.push((i, 10000));
            }
        }

        self.dijsktra_util(&mut dijsktra_list, start, &mut visited);

        dijsktra_list
    }

    fn dijsktra_util(&self, dijsktra_list: &mut Vec<(usize, usize)>, start: usize, visited: &mut Vec<bool>) {
        visited[start] = true;
        if let Some(edges) = self.edges.get(&start){
            // println!("Edges: {:?}", edges);
            let mut min= (0, 1000);
            for node in edges {
                let current_node = dijsktra_list[node.0];

                // Obtenemos la tentativa menor
                if min.1 > node.1 {
                    min = *node;
                }

                // Cambiamos el minimo de la lista
                let suma = dijsktra_list[start].1 + node.1;
                if suma < current_node.1 {
                    dijsktra_list[node.0].1 = suma;
                }
            }
            println!("{:?}", dijsktra_list);

            if !visited[min.0] {
                self.dijsktra_util(dijsktra_list, min.0, visited);
            }
        }
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

    let list = graph.dijsktra(0);

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

    let list = graph.dijsktra(0);

    println!("{:?}", list);
}
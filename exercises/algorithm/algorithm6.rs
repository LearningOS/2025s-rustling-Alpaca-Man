/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/
use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        visit_order.push(v);
        visited.insert(v);
        let mut index = 0;
        while index < visit_order.len() {
            let current = visit_order[index];
            let adj = &self.adj[current];
            for j in 0..adj.len() {
                if !visited.contains(&adj[j]) {
                    visited.insert(adj[j]);
                    visit_order.push(adj[j]);
                }
            }
            index += 1;
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new(); 
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}


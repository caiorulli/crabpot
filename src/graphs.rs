#![allow(dead_code)]

use std::collections::VecDeque;

type Vertex = usize;
type AdjList = Vec<Vec<(Vertex, i32)>>;
type VisitedList = Vec<Vertex>;

trait Graph {
    fn bfs(&self, source: Vertex) -> VisitedList;
    fn dfs(&self, source: Vertex) -> VisitedList;
    fn bellman_ford(&self, source: Vertex) -> Vec<(Vertex, i32)>;
    fn djikstra(&self, source: Vertex) -> Vec<(Vertex, i32)>;
}

impl Graph for AdjList {
    fn bfs(&self, source: Vertex) -> VisitedList {
        let mut visited = vec![source];
        let mut deq = VecDeque::from([source]);

        while let Some(curr_v) = deq.pop_front() {
            for (v, _) in &self[curr_v] {
                if !visited.contains(v) {
                    visited.push(*v);
                    deq.push_back(*v)
                }
            }
        }

        visited
    }

    fn dfs(&self, source: Vertex) -> VisitedList {
        let mut visited = vec![];
        dfs_recur(self, source, &mut visited);
        visited
    }

    fn bellman_ford(&self, _source: Vertex) -> Vec<(Vertex, i32)> {
        vec![]
    }

    fn djikstra(&self, _source: Vertex) -> Vec<(Vertex, i32)> {
        vec![]
    }
}

fn dfs_recur(graph: &AdjList, curr_v: Vertex, visited: &mut VisitedList) {
    if !visited.contains(&curr_v) {
        visited.push(curr_v);
        for (v, _) in &graph[curr_v] {
            dfs_recur(graph, *v, visited)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs() {
        let graph = vec![
            vec![(1, 1), (2, 1)],
            vec![(2, 1)],
            vec![(0, 1), (3, 1)],
            vec![(3, 1)],
        ];

        assert_eq!(graph.bfs(2), vec![2, 0, 3, 1]);
    }

    #[test]
    fn dfs() {
        let graph = vec![
            vec![(1, 1), (2, 1)],
            vec![(2, 1)],
            vec![(0, 1), (3, 1)],
            vec![(3, 1)],
        ];

        assert_eq!(graph.dfs(2), vec![2, 0, 1, 3]);
    }
}

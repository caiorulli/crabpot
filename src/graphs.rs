#![allow(dead_code)]

use std::collections::VecDeque;

type AdjList = Vec<Vec<usize>>;
type VisitedList = Vec<usize>;

trait Graph {
    fn bfs(&self, initial_v: usize) -> VisitedList;
    fn dfs(&self, initial_v: usize) -> VisitedList;
}

impl Graph for AdjList {
    fn bfs(&self, initial_v: usize) -> VisitedList {
        let mut visited = vec![initial_v];
        let mut deq = VecDeque::from([initial_v]);

        while let Some(curr_v) = deq.pop_front() {
            for v in &self[curr_v] {
                if !visited.contains(v) {
                    visited.push(*v);
                    deq.push_back(*v)
                }
            }
        }

        visited
    }

    fn dfs(&self, initial_v: usize) -> VisitedList {
        let mut visited = vec![];
        dfs_recur(self, initial_v, &mut visited);
        visited
    }
}

fn dfs_recur(graph: &AdjList, curr_v: usize, visited: &mut VisitedList) {
    if !visited.contains(&curr_v) {
        visited.push(curr_v);
        for v in &graph[curr_v] {
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
            vec![1, 2],
            vec![2],
            vec![0, 3],
            vec![3]
        ];

        assert_eq!(graph.bfs(2), vec![2, 0, 3, 1]);
    }

    #[test]
    fn dfs() {
        let graph = vec![
            vec![1, 2],
            vec![2],
            vec![0, 3],
            vec![3]
        ];

        assert_eq!(graph.dfs(2), vec![2, 0, 1, 3]);
    }
}

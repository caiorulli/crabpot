use std::collections::VecDeque;

type Vertex = usize;
type AdjList = Vec<Vec<(Vertex, i32)>>;
type VisitedList = Vec<Vertex>;

trait Graph {
    fn bfs(&self, source: Vertex) -> VisitedList;
    fn dfs(&self, source: Vertex) -> VisitedList;
    fn bellman_ford(&self, source: Vertex) -> Vec<i32>;
    fn djikstra(&self, source: Vertex) -> Vec<i32>;
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

    fn bellman_ford(&self, source: Vertex) -> Vec<i32> {
        let len = self.len();
        let mut distances = vec![i32::MAX; len];
        distances[source] = 0;

        for _ in 0..len + 1 {
            for u in 0..len {
                for (v, weight) in self.get(u).unwrap() {
                    let dist_u = safe_add(*distances.get(u).unwrap(), *weight);
                    let dist_v = *distances.get(*v).unwrap();
                    if dist_u < dist_v {
                        distances[*v] = dist_u;
                    }
                }
            }
        }

        distances
    }

    fn djikstra(&self, _source: Vertex) -> Vec<i32> {
        vec![]
    }
}

// Not all that safe but does the trick for now
fn safe_add(a: i32, b: i32) -> i32 {
    if a == i32::MAX {
        return a;
    }
    a + b
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

    #[test]
    fn bellman_ford() {
        let graph = vec![
            vec![(1, 9), (2, 3)],
            vec![(2, 6), (4, 2)],
            vec![(1, 2), (3, 1)],
            vec![(2, 2), (4, 2)],
            vec![]
        ];

        assert_eq!(graph.bellman_ford(0), vec![0, 5, 3, 4, 6]);
    }
}

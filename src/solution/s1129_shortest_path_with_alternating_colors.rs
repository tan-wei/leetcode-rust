/**
 * [1129] Shortest Path with Alternating Colors
 *
 * You are given an integer n, the number of nodes in a directed graph where the nodes are labeled from 0 to n - 1. Each edge is red or blue in this graph, and there could be self-edges and parallel edges.
 * You are given two arrays redEdges and blueEdges where:
 *
 * 	redEdges[i] = [ai, bi] indicates that there is a directed red edge from node ai to node bi in the graph, and
 * 	blueEdges[j] = [uj, vj] indicates that there is a directed blue edge from node uj to node vj in the graph.
 *
 * Return an array answer of length n, where each answer[x] is the length of the shortest path from node 0 to node x such that the edge colors alternate along the path, or -1 if such a path does not exist.
 *  
 * Example 1:
 *
 * Input: n = 3, redEdges = [[0,1],[1,2]], blueEdges = []
 * Output: [0,1,-1]
 *
 * Example 2:
 *
 * Input: n = 3, redEdges = [[0,1]], blueEdges = [[2,1]]
 * Output: [0,1,-1]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 100
 * 	0 <= redEdges.length, blueEdges.length <= 400
 * 	redEdges[i].length == blueEdges[j].length == 2
 * 	0 <= ai, bi, uj, vj < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-path-with-alternating-colors/
// discuss: https://leetcode.com/problems/shortest-path-with-alternating-colors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/shortest-path-with-alternating-colors/solutions/3170129/rust-3ms-solution/
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut adj: Vec<Vec<(usize, usize)>> = vec![vec![]; n as usize];

        for it in red_edges {
            adj[it[0] as usize].push((it[1] as usize, 1));
        }

        for it in blue_edges {
            adj[it[0] as usize].push((it[1] as usize, 0));
        }

        let mut vis = vec![vec![false; 2]; n as usize]; // for both colors

        let mut dist = vec![-1; n as usize];
        dist[0] = 0;
        vis[0][0] = true;
        vis[0][1] = true;

        let mut q = std::collections::VecDeque::new();
        q.push_back((0, 0, 9)); // can insert anything usize, except 0 and 1, in place of 9

        while !q.is_empty() {
            let (node, temp_dis, is_red) = q.pop_front().unwrap();

            for it in &adj[node] {
                if vis[it.0][it.1] == false && it.1 != is_red {
                    if dist[it.0] == -1 {
                        dist[it.0] = temp_dis + 1;
                    }

                    vis[it.0][it.1] = true;
                    q.push_back((it.0, temp_dis + 1, it.1));
                }
            }
        }

        dist
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1129_example_1() {
        let n = 3;
        let red_edges = vec![vec![0, 1], vec![1, 2]];
        let blue_edges = vec![];
        let result = vec![0, 1, -1];

        assert_eq!(
            Solution::shortest_alternating_paths(n, red_edges, blue_edges),
            result
        );
    }

    #[test]
    fn test_1129_example_2() {
        let n = 3;
        let red_edges = vec![vec![0, 1]];
        let blue_edges = vec![vec![2, 1]];
        let result = vec![0, 1, -1];

        assert_eq!(
            Solution::shortest_alternating_paths(n, red_edges, blue_edges),
            result
        );
    }
}

/**
 * [1971] Find if Path Exists in Graph
 *
 * There is a bi-directional graph with n vertices, where each vertex is labeled from 0 to n - 1 (inclusive). The edges in the graph are represented as a 2D integer array edges, where each edges[i] = [ui, vi] denotes a bi-directional edge between vertex ui and vertex vi. Every vertex pair is connected by at most one edge, and no vertex has an edge to itself.
 * You want to determine if there is a valid path that exists from vertex source to vertex destination.
 * Given edges and the integers n, source, and destination, return true if there is a valid path from source to destination, or false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/14/validpath-ex1.png" style="width: 141px; height: 121px;" />
 * Input: n = 3, edges = [[0,1],[1,2],[2,0]], source = 0, destination = 2
 * Output: true
 * Explanation: There are two paths from vertex 0 to vertex 2:
 * - 0 &rarr; 1 &rarr; 2
 * - 0 &rarr; 2
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/14/validpath-ex2.png" style="width: 281px; height: 141px;" />
 * Input: n = 6, edges = [[0,1],[0,2],[3,5],[5,4],[4,3]], source = 0, destination = 5
 * Output: false
 * Explanation: There is no path from vertex 0 to vertex 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 2 * 10^5
 * 	0 <= edges.length <= 2 * 10^5
 * 	edges[i].length == 2
 * 	0 <= ui, vi <= n - 1
 * 	ui != vi
 * 	0 <= source, destination <= n - 1
 * 	There are no duplicate edges.
 * 	There are no self edges.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-if-path-exists-in-graph/
// discuss: https://leetcode.com/problems/find-if-path-exists-in-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let graph: Vec<std::collections::HashSet<usize>> = {
            let mut graph: Vec<std::collections::HashSet<usize>> =
                vec![std::collections::HashSet::new(); n as usize];
            for edge in edges.iter() {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                graph[u].insert(v);
                graph[v].insert(u);
            }
            graph
        };

        let mut seen: std::collections::HashSet<usize> =
            std::collections::HashSet::with_capacity(n as usize);

        seen.insert(source as usize);

        Self::dfs_helper(&mut seen, source as usize, destination as usize, &graph)
    }

    fn dfs_helper(
        seen: &mut std::collections::HashSet<usize>,
        cur: usize,
        destination: usize,
        graph: &Vec<std::collections::HashSet<usize>>,
    ) -> bool {
        if cur == destination {
            return true;
        }

        for &next in &graph[cur] {
            if seen.insert(next) {
                if Self::dfs_helper(seen, next, destination, graph) {
                    return true;
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1971_example_1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![1, 2], vec![2, 0]];
        let source = 0;
        let destination = 2;

        let result = true;

        assert_eq!(Solution::valid_path(n, edges, source, destination), result);
    }

    #[test]
    fn test_1971_example_2() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]];
        let source = 0;
        let destination = 5;

        let result = false;

        assert_eq!(Solution::valid_path(n, edges, source, destination), result);
    }
}

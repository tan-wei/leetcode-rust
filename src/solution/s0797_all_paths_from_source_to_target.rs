/**
 * [0797] All Paths From Source to Target
 *
 * Given a directed acyclic graph (DAG) of n nodes labeled from 0 to n - 1, find all possible paths from node 0 to node n - 1 and return them in any order.
 * The graph is given as follows: graph[i] is a list of all nodes you can visit from node i (i.e., there is a directed edge from node i to node graph[i][j]).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/28/all_1.jpg" style="width: 242px; height: 242px;" />
 * Input: graph = [[1,2],[3],[3],[]]
 * Output: [[0,1,3],[0,2,3]]
 * Explanation: There are two paths: 0 -> 1 -> 3 and 0 -> 2 -> 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/28/all_2.jpg" style="width: 423px; height: 301px;" />
 * Input: graph = [[4,3,1],[3,2,4],[3],[4],[]]
 * Output: [[0,4],[0,3,4],[0,1,3,4],[0,1,2,3,4],[0,1,4]]
 *
 *  
 * Constraints:
 *
 * 	n == graph.length
 * 	2 <= n <= 15
 * 	0 <= graph[i][j] < n
 * 	graph[i][j] != i (i.e., there will be no self-loops).
 * 	All the elements of graph[i] are unique.
 * 	The input graph is guaranteed to be a DAG.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/all-paths-from-source-to-target/
// discuss: https://leetcode.com/problems/all-paths-from-source-to-target/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::dfs_helper(&graph, 0, (graph.len() - 1) as i32, &mut vec![], vec![])
    }

    fn dfs_helper(
        graph: &[Vec<i32>],
        node: i32,
        n: i32,
        path: &mut Vec<i32>,
        mut paths: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        path.push(node);

        if node == n {
            paths.push(path.clone());
        } else {
            for child in &graph[node as usize] {
                paths = Self::dfs_helper(graph, *child, n, path, paths);
            }
        }

        path.pop();
        paths
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0797_example_1() {
        let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let result = vec![vec![0, 1, 3], vec![0, 2, 3]];

        assert_eq_sorted!(Solution::all_paths_source_target(graph), result);
    }

    #[test]
    fn test_0797_example_2() {
        let graph = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let result = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ];

        assert_eq_sorted!(Solution::all_paths_source_target(graph), result);
    }
}

/**
 * [1857] Largest Color Value in a Directed Graph
 *
 * There is a directed graph of n colored nodes and m edges. The nodes are numbered from 0 to n - 1.
 *
 * You are given a string colors where colors[i] is a lowercase English letter representing the color of the i^th node in this graph (0-indexed). You are also given a 2D array edges where edges[j] = [aj, bj] indicates that there is a directed edge from node aj to node bj.
 *
 * A valid path in the graph is a sequence of nodes x1 -> x2 -> x3 -> ... -> xk such that there is a directed edge from xi to xi+1 for every 1 <= i < k. The color value of the path is the number of nodes that are colored the most frequently occurring color along that path.
 *
 * Return the largest color value of any valid path in the given graph, or -1 if the graph contains a cycle.
 *
 *  
 * Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/21/leet1.png" style="width: 400px; height: 182px;" />
 *
 *
 * Input: colors = "abaca", edges = [[0,1],[0,2],[2,3],[3,4]]
 * Output: 3
 * Explanation: The path 0 -> 2 -> 3 -> 4 contains 3 nodes that are colored "a" (red in the above image).
 *
 *
 * Example 2:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/21/leet2.png" style="width: 85px; height: 85px;" />
 *
 *
 * Input: colors = "a", edges = [[0,0]]
 * Output: -1
 * Explanation: There is a cycle from 0 to 0.
 *
 *
 *  
 * Constraints:
 *
 *
 * 	n == colors.length
 * 	m == edges.length
 * 	1 <= n <= 10^5
 * 	0 <= m <= 10^5
 * 	colors consists of lowercase English letters.
 * 	0 <= aj, bj < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-color-value-in-a-directed-graph/
// discuss: https://leetcode.com/problems/largest-color-value-in-a-directed-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1857_example_1() {
        let colors = "abaca".to_string();
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]];

        let result = 3;

        assert_eq!(Solution::largest_path_value(colors, edges), result);
    }

    #[test]
    #[ignore]
    fn test_1857_example_2() {
        let colors = "a".to_string();
        let edges = vec![vec![0, 0]];

        let result = -1;

        assert_eq!(Solution::largest_path_value(colors, edges), result);
    }
}

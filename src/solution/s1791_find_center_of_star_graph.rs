/**
 * [1791] Find Center of Star Graph
 *
 * There is an undirected star graph consisting of n nodes labeled from 1 to n. A star graph is a graph where there is one center node and exactly n - 1 edges that connect the center node with every other node.
 * You are given a 2D integer array edges where each edges[i] = [ui, vi] indicates that there is an edge between the nodes ui and vi. Return the center of the given star graph.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/24/star_graph.png" style="width: 331px; height: 321px;" />
 * Input: edges = [[1,2],[2,3],[4,2]]
 * Output: 2
 * Explanation: As shown in the figure above, node 2 is connected to every other node, so 2 is the center.
 *
 * Example 2:
 *
 * Input: edges = [[1,2],[5,1],[1,3],[1,4]]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	3 <= n <= 10^5
 * 	edges.length == n - 1
 * 	edges[i].length == 2
 * 	1 <= ui, vi <= n
 * 	ui != vi
 * 	The given edges represent a valid star graph.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-center-of-star-graph/
// discuss: https://leetcode.com/problems/find-center-of-star-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if let Some(x) = edges[1].iter().position(|x| *x == edges[0][0]) {
            edges[1][x]
        } else {
            edges[0][1]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1791_example_1() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![4, 2]];

        let result = 2;

        assert_eq!(Solution::find_center(edges), result);
    }

    #[test]
    fn test_1791_example_2() {
        let edges = vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]];

        let result = 1;

        assert_eq!(Solution::find_center(edges), result);
    }
}

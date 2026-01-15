/**
 * [2316] Count Unreachable Pairs of Nodes in an Undirected Graph
 *
 * You are given an integer n. There is an undirected graph with n nodes, numbered from 0 to n - 1. You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting nodes ai and bi.
 * Return the number of pairs of different nodes that are unreachable from each other.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/05/tc-3.png" style="width: 267px; height: 169px;" />
 * Input: n = 3, edges = [[0,1],[0,2],[1,2]]
 * Output: 0
 * Explanation: There are no pairs of nodes that are unreachable from each other. Therefore, we return 0.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/05/tc-2.png" style="width: 295px; height: 269px;" />
 * Input: n = 7, edges = [[0,2],[0,5],[2,4],[1,6],[5,4]]
 * Output: 14
 * Explanation: There are 14 pairs of nodes that are unreachable from each other:
 * [[0,1],[0,3],[0,6],[1,2],[1,3],[1,4],[1,5],[2,3],[2,6],[3,4],[3,5],[3,6],[4,6],[5,6]].
 * Therefore, we return 14.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	0 <= edges.length <= 2 * 10^5
 * 	edges[i].length == 2
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	There are no repeated edges.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
// discuss: https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2316_example_1() {
        let n = 3;
        let edges = vec![vec![0, 1], vec![0, 2], vec![1, 2]];

        let result = 0;

        assert_eq!(Solution::count_pairs(n, edges), result);
    }

    #[test]
    #[ignore]
    fn test_2316_example_2() {
        let n = 7;
        let edges = vec![vec![0, 2], vec![0, 5], vec![2, 4], vec![1, 6], vec![5, 4]];

        let result = 14;

        assert_eq!(Solution::count_pairs(n, edges), result);
    }
}

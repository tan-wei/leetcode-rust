/**
 * [2368] Reachable Nodes With Restrictions
 *
 * There is an undirected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.
 * You are given a 2D integer array edges of length n - 1 where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree. You are also given an integer array restricted which represents restricted nodes.
 * Return the maximum number of nodes you can reach from node 0 without visiting a restricted node.
 * Note that node 0 will not be a restricted node.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/15/ex1drawio.png" style="width: 402px; height: 322px;" />
 * Input: n = 7, edges = [[0,1],[1,2],[3,1],[4,0],[0,5],[5,6]], restricted = [4,5]
 * Output: 4
 * Explanation: The diagram above shows the tree.
 * We have that [0,1,2,3] are the only nodes that can be reached from node 0 without visiting a restricted node.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/15/ex2drawio.png" style="width: 412px; height: 312px;" />
 * Input: n = 7, edges = [[0,1],[0,2],[0,5],[0,4],[3,2],[6,5]], restricted = [4,2,1]
 * Output: 3
 * Explanation: The diagram above shows the tree.
 * We have that [0,5,6] are the only nodes that can be reached from node 0 without visiting a restricted node.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 10^5
 * 	edges.length == n - 1
 * 	edges[i].length == 2
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	edges represents a valid tree.
 * 	1 <= restricted.length < n
 * 	1 <= restricted[i] < n
 * 	All the values of restricted are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reachable-nodes-with-restrictions/
// discuss: https://leetcode.com/problems/reachable-nodes-with-restrictions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2368_example_1() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![1, 2],
            vec![3, 1],
            vec![4, 0],
            vec![0, 5],
            vec![5, 6],
        ];
        let restricted = vec![4, 5];

        let result = 4;

        assert_eq!(Solution::reachable_nodes(n, edges, restricted), result);
    }

    #[test]
    #[ignore]
    fn test_2368_example_2() {
        let n = 7;
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 5],
            vec![0, 4],
            vec![3, 2],
            vec![6, 5],
        ];
        let restricted = vec![4, 2, 1];

        let result = 3;

        assert_eq!(Solution::reachable_nodes(n, edges, restricted), result);
    }
}

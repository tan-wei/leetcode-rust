/**
 * [2359] Find Closest Node to Given Two Nodes
 *
 * You are given a directed graph of n nodes numbered from 0 to n - 1, where each node has at most one outgoing edge.
 * The graph is represented with a given 0-indexed array edges of size n, indicating that there is a directed edge from node i to node edges[i]. If there is no outgoing edge from i, then edges[i] == -1.
 * You are also given two integers node1 and node2.
 * Return the index of the node that can be reached from both node1 and node2, such that the maximum between the distance from node1 to that node, and from node2 to that node is minimized. If there are multiple answers, return the node with the smallest index, and if no possible answer exists, return -1.
 * Note that edges may contain cycles.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-2.png" style="width: 321px; height: 161px;" />
 * Input: edges = [2,2,3,-1], node1 = 0, node2 = 1
 * Output: 2
 * Explanation: The distance from node 0 to node 2 is 1, and the distance from node 1 to node 2 is 1.
 * The maximum of those two distances is 1. It can be proven that we cannot get a node with a smaller maximum distance than 1, so we return node 2.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/07/graph4drawio-4.png" style="width: 195px; height: 161px;" />
 * Input: edges = [1,2,-1], node1 = 0, node2 = 2
 * Output: 2
 * Explanation: The distance from node 0 to node 2 is 2, and the distance from node 2 to itself is 0.
 * The maximum of those two distances is 2. It can be proven that we cannot get a node with a smaller maximum distance than 2, so we return node 2.
 *
 *  
 * Constraints:
 *
 * 	n == edges.length
 * 	2 <= n <= 10^5
 * 	-1 <= edges[i] < n
 * 	edges[i] != i
 * 	0 <= node1, node2 < n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-closest-node-to-given-two-nodes/
// discuss: https://leetcode.com/problems/find-closest-node-to-given-two-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2359_example_1() {
        let edges = vec![2, 2, 3, -1];
        let node1 = 0;
        let node2 = 1;

        let result = 2;

        assert_eq!(Solution::closest_meeting_node(edges, node1, node2), result);
    }

    #[test]
    #[ignore]
    fn test_2359_example_2() {
        let edges = vec![1, 2, -1];
        let node1 = 0;
        let node2 = 2;

        let result = 2;

        assert_eq!(Solution::closest_meeting_node(edges, node1, node2), result);
    }
}

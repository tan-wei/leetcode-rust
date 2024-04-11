/**
 * [1557] Minimum Number of Vertices to Reach All Nodes
 *
 * Given a directed acyclic graph, with n vertices numbered from 0 to n-1, and an array edges where edges[i] = [fromi, toi] represents a directed edge from node fromi to node toi.
 * Find the smallest set of vertices from which all nodes in the graph are reachable. It's guaranteed that a unique solution exists.
 * Notice that you can return the vertices in any order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/07/untitled22.png" style="width: 231px; height: 181px;" />
 *
 * Input: n = 6, edges = [[0,1],[0,2],[2,5],[3,4],[4,2]]
 * Output: [0,3]
 * Explanation: It's not possible to reach all the nodes from a single vertex. From 0 we can reach [0,1,2,5]. From 3 we can reach [3,4,2,5]. So we output [0,3].
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/07/untitled.png" style="width: 201px; height: 201px;" />
 *
 * Input: n = 5, edges = [[0,1],[2,1],[3,1],[1,4],[2,4]]
 * Output: [0,2,3]
 * Explanation: Notice that vertices 0, 3 and 2 are not reachable from any other node, so we must include them. Also any of these vertices can reach nodes 1 and 4.
 *
 *  
 * Constraints:
 *
 * 	2 <= n <= 10^5
 * 	1 <= edges.length <= min(10^5, n * (n - 1) / 2)
 * 	edges[i].length == 2
 * 	0 <= fromi, toi < n
 * 	All pairs (fromi, toi) are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/
// discuss: https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        use std::iter::FromIterator;

        let mut set: std::collections::HashSet<i32> =
            std::collections::HashSet::from_iter((0..n).into_iter());

        for edge in edges {
            set.remove(&edge[1]);
        }

        set.into_iter().collect::<Vec<i32>>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1557_example_1() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]];

        let result = vec![0, 3];

        assert_eq_sorted!(Solution::find_smallest_set_of_vertices(n, edges), result)
    }

    #[test]
    fn test_1557_example_2() {
        let n = 5;
        let edges = vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]];

        let result = vec![0, 2, 3];

        assert_eq_sorted!(Solution::find_smallest_set_of_vertices(n, edges), result)
    }
}

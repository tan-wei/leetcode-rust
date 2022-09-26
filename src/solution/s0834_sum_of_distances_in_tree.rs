/**
 * [0834] Sum of Distances in Tree
 *
 * There is an undirected connected tree with n nodes labeled from 0 to n - 1 and n - 1 edges.
 * You are given the integer n and the array edges where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the tree.
 * Return an array answer of length n where answer[i] is the sum of the distances between the i^th node in the tree and all other nodes.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist1.jpg" style="width: 304px; height: 224px;" />
 * Input: n = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
 * Output: [8,12,6,10,10,10]
 * Explanation: The tree is shown above.
 * We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
 * equals 1 + 1 + 2 + 2 + 2 = 8.
 * Hence, answer[0] = 8, and so on.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist2.jpg" style="width: 64px; height: 65px;" />
 * Input: n = 1, edges = []
 * Output: [0]
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist3.jpg" style="width: 144px; height: 145px;" />
 * Input: n = 2, edges = [[1,0]]
 * Output: [1,1]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 3 * 10^4
 * 	edges.length == n - 1
 * 	edges[i].length == 2
 * 	0 <= ai, bi < n
 * 	ai != bi
 * 	The given input represents a valid tree.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-distances-in-tree/
// discuss: https://leetcode.com/problems/sum-of-distances-in-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sum-of-distances-in-tree/discuss/892069/Rust-translated-8ms-100
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tree = vec![std::collections::HashSet::<i32>::new(); n as usize];
        let mut result = vec![0; n as usize];
        let mut count = vec![0; n as usize];

        for edge in &edges {
            tree[edge[0] as usize].insert(edge[1]);
            tree[edge[1] as usize].insert(edge[0]);
        }

        Self::dfs(&tree, &mut count, &mut result, 0, -1);
        Self::dfs2(&tree, &mut count, &mut result, 0, -1);

        result
    }

    fn dfs(
        tree: &[std::collections::HashSet<i32>],
        count: &mut Vec<i32>,
        result: &mut Vec<i32>,
        root: i32,
        pre: i32,
    ) {
        for &i in &tree[root as usize] {
            if i == pre {
                continue;
            }
            Self::dfs(tree, count, result, i, root);
            count[root as usize] += count[i as usize];
            result[root as usize] += result[i as usize] + count[i as usize];
        }
        count[root as usize] += 1;
    }

    fn dfs2(
        tree: &[std::collections::HashSet<i32>],
        count: &mut Vec<i32>,
        result: &mut Vec<i32>,
        root: i32,
        pre: i32,
    ) {
        for &i in &tree[root as usize] {
            if i == pre {
                continue;
            }
            result[i as usize] =
                result[root as usize] - count[i as usize] + count.len() as i32 - count[i as usize];
            Self::dfs2(tree, count, result, i, root);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0834_example_1() {
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]];
        let result = vec![8, 12, 6, 10, 10, 10];

        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), result);
    }

    #[test]
    fn test_0834_example_2() {
        let n = 1;
        let edges: Vec<Vec<i32>> = vec![];
        let result = vec![0];

        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), result);
    }

    #[test]
    fn test_0834_example_3() {
        let n = 2;
        let edges: Vec<Vec<i32>> = vec![vec![1, 0]];
        let result = vec![1, 1];

        assert_eq!(Solution::sum_of_distances_in_tree(n, edges), result);
    }
}

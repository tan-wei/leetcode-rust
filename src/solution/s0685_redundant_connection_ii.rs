/**
 * [0685] Redundant Connection II
 *
 * In this problem, a rooted tree is a directed graph such that, there is exactly one node (the root) for which all other nodes are descendants of this node, plus every node has exactly one parent, except for the root node which has no parents.
 * The given input is a directed graph that started as a rooted tree with n nodes (with distinct values from 1 to n), with one additional directed edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed.
 * The resulting graph is given as a 2D-array of edges. Each element of edges is a pair [ui, vi] that represents a directed edge connecting nodes ui and vi, where ui is a parent of child vi.
 * Return an edge that can be removed so that the resulting graph is a rooted tree of n nodes. If there are multiple answers, return the answer that occurs last in the given 2D-array.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploadisjoint_set/2020/12/20/graph1.jpg" style="width: 222px; height: 222px;" />
 * Input: edges = [[1,2],[1,3],[2,3]]
 * Output: [2,3]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploadisjoint_set/2020/12/20/graph2.jpg" style="width: 222px; height: 382px;" />
 * Input: edges = [[1,2],[2,3],[3,4],[4,1],[1,5]]
 * Output: [4,1]
 *
 *  
 * Constraints:
 *
 * 	n == edges.length
 * 	3 <= n <= 1000
 * 	edges[i].length == 2
 * 	1 <= ui, vi <= n
 * 	ui != vi
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/redundant-connection-ii/
// discuss: https://leetcode.com/problems/redundant-connection-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/redundant-connection-ii/discuss/108058/one-pass-disjoint-set-solution-with-explain
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parents = vec![-1; n + 1];
        let mut disjoint_set = vec![0; n + 1];
        let mut x1 = -1i32;
        let mut x2 = -1i32;
        let mut xn = -1i32;

        for i in 0..n {
            let p = edges[i][0];
            let c = edges[i][1];
            if parents[c as usize] != -1 {
                x1 = parents[c as usize];
                x2 = i as i32;
                continue;
            }
            parents[c as usize] = i as i32;
            let p1 = Self::dfs_helper(&mut disjoint_set, p);
            if p1 == c {
                xn = i as i32;
            } else {
                disjoint_set[c as usize] = p1;
            }
        }

        if xn == -1 {
            return edges[x2 as usize].clone();
        }

        if x2 == -1 {
            return edges[xn as usize].clone();
        }

        edges[x1 as usize].clone()
    }

    fn dfs_helper(disjoint_set: &mut Vec<i32>, i: i32) -> i32 {
        if disjoint_set[i as usize] == 0 {
            i
        } else {
            disjoint_set[i as usize] = Self::dfs_helper(disjoint_set, disjoint_set[i as usize]);
            disjoint_set[i as usize]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0685_example_1() {
        let edges = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
        let result = vec![2, 3];

        assert_eq!(Solution::find_redundant_directed_connection(edges), result);
    }

    #[test]
    fn test_0685_example_2() {
        let edges = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]];
        let result = vec![4, 1];

        assert_eq!(Solution::find_redundant_directed_connection(edges), result);
    }
}

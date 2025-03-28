/**
 * [1938] Maximum Genetic Difference Query
 *
 * There is a rooted tree consisting of n nodes numbered 0 to n - 1. Each node's number denotes its unique genetic value (i.e. the genetic value of node x is x). The genetic difference between two genetic values is defined as the bitwise-XOR of their values. You are given the integer array parents, where parents[i] is the parent for node i. If node x is the root of the tree, then parents[x] == -1.
 * You are also given the array queries where queries[i] = [nodei, vali]. For each query i, find the maximum genetic difference between vali and pi, where pi is the genetic value of any node that is on the path between nodei and the root (including nodei and the root). More formally, you want to maximize vali XOR pi.
 * Return an array ans where ans[i] is the answer to the i^th query.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/c1.png" style="width: 118px; height: 163px;" />
 * Input: parents = [-1,0,1,1], queries = [[0,2],[3,2],[2,5]]
 * Output: [2,3,7]
 * Explanation: The queries are processed as follows:
 * - [0,2]: The node with the maximum genetic difference is 0, with a difference of 2 XOR 0 = 2.
 * - [3,2]: The node with the maximum genetic difference is 1, with a difference of 2 XOR 1 = 3.
 * - [2,5]: The node with the maximum genetic difference is 2, with a difference of 5 XOR 2 = 7.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/29/c2.png" style="width: 256px; height: 221px;" />
 * Input: parents = [3,7,-1,2,0,7,0,2], queries = [[4,6],[1,15],[0,5]]
 * Output: [6,14,7]
 * Explanation: The queries are processed as follows:
 * - [4,6]: The node with the maximum genetic difference is 0, with a difference of 6 XOR 0 = 6.
 * - [1,15]: The node with the maximum genetic difference is 1, with a difference of 15 XOR 1 = 14.
 * - [0,5]: The node with the maximum genetic difference is 2, with a difference of 5 XOR 2 = 7.
 *
 *  
 * Constraints:
 *
 * 	2 <= parents.length <= 10^5
 * 	0 <= parents[i] <= parents.length - 1 for every node i that is not the root.
 * 	parents[root] == -1
 * 	1 <= queries.length <= 3 * 10^4
 * 	0 <= nodei <= parents.length - 1
 * 	0 <= vali <= 2 * 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-genetic-difference-query/
// discuss: https://leetcode.com/problems/maximum-genetic-difference-query/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1938_example_1() {
        let parents = vec![-1, 0, 1, 1];
        let queries = vec![vec![0, 2], vec![3, 2], vec![2, 5]];

        let result = vec![2, 3, 7];

        assert_eq!(Solution::max_genetic_difference(parents, queries), result);
    }

    #[test]
    #[ignore]
    fn test_1938_example_2() {
        let parents = vec![3, 7, -1, 2, 0, 7, 0, 2];
        let queries = vec![vec![4, 6], vec![1, 15], vec![0, 5]];

        let result = vec![6, 14, 7];

        assert_eq!(Solution::max_genetic_difference(parents, queries), result);
    }
}

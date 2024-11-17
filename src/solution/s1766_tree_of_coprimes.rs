/**
 * [1766] Tree of Coprimes
 *
 * There is a tree (i.e., a connected, undirected graph that has no cycles) consisting of n nodes numbered from 0 to n - 1 and exactly n - 1 edges. Each node has a value associated with it, and the root of the tree is node 0.
 * To represent this tree, you are given an integer array nums and a 2D array edges. Each nums[i] represents the i^th node's value, and each edges[j] = [uj, vj] represents an edge between nodes uj and vj in the tree.
 * Two values x and y are coprime if gcd(x, y) == 1 where gcd(x, y) is the greatest common divisor of x and y.
 * An ancestor of a node i is any other node on the shortest path from node i to the root. A node is not considered an ancestor of itself.
 * Return an array ans of size n, where ans[i] is the closest ancestor to node i such that nums[i] and nums[ans[i]] are coprime, or -1 if there is no such ancestor.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/06/untitled-diagram.png" style="width: 191px; height: 281px;" />
 *
 * Input: nums = [2,3,3,2], edges = [[0,1],[1,2],[1,3]]
 * Output: [-1,0,0,1]
 * Explanation: In the above figure, each node's value is in parentheses.
 * - Node 0 has no coprime ancestors.
 * - Node 1 has only one ancestor, node 0. Their values are coprime (gcd(2,3) == 1).
 * - Node 2 has two ancestors, nodes 1 and 0. Node 1's value is not coprime (gcd(3,3) == 3), but node 0's
 *   value is (gcd(2,3) == 1), so node 0 is the closest valid ancestor.
 * - Node 3 has two ancestors, nodes 1 and 0. It is coprime with node 1 (gcd(3,2) == 1), so node 1 is its
 *   closest valid ancestor.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/06/untitled-diagram1.png" style="width: 441px; height: 291px;" />
 *
 * Input: nums = [5,6,10,2,3,6,15], edges = [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]]
 * Output: [-1,0,-1,0,0,0,-1]
 *
 *  
 * Constraints:
 *
 * 	nums.length == n
 * 	1 <= nums[i] <= 50
 * 	1 <= n <= 10^5
 * 	edges.length == n - 1
 * 	edges[j].length == 2
 * 	0 <= uj, vj < n
 * 	uj != vj
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/tree-of-coprimes/
// discuss: https://leetcode.com/problems/tree-of-coprimes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_coprimes(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1766_example_1() {
        let nums = vec![2, 3, 3, 2];
        let edges = vec![vec![0, 1], vec![1, 2], vec![1, 3]];

        let result = vec![-1, 0, 0, 1];

        assert_eq!(Solution::get_coprimes(nums, edges), result);
    }

    #[test]
    #[ignore]
    fn test_1766_example_2() {
        let nums = vec![5, 6, 10, 2, 3, 6, 15];
        let edges = vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
        ];

        let result = vec![-1, 0, -1, 0, 0, 0, -1];

        assert_eq!(Solution::get_coprimes(nums, edges), result);
    }
}

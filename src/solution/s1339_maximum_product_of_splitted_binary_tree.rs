/**
 * [1339] Maximum Product of Splitted Binary Tree
 *
 * Given the root of a binary tree, split the binary tree into two subtrees by removing one edge such that the product of the sums of the subtrees is maximized.
 * Return the maximum product of the sums of the two subtrees. Since the answer may be too large, return it modulo 10^9 + 7.
 * Note that you need to maximize the answer before taking the mod and not after taking it.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/sample_1_1699.png" style="width: 500px; height: 167px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: 110
 * Explanation: Remove the red edge and get 2 binary trees with sum 11 and 10. Their product is 110 (11*10)
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/21/sample_2_1699.png" style="width: 500px; height: 211px;" />
 * Input: root = [1,null,2,3,4,null,null,5,6]
 * Output: 90
 * Explanation: Remove the red edge and get 2 binary trees with sum 15 and 6.Their product is 90 (15*6)
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 5 * 10^4].
 * 	1 <= Node.val <= 10^4
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/
// discuss: https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sums = Vec::new();
        Self::dfs(&root, &mut sums);
        ((0..sums.len() - 1)
            .map(|i| sums[i] * (sums[sums.len() - 1] - sums[i]))
            .max()
            .unwrap()
            % MOD) as i32
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, sums: &mut Vec<i64>) -> i64 {
        if let Some(n) = node {
            let sum = n.borrow().val as i64
                + Self::dfs(&n.borrow().left, sums)
                + Self::dfs(&n.borrow().right, sums);
            sums.push(sum);
            sum
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1339_example_1() {
        let root = tree![1, 2, 3, 4, 5, 6];

        let result = 110;

        assert_eq!(Solution::max_product(root), result);
    }

    #[test]
    fn test_1339_example_2() {
        let root = tree![1, null, 2, 3, 4, null, null, 5, 6];

        let result = 90;

        assert_eq!(Solution::max_product(root), result);
    }
}

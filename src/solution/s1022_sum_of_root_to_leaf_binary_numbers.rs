/**
 * [1022] Sum of Root To Leaf Binary Numbers
 *
 * You are given the root of a binary tree where each node has a value 0 or 1. Each root-to-leaf path represents a binary number starting with the most significant bit.
 *
 * 	For example, if the path is 0 -> 1 -> 1 -> 0 -> 1, then this could represent 01101 in binary, which is 13.
 *
 * For all leaves in the tree, consider the numbers represented by the path from the root to that leaf. Return the sum of these numbers.
 * The test cases are generated so that the answer fits in a 32-bits integer.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/04/04/sum-of-root-to-leaf-binary-numbers.png" style="width: 400px; height: 263px;" />
 * Input: root = [1,0,1,0,1,0,1]
 * Output: 22
 * Explanation: (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
 *
 * Example 2:
 *
 * Input: root = [0]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	Node.val is 0 or 1.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/
// discuss: https://leetcode.com/problems/sum-of-root-to-leaf-binary-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

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
impl Solution {
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dfs_helper(&root, 0)
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, num: i32) -> i32 {
        if let Some(node) = root {
            let val = num * 2 + node.borrow().val;
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                val
            } else {
                Solution::dfs_helper(&node.borrow().left, val)
                    + Solution::dfs_helper(&node.borrow().right, val)
            }
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
    fn test_1022_example_1() {
        let root = tree![1, 0, 1, 0, 1, 0, 1];
        let result = 22;

        assert_eq!(Solution::sum_root_to_leaf(root), result);
    }

    #[test]
    fn test_1022_example_2() {
        let root = tree![0];
        let result = 0;

        assert_eq!(Solution::sum_root_to_leaf(root), result);
    }
}

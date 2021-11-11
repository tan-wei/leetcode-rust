/**
 * [0404] Sum of Left Leaves
 *
 * Given the root of a binary tree, return the sum of all left leaves.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/leftsum-tree.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 24
 * Explanation: There are two left leaves in the binary tree, with values 9 and 15 respectively.
 *
 * Example 2:
 *
 * Input: root = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	-1000 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/sum-of-left-leaves/
// discuss: https://leetcode.com/problems/sum-of-left-leaves/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        if let Some(node) = root {
            let node = node.borrow();
            let left = &node.left;
            let right = &node.right;
            if let Some(left_node) = left {
                let left_node = left_node.borrow();
                if left_node.left.is_none() && left_node.right.is_none() {
                    result += left_node.val;
                } else {
                    result += Self::sum_of_left_leaves(left.clone());
                }
            }
            result += Self::sum_of_left_leaves(right.clone());
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0404_example_1() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        let result = 24;

        assert_eq!(Solution::sum_of_left_leaves(root), result);
    }

    #[test]
    fn test_0404_example_2() {
        let root = tree![1];
        let result = 0;

        assert_eq!(Solution::sum_of_left_leaves(root), result);
    }
}

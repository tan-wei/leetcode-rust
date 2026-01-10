/**
 * [1372] Longest ZigZag Path in a Binary Tree
 *
 * You are given the root of a binary tree.
 * A ZigZag path for a binary tree is defined as follow:
 *
 * 	Choose any node in the binary tree and a direction (right or left).
 * 	If the current direction is right, move to the right child of the current node; otherwise, move to the left child.
 * 	Change the direction from right to left or from left to right.
 * 	Repeat the second and third steps until you can't move in the tree.
 *
 * Zigzag length is defined as the number of nodes visited - 1. (A single node has a length of 0).
 * Return the longest ZigZag path contained in that tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/22/sample_1_1702.png" style="width: 221px; height: 383px;" />
 * Input: root = [1,null,1,1,1,null,null,1,1,null,1,null,null,null,1]
 * Output: 3
 * Explanation: Longest ZigZag path in blue nodes (right -> left -> right).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/22/sample_2_1702.png" style="width: 157px; height: 329px;" />
 * Input: root = [1,1,1,null,1,null,null,1,1,null,1]
 * Output: 4
 * Explanation: Longest ZigZag path in blue nodes (left -> right -> left -> right).
 *
 * Example 3:
 *
 * Input: root = [1]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 5 * 10^4].
 * 	1 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/
// discuss: https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs_helper(&root, 0, 0)
    }

    fn dfs_helper(node: &Option<Rc<RefCell<TreeNode>>>, l_count: i32, r_count: i32) -> i32 {
        match node.as_ref() {
            None => l_count.max(r_count) - 1,
            Some(n) => {
                let b = n.borrow();
                Self::dfs_helper(&b.left, r_count + 1, 0).max(Self::dfs_helper(
                    &b.right,
                    0,
                    l_count + 1,
                ))
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1372_example_1() {
        let root = tree![
            1, null, 1, 1, 1, null, null, 1, 1, null, 1, null, null, null, 1
        ];

        let result = 3;

        assert_eq!(Solution::longest_zig_zag(root), result);
    }

    #[test]
    fn test_1372_example_2() {
        let root = tree![1, 1, 1, null, 1, null, null, 1, 1, null, 1];

        let result = 4;

        assert_eq!(Solution::longest_zig_zag(root), result);
    }

    #[test]
    fn test_1372_example_3() {
        let root = tree![1];

        let result = 0;

        assert_eq!(Solution::longest_zig_zag(root), result);
    }
}

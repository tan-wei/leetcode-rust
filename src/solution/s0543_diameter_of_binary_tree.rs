/**
 * [0543] Diameter of Binary Tree
 *
 * Given the root of a binary tree, return the length of the diameter of the tree.
 * The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
 * The length of a path between two nodes is represented by the number of edges between them.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/06/diamtree.jpg" style="width: 292px; height: 302px;" />
 * Input: root = [1,2,3,4,5]
 * Output: 3
 * Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
 *
 * Example 2:
 *
 * Input: root = [1,2]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/diameter-of-binary-tree/
// discuss: https://leetcode.com/problems/diameter-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Self::dfs_helper(&root, &mut result);
        result
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, current_max: &mut i32) -> i32 {
        if let Some(root) = root {
            let left = Self::dfs_helper(&root.borrow().left, current_max);
            let right = Self::dfs_helper(&root.borrow().right, current_max);

            (*current_max) = (*current_max).max(left + right);

            left.max(right) + 1
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
    fn test_0543_example_1() {
        let root = tree![1, 2, 3, 4, 5];
        let result = 3;

        assert_eq!(Solution::diameter_of_binary_tree(root), result);
    }

    #[test]
    fn test_0543_example_2() {
        let root = tree![1, 2];
        let result = 1;

        assert_eq!(Solution::diameter_of_binary_tree(root), result);
    }
}

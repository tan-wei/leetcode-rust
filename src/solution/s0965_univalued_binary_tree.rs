/**
 * [0965] Univalued Binary Tree
 *
 * A binary tree is uni-valued if every node in the tree has the same value.
 * Given the root of a binary tree, return true if the given tree is uni-valued, or false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/28/unival_bst_1.png" style="width: 265px; height: 172px;" />
 * Input: root = [1,1,1,1,1,null,1]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/28/unival_bst_2.png" style="width: 198px; height: 169px;" />
 * Input: root = [2,2,2,5,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 100].
 * 	0 <= Node.val < 100
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/univalued-binary-tree/
// discuss: https://leetcode.com/problems/univalued-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let node = node.borrow();

                Self::dfs_helper(&node.left, node.val) && Self::dfs_helper(&node.right, node.val)
            }
            None => true,
        }
    }

    fn dfs_helper(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
        match node {
            Some(node) => {
                let node = node.borrow();

                node.val == val
                    && Self::dfs_helper(&node.left, val)
                    && Self::dfs_helper(&node.right, val)
            }
            None => true,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0965_example_1() {
        let root = tree![1, 1, 1, 1, 1, null, 1];
        let result = true;

        assert_eq!(Solution::is_unival_tree(root), result);
    }

    #[test]
    fn test_0965_example_2() {
        let root = tree![2, 2, 2, 5, 2];
        let result = false;

        assert_eq!(Solution::is_unival_tree(root), result);
    }
}

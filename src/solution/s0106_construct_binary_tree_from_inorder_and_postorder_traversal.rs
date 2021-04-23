/**
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
 *
 * Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;" />
 * Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
 * Output: [3,9,20,null,null,15,7]
 *
 * Example 2:
 *
 * Input: inorder = [-1], postorder = [-1]
 * Output: [-1]
 *
 *  
 * Constraints:
 *
 * 	1 <= inorder.length <= 3000
 * 	postorder.length == inorder.length
 * 	-3000 <= inorder[i], postorder[i] <= 3000
 * 	inorder and postorder consist of unique values.
 * 	Each value of postorder also appears in inorder.
 * 	inorder is guaranteed to be the inorder traversal of the tree.
 * 	postorder is guaranteed to be the postorder traversal of the tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(root) = postorder.last() {
                let pivot_idx = inorder
                    .iter()
                    .enumerate()
                    .find(|(_, v)| v == &root)
                    .unwrap()
                    .0;

                Some(Rc::new(RefCell::new(TreeNode {
                    val: *root,
                    left: helper(&inorder[0..pivot_idx], &postorder[0..pivot_idx]),
                    right: helper(
                        &inorder[(1 + pivot_idx)..],
                        &postorder[pivot_idx..(postorder.len() - 1)],
                    ),
                })))
            } else {
                None
            }
        }

        helper(&inorder, &postorder)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0106_example_1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let result = tree![3, 9, 20, null, null, 15, 7];

        assert_eq!(Solution::build_tree(inorder, postorder), result);
    }

    #[test]
    fn test_0106_example_2() {
        let inorder = vec![-1];
        let postorder = vec![-1];
        let result = tree![-1];

        assert_eq!(Solution::build_tree(inorder, postorder), result);
    }
}

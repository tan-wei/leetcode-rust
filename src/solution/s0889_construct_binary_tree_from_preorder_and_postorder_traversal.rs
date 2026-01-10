/**
 * [0889] Construct Binary Tree from Preorder and Postorder Traversal
 *
 * Given two integer arrays, preorder and postorder where preorder is the preorder traversal of a binary tree of distinct values and postorder is the postorder traversal of the same tree, reconstruct and return the binary tree.
 * If there exist multiple answers, you can return any of them.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/24/lc-prepost.jpg" style="width: 304px; height: 265px;" />
 * Input: preorder = [1,2,4,5,3,6,7], postorder = [4,5,2,6,7,3,1]
 * Output: [1,2,3,4,5,6,7]
 *
 * Example 2:
 *
 * Input: preorder = [1], postorder = [1]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	1 <= preorder.length <= 30
 * 	1 <= preorder[i] <= preorder.length
 * 	All the values of preorder are unique.
 * 	postorder.length == preorder.length
 * 	1 <= postorder[i] <= postorder.length
 * 	All the values of postorder are unique.
 * 	It is guaranteed that preorder and postorder are the preorder traversal and postorder traversal of the same binary tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (mut pre, mut post) = (0, 0);

        Self::dfs(&preorder, &postorder, &mut pre, &mut post)
    }

    fn dfs(
        preorder: &[i32],
        postorder: &[i32],
        pre: &mut usize,
        post: &mut usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = Rc::new(RefCell::new(TreeNode {
            val: preorder[*pre],
            right: None,
            left: None,
        }));

        *pre += 1;
        if node.borrow().val != postorder[*post] {
            node.borrow_mut().left = Self::dfs(preorder, postorder, pre, post);
        }
        if node.borrow().val != postorder[*post] {
            node.borrow_mut().right = Self::dfs(preorder, postorder, pre, post);
        }
        *post += 1;

        Some(node)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0889_example_1() {
        let preorder = vec![1, 2, 4, 5, 3, 6, 7];
        let postorder = vec![4, 5, 2, 6, 7, 3, 1];
        let result = tree![1, 2, 3, 4, 5, 6, 7];

        assert_eq!(
            Solution::construct_from_pre_post(preorder, postorder),
            result
        );
    }

    #[test]
    fn test_0889_example_2() {
        let preorder = vec![1];
        let postorder = vec![1];
        let result = tree![1];

        assert_eq!(
            Solution::construct_from_pre_post(preorder, postorder),
            result
        );
    }
}

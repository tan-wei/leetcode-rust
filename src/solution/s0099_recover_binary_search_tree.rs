/**
 * [99] Recover Binary Search Tree
 *
 * You are given the root of a binary search tree (BST), where exactly two nodes of the tree were swapped by mistake. Recover the tree without changing its structure.
 * Follow up: A solution using O(n) space is pretty straight forward. Could you devise a constant space solution?
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/28/recover1.jpg" style="width: 422px; height: 302px;" />
 * Input: root = [1,3,null,null,2]
 * Output: [3,1,null,null,2]
 * Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/28/recover2.jpg" style="width: 581px; height: 302px;" />
 * Input: root = [3,1,4,null,null,2]
 * Output: [2,1,4,null,null,3]
 * Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 1000].
 * 	-2^31 <= Node.val <= 2^31 - 1
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/recover-binary-search-tree/
// discuss: https://leetcode.com/problems/recover-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/recover-binary-search-tree/discuss/448826/Rust-Inorder-Solution
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn helper(
            node: &Option<Rc<RefCell<TreeNode>>>,
            first: &mut Option<Rc<RefCell<TreeNode>>>,
            second: &mut Option<Rc<RefCell<TreeNode>>>,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(real_node) = node {
                let real_node_ref = real_node.as_ref().borrow();
                helper(&real_node_ref.left, first, second, prev);
                if let Some(prev_node) = prev {
                    if prev_node.as_ref().borrow().val >= real_node_ref.val {
                        if first.is_none() {
                            *first = Some(prev_node.clone());
                        }
                        if first.is_some() {
                            *second = Some(real_node.clone());
                        }
                    }
                }
                *prev = Some(real_node.clone());
                helper(&real_node_ref.right, first, second, prev);
            }
        }

        let (mut first, mut second) = (None, None);
        let mut prev = None;
        helper(root, &mut first, &mut second, &mut prev);
        std::mem::swap(
            &mut first.unwrap().borrow_mut().val,
            &mut second.unwrap().borrow_mut().val,
        );
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0099_example_1() {
        let mut root = tree![1, 3, null, null, 2];
        let result = tree![3, 1, null, null, 2];

        Solution::recover_tree(&mut root);

        assert_eq!(root, result);
    }

    #[test]
    fn test_0099_example_2() {
        let mut root = tree![3, 1, 4, null, null, 2];
        let result = tree![2, 1, 4, null, null, 3];

        Solution::recover_tree(&mut root);

        assert_eq!(root, result);
    }
}

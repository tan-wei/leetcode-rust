/**
 * [0700] Search in a Binary Search Tree
 *
 * You are given the root of a binary search tree (BST) and an integer val.
 * Find the node in the BST that the node's value equals val and return the subtree rooted with that node. If such a node does not exist, return null.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/12/tree1.jpg" style="width: 422px; height: 302px;" />
 * Input: root = [4,2,7,1,3], val = 2
 * Output: [2,1,3]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/12/tree2.jpg" style="width: 422px; height: 302px;" />
 * Input: root = [4,2,7,1,3], val = 5
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 5000].
 * 	1 <= Node.val <= 10^7
 * 	root is a binary search tree.
 * 	1 <= val <= 10^7
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/search-in-a-binary-search-tree/
// discuss: https://leetcode.com/problems/search-in-a-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        while let Some(rc_node) = root.clone() {
            let cur_node = rc_node.borrow();
            match cur_node.val.cmp(&val) {
                std::cmp::Ordering::Equal => return root,
                std::cmp::Ordering::Less if cur_node.right.is_some() => {
                    root = cur_node.right.clone()
                }
                std::cmp::Ordering::Greater if cur_node.left.is_some() => {
                    root = cur_node.left.clone()
                }
                _ => break,
            }
        }
        None
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0700_example_1() {
        let root = tree![4, 2, 7, 1, 3];
        let val = 2;
        let result = tree![2, 1, 3];

        assert_eq!(Solution::search_bst(root, val), result);
    }

    #[test]
    fn test_0700_example_2() {
        let root = tree![4, 2, 7, 1, 3];
        let val = 5;
        let result = tree![];

        assert_eq!(Solution::search_bst(root, val), result);
    }
}

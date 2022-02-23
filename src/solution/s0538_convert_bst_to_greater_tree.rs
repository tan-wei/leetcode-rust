/**
 * [0538] Convert BST to Greater Tree
 *
 * Given the root of a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original BST is changed to the original key plus the sum of all keys greater than the original key in BST.
 * As a reminder, a binary search tree is a tree that satisfies these constraints:
 *
 * 	The left subtree of a node contains only nodes with keys less than the node's key.
 * 	The right subtree of a node contains only nodes with keys greater than the node's key.
 * 	Both the left and right subtrees must also be binary search trees.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/02/tree.png" style="width: 500px; height: 341px;" />
 * Input: root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
 * Output: [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
 *
 * Example 2:
 *
 * Input: root = [0,null,1]
 * Output: [1,null,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	-10^4 <= Node.val <= 10^4
 * 	All the values in the tree are unique.
 * 	root is guaranteed to be a valid binary search tree.
 *
 *  
 * Note: This question is the same as 1038: <a href="https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/" target="_blank">https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/</a>
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/convert-bst-to-greater-tree/
// discuss: https://leetcode.com/problems/convert-bst-to-greater-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        let mut root = root;
        Self::dfs_helper(&mut root, &mut sum);
        root
    }

    fn dfs_helper(r: &mut Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if let Some(r) = r {
            let mut r = r.borrow_mut();
            Self::dfs_helper(&mut r.right, sum);
            r.val += *sum;
            *sum = r.val;
            Self::dfs_helper(&mut r.left, sum);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0538_example_1() {
        let root = tree![4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8];
        let result = tree![30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8];

        assert_eq!(Solution::convert_bst(root), result);
    }

    #[test]
    fn test_0538_example_2() {
        let root = tree![0, null, 1];
        let result = tree![1, null, 1];

        assert_eq!(Solution::convert_bst(root), result);
    }
}

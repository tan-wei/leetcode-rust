/**
 * [0623] Add One Row to Tree
 *
 * Given the root of a binary tree and two integers val and depth, add a row of nodes with value val at the given depth depth.
 * Note that the root node is at depth 1.
 * The adding rule is:
 *
 * 	Given the integer depth, for each not null tree node cur at the depth depth - 1, create two tree nodes with value val as cur's left subtree root and right subtree root.
 * 	cur's original left subtree should be the left subtree of the new left subtree root.
 * 	cur's original right subtree should be the right subtree of the new right subtree root.
 * 	If depth == 1 that means there is no depth depth - 1 at all, then create a tree node with value val as the new root of the whole original tree, and the original tree is the new root's left subtree.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/15/addrow-tree.jpg" style="width: 500px; height: 231px;" />
 * Input: root = [4,2,6,3,1,5], val = 1, depth = 2
 * Output: [4,1,1,2,null,null,6,3,1,5]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/11/add2-tree.jpg" style="width: 500px; height: 277px;" />
 * Input: root = [4,2,null,3,1], val = 1, depth = 3
 * Output: [4,2,null,1,1,3,null,null,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	The depth of the tree is in the range [1, 10^4].
 * 	-100 <= Node.val <= 100
 * 	-10^5 <= val <= 10^5
 * 	1 <= depth <= the depth of tree + 1
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/add-one-row-to-tree/
// discuss: https://leetcode.com/problems/add-one-row-to-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            Some(Rc::new(RefCell::new(TreeNode {
                val: val,
                left: root,
                right: None,
            })))
        } else {
            Self::dfs_helper(&root, val, depth - 2);
            root
        }
    }

    fn dfs_helper(node: &Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) {
        if let Some(n) = node {
            if depth > 0 {
                Self::dfs_helper(&n.borrow().left, val, depth - 1);
                Self::dfs_helper(&n.borrow().right, val, depth - 1);
            } else {
                let mut n = n.borrow_mut();
                n.left = Some(Rc::new(RefCell::new(TreeNode {
                    val: val,
                    left: n.left.take(),
                    right: None,
                })));
                n.right = Some(Rc::new(RefCell::new(TreeNode {
                    val: val,
                    left: None,
                    right: n.right.take(),
                })));
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0623_example_1() {
        let root = tree![4, 2, 6, 3, 1, 5];
        let val = 1;
        let depth = 2;
        let result = tree![4, 1, 1, 2, null, null, 6, 3, 1, 5];

        assert_eq!(Solution::add_one_row(root, val, depth), result);
    }

    #[test]
    fn test_0623_example_2() {
        let root = tree![4, 2, null, 3, 1];
        let val = 1;
        let depth = 3;
        let result = tree![4, 2, null, 1, 1, 3, null, null, 1];

        assert_eq!(Solution::add_one_row(root, val, depth), result);
    }
}

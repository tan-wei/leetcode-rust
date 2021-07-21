/**
 * [226] Invert Binary Tree
 *
 * Given the root of a binary tree, invert the tree, and return its root.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/invert1-tree.jpg" style="width: 500px; height: 165px;" />
 * Input: root = [4,2,7,1,3,6,9]
 * Output: [4,7,2,9,6,3,1]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/invert2-tree.jpg" style="width: 500px; height: 120px;" />
 * Input: root = [2,1,3]
 * Output: [2,3,1]
 *
 * Example 3:
 *
 * Input: root = []
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/invert-binary-tree/
// discuss: https://leetcode.com/problems/invert-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(root) => {
                let (right, left) = (
                    Self::invert_tree(root.borrow().left.clone()),
                    Self::invert_tree(root.borrow().right.clone()),
                );
                let node = root.clone();
                node.borrow_mut().left = left;
                node.borrow_mut().right = right;
                Some(node)
            }
            None => None,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0226_example_1() {
        let root = tree![4, 2, 7, 1, 3, 6, 9];
        let result = tree![4, 7, 2, 9, 6, 3, 1];

        assert_eq!(Solution::invert_tree(root), result);
    }

    #[test]
    fn test_0226_example_2() {
        let root = tree![2, 1, 3];
        let result = tree![2, 3, 1];

        assert_eq!(Solution::invert_tree(root), result);
    }

    #[test]
    fn test_0226_example_3() {
        let root = tree![];
        let result = tree![];

        assert_eq!(Solution::invert_tree(root), result);
    }
}

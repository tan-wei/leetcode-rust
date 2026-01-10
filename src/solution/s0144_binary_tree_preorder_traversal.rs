/**
 * [144] Binary Tree Preorder Traversal
 *
 * Given the root of a binary tree, return the preorder traversal of its nodes' values.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_1.jpg" style="width: 202px; height: 324px;" />
 * Input: root = [1,null,2,3]
 * Output: [1,2,3]
 *
 * Example 2:
 *
 * Input: root = []
 * Output: []
 *
 * Example 3:
 *
 * Input: root = [1]
 * Output: [1]
 *
 * Example 4:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_5.jpg" style="width: 202px; height: 202px;" />
 * Input: root = [1,2]
 * Output: [1,2]
 *
 * Example 5:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_4.jpg" style="width: 202px; height: 202px;" />
 * Input: root = [1,null,2]
 * Output: [1,2]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 *
 *  
 * Follow up: Recursive solution is trivial, could you do it iteratively?
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-preorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-preorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => vec![],
            Some(root) => {
                let left = Self::preorder_traversal(root.as_ref().borrow().left.clone());
                let right = Self::preorder_traversal(root.as_ref().borrow().right.clone());
                let mut result = vec![root.as_ref().borrow().val];

                for i in left {
                    result.push(i);
                }

                for i in right {
                    result.push(i);
                }
                result
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0144_example_1() {
        let root = tree![1, null, 2, 3];
        let result = vec![1, 2, 3];

        assert_eq!(Solution::preorder_traversal(root), result);
    }

    #[test]
    fn test_0144_example_2() {
        let root = tree![];
        let result = vec![];

        assert_eq!(Solution::preorder_traversal(root), result);
    }

    #[test]
    fn test_0144_example_3() {
        let root = tree![1];
        let result = vec![1];

        assert_eq!(Solution::preorder_traversal(root), result);
    }

    #[test]
    fn test_0144_example_4() {
        let root = tree![1, 2];
        let result = vec![1, 2];

        assert_eq!(Solution::preorder_traversal(root), result);
    }

    #[test]
    fn test_0144_example_5() {
        let root = tree![1, null, 2];
        let result = vec![1, 2];

        assert_eq!(Solution::preorder_traversal(root), result);
    }
}

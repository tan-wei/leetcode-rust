/**
 * [94] Binary Tree Inorder Traversal
 *
 * Given the root of a binary tree, return the inorder traversal of its nodes' values.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_1.jpg" style="width: 202px; height: 324px;" />
 * Input: root = [1,null,2,3]
 * Output: [1,3,2]
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
 * Output: [2,1]
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
 * Follow up:
 * Recursive solution is trivial, could you do it iteratively?
 *  
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-inorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-inorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut node_stack = vec![];
        let mut pointer = root.clone();

        while let Some(node) = pointer {
            node_stack.push(node.clone());
            pointer = node.borrow().left.clone();
        }

        while let Some(node) = node_stack.pop() {
            res.push(node.borrow().val);
            pointer = node.borrow().right.clone();
            while let Some(n) = pointer {
                node_stack.push(n.clone());
                pointer = n.borrow().left.clone();
            }
        }

        res
    }

    pub fn inorder_traversal_v2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        Self::helper(&mut root, &mut res);
        res
    }

    fn helper(r: &mut Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        if let Some(n) = r {
            let mut n = n.borrow_mut();
            Self::helper(&mut n.left, order);
            order.push(n.val);
            Self::helper(&mut n.right, order);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0094_example_1() {
        let root = tree![1, null, 2, 3];
        let result = vec![1, 3, 2];

        assert_eq!(Solution::inorder_traversal(root), result);

        let root = tree![1, null, 2, 3];
        let result = vec![1, 3, 2];

        assert_eq!(Solution::inorder_traversal_v2(root), result);
    }

    #[test]
    fn test_0094_example_2() {
        let root = tree![];
        let result = vec![];

        assert_eq!(Solution::inorder_traversal(root), result);

        let root = tree![];
        let result = vec![];

        assert_eq!(Solution::inorder_traversal_v2(root), result);
    }

    #[test]
    fn test_0094_example_3() {
        let root = tree![1];
        let result = vec![1];

        assert_eq!(Solution::inorder_traversal(root), result);

        let root = tree![1];
        let result = vec![1];

        assert_eq!(Solution::inorder_traversal_v2(root), result);
    }

    #[test]
    fn test_0094_example_4() {
        let root = tree![1, 2];
        let result = vec![2, 1];

        assert_eq!(Solution::inorder_traversal(root), result);

        let root = tree![1, 2];
        let result = vec![2, 1];

        assert_eq!(Solution::inorder_traversal_v2(root), result);
    }

    #[test]
    fn test_0094_example_5() {
        let root = tree![1, null, 2];
        let result = vec![1, 2];

        assert_eq!(Solution::inorder_traversal(root), result);

        let root = tree![1, null, 2];
        let result = vec![1, 2];

        assert_eq!(Solution::inorder_traversal_v2(root), result);
    }
}

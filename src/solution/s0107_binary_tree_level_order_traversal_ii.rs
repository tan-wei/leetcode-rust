/**
 * [107] Binary Tree Level Order Traversal II
 *
 * Given the root of a binary tree, return the bottom-up level order traversal of its nodes' values. (i.e., from left to right, level by level from leaf to root).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[15,7],[9,20],[3]]
 *
 * Example 2:
 *
 * Input: root = [1]
 * Output: [[1]]
 *
 * Example 3:
 *
 * Input: root = []
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 2000].
 * 	-1000 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
// discuss: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut deque = std::collections::VecDeque::new();
        let mut result = Vec::new();

        deque.push_back(root.clone());

        while deque.len() > 0 {
            let mut level_vec: Vec<i32> = vec![];
            for i in 0..deque.len() {
                if let Some(Some(node)) = deque.pop_front() {
                    if node.borrow().left.is_some() {
                        deque.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        deque.push_back(node.borrow().right.clone());
                    }
                    level_vec.push(node.borrow().val);
                }
            }
            if level_vec.len() > 0 {
                result.push(level_vec.clone());
            }
        }

        result.reverse();
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0107_example_1() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        let result = vec![vec![15, 7], vec![9, 20], vec![3]];

        assert_eq!(Solution::level_order_bottom(root), result);
    }

    #[test]
    fn test_0107_example_2() {
        let root = tree![1];
        let result = vec![vec![1]];

        assert_eq!(Solution::level_order_bottom(root), result);
    }

    #[test]
    fn test_0107_example_3() {
        let root = tree![];
        let result: Vec<Vec<_>> = vec![];

        assert_eq!(Solution::level_order_bottom(root), result);
    }
}

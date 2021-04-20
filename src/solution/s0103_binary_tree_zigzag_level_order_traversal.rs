/**
 * [103] Binary Tree Zigzag Level Order Traversal
 *
 * Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[3],[20,9],[15,7]]
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
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
// discuss: https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = std::collections::VecDeque::<Rc<RefCell<TreeNode>>>::new();
        let mut result = vec![];
        let mut left_direction = true;

        if let Some(root) = root {
            queue.push_back(root);
        } else {
            return result;
        }

        while !queue.is_empty() {
            if left_direction {
                result.push(queue.iter().map(|node| node.borrow().val).collect());
                left_direction = false;
            } else {
                result.push(queue.iter().rev().map(|node| node.borrow().val).collect());
                left_direction = true;
            }

            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(Rc::clone(&left));
                    }
                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(Rc::clone(&right));
                    }
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0103_example_1() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        let result = vec![vec![3], vec![20, 9], vec![15, 7]];

        assert_eq!(Solution::zigzag_level_order(root), result);
    }

    #[test]
    fn test_0103_example_2() {
        let root = tree![1];
        let result = vec![vec![1]];

        assert_eq!(Solution::zigzag_level_order(root), result);
    }

    #[test]
    fn test_0103_example_3() {
        let root = tree![];
        let result: Vec<Vec<_>> = vec![];

        assert_eq!(Solution::zigzag_level_order(root), result);
    }
}

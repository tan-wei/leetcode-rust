/**
 * [0958] Check Completeness of a Binary Tree
 *
 * Given the root of a binary tree, determine if it is a complete binary tree.
 * In a <a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">complete binary tree</a>, every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible. It can have between 1 and 2^h nodes inclusive at the last level h.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/complete-binary-tree-1.png" style="width: 180px; height: 145px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: true
 * Explanation: Every level before the last is full (ie. levels with node-values {1} and {2, 3}), and all nodes in the last level ({4, 5, 6}) are as far left as possible.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/complete-binary-tree-2.png" style="width: 200px; height: 145px;" />
 * Input: root = [1,2,3,4,5,null,7]
 * Output: false
 * Explanation: The node with value 7 isn't as far left as possible.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 100].
 * 	1 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/check-completeness-of-a-binary-tree/
// discuss: https://leetcode.com/problems/check-completeness-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let mut nodes = vec![Some(root.clone())];
                let mut i = 0;

                while i < nodes.len() && nodes[i].is_some() {
                    let left = nodes[i].as_ref().unwrap().borrow().left.clone();
                    let right = nodes[i].as_ref().unwrap().borrow().right.clone();
                    nodes.push(left);
                    nodes.push(right);
                    i += 1;
                }

                while i < nodes.len() && nodes[i].is_none() {
                    i += 1;
                }
                i == nodes.len()
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0958_example_1() {
        let root = tree![1, 2, 3, 4, 5, 6];
        let result = true;

        assert_eq!(Solution::is_complete_tree(root), result);
    }

    #[test]
    fn test_0958_example_2() {
        let root = tree![1, 2, 3, 4, 5, null, 7];
        let result = false;

        assert_eq!(Solution::is_complete_tree(root), result);
    }
}

/**
 * [114] Flatten Binary Tree to Linked List
 *
 * Given the root of a binary tree, flatten the tree into a "linked list":
 *
 * 	The "linked list" should use the same TreeNode class where the right child pointer points to the next node in the list and the left child pointer is always null.
 * 	The "linked list" should be in the same order as a <a href="https://en.wikipedia.org/wiki/Tree_traversal#Pre-order,_NLR" target="_blank">pre-order traversal</a> of the binary tree.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/flaten.jpg" style="width: 500px; height: 226px;" />
 * Input: root = [1,2,5,3,4,null,6]
 * Output: [1,null,2,null,3,null,4,null,5,null,6]
 *
 * Example 2:
 *
 * Input: root = []
 * Output: []
 *
 * Example 3:
 *
 * Input: root = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 2000].
 * 	-100 <= Node.val <= 100
 *
 *  
 * Follow up: Can you flatten the tree in-place (with O(1) extra space)?
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/
// discuss: https://leetcode.com/problems/flatten-binary-tree-to-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn helper(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
            {
                let mut node = root.borrow_mut();
                if node.left.is_some() {
                    let lr = helper(Rc::clone(node.left.as_ref().unwrap()));
                    lr.borrow_mut().right = node.right.clone();
                    node.right = node.left.clone();
                    node.left = None;
                }

                if node.right.is_some() {
                    return helper(Rc::clone(node.right.as_ref().unwrap()));
                }
            }
            return root;
        }

        if let Some(node) = root {
            helper(Rc::clone(&node));
        };
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0114_example_1() {
        let mut root = tree![1, 2, 5, 3, 4, null, 6];
        let result = tree![1, null, 2, null, 3, null, 4, null, 5, null, 6];

        Solution::flatten(&mut root);

        assert_eq!(root, result);
    }

    #[test]
    fn test_0114_example_2() {
        let mut root = tree![];
        let result = tree![];

        Solution::flatten(&mut root);

        assert_eq!(root, result);
    }

    #[test]
    fn test_0114_example_3() {
        let mut root = tree![0];
        let result = tree![0];

        Solution::flatten(&mut root);

        assert_eq!(root, result);
    }
}

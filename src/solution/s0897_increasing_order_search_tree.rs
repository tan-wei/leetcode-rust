/**
 * [0897] Increasing Order Search Tree
 *
 * Given the root of a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/ex1.jpg" style="width: 600px; height: 350px;" />
 * Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
 * Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/17/ex2.jpg" style="width: 300px; height: 114px;" />
 * Input: root = [5,1,7]
 * Output: [1,null,5,null,7]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the given tree will be in the range [1, 100].
 * 	0 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/increasing-order-search-tree/
// discuss: https://leetcode.com/problems/increasing-order-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        let mut stack = Vec::new();
        let mut dm = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut cur = Rc::clone(&dm);

        while !stack.is_empty() || root.is_some() {
            while let Some(node) = root {
                root = node.borrow_mut().left.take();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                root = node.borrow_mut().right.take();
                cur.borrow_mut().right = Some(Rc::clone(&node));
                cur = node;
            }
        }

        let head = dm.borrow_mut().right.take();
        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0897_example_1() {
        let root = tree![5, 3, 6, 2, 4, null, 8, 1, null, null, null, 7, 9];
        let result =
            tree![1, null, 2, null, 3, null, 4, null, 5, null, 6, null, 7, null, 8, null, 9];

        assert_eq!(Solution::increasing_bst(root), result);
    }

    #[test]
    fn test_0897_example_2() {
        let root = tree![5, 1, 7];
        let result = tree![1, null, 5, null, 7];

        assert_eq!(Solution::increasing_bst(root), result);
    }
}

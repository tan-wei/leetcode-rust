/**
 * [98] Validate Binary Search Tree
 *
 * Given the root of a binary tree, determine if it is a valid binary search tree (BST).
 * A valid BST is defined as follows:
 *
 * 	The left subtree of a node contains only nodes with keys less than the node's key.
 * 	The right subtree of a node contains only nodes with keys greater than the node's key.
 * 	Both the left and right subtrees must also be binary search trees.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/tree1.jpg" style="width: 302px; height: 182px;" />
 * Input: root = [2,1,3]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/01/tree2.jpg" style="width: 422px; height: 292px;" />
 * Input: root = [5,1,4,null,null,3,6]
 * Output: false
 * Explanation: The root node's value is 5 but its right child's value is 4.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-2^31 <= Node.val <= 2^31 - 1
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/validate-binary-search-tree/
// discuss: https://leetcode.com/problems/validate-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid_helper(
            node: &Option<Rc<RefCell<TreeNode>>>,
            min_limit: Option<i32>,
            max_limit: Option<i32>,
        ) -> bool {
            if let Some(n) = node {
                let n = n.borrow();
                if let Some(v) = min_limit {
                    if n.val <= v {
                        return false;
                    }
                }
                if let Some(v) = max_limit {
                    if n.val >= v {
                        return false;
                    }
                }
                return is_valid_helper(&n.left, min_limit, Some(n.val))
                    && is_valid_helper(&n.right, Some(n.val), max_limit);
            } else {
                true
            }
        }

        is_valid_helper(&root, None, None)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0098_example_1() {
        let root = tree![2, 1, 3];
        let result = true;

        assert_eq!(Solution::is_valid_bst(root), result);
    }

    #[test]
    fn test_0098_example_2() {
        let root = tree![5, 1, 4, null, null, 3, 6];
        let result = false;

        assert_eq!(Solution::is_valid_bst(root), result);
    }
}

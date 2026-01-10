/**
 * [0669] Trim a Binary Search Tree
 *
 * Given the root of a binary search tree and the lowest and highest boundaries as low and high, trim the tree so that all its elements lies in [low, high]. Trimming the tree should not change the relative structure of the elements that will remain in the tree (i.e., any node's descendant should remain a descendant). It can be proven that there is a unique answer.
 * Return the root of the trimmed binary search tree. Note that the root may change depending on the given bounds.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/trim1.jpg" style="width: 450px; height: 126px;" />
 * Input: root = [1,0,2], low = 1, high = 2
 * Output: [1,null,2]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/09/trim2.jpg" style="width: 450px; height: 277px;" />
 * Input: root = [3,0,4,null,2,null,null,1], low = 1, high = 3
 * Output: [3,2,null,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	0 <= Node.val <= 10^4
 * 	The value of each node in the tree is unique.
 * 	root is guaranteed to be a valid binary search tree.
 * 	0 <= low <= high <= 10^4
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/trim-a-binary-search-tree/
// discuss: https://leetcode.com/problems/trim-a-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.as_ref() {
            let mut node = root.borrow_mut();
            if node.val < low {
                return Self::trim_bst(node.right.take(), low, high);
            } else if node.val > high {
                return Self::trim_bst(node.left.take(), low, high);
            }
            node.left = Self::trim_bst(node.left.take(), low, high);
            node.right = Self::trim_bst(node.right.take(), low, high);
        }
        root
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0669_example_1() {
        let root = tree![1, 0, 2];
        let low = 1;
        let high = 2;
        let result = tree![1, null, 2];

        assert_eq!(Solution::trim_bst(root, low, high), result);
    }

    #[test]
    fn test_0669_example_2() {
        let root = tree![3, 0, 4, null, 2, null, null, 1];
        let low = 1;
        let high = 3;
        let result = tree![3, 2, null, 1];

        assert_eq!(Solution::trim_bst(root, low, high), result);
    }
}

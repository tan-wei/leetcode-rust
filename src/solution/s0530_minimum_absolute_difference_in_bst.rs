/**
 * [0530] Minimum Absolute Difference in BST
 *
 * Given the root of a Binary Search Tree (BST), return the minimum absolute difference between the values of any two different nodes in the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/bst1.jpg" style="width: 292px; height: 301px;" />
 * Input: root = [4,2,6,1,3]
 * Output: 1
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/bst2.jpg" style="width: 282px; height: 301px;" />
 * Input: root = [1,0,48,null,null,12,49]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 10^4].
 * 	0 <= Node.val <= 10^5
 *
 *  
 * Note: This question is the same as 783: <a href="https://leetcode.com/problems/minimum-distance-between-bst-nodes/" target="_blank">https://leetcode.com/problems/minimum-distance-between-bst-nodes/</a>
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/minimum-absolute-difference-in-bst/
// discuss: https://leetcode.com/problems/minimum-absolute-difference-in-bst/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs_helper(&root, i32::MIN, i32::MAX).1
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, last: i32, res: i32) -> (i32, i32) {
        match root {
            Some(node) => {
                let b = node.borrow();
                let (mut last, mut res) = Self::dfs_helper(&b.left, last, res);
                res = res.min(b.val.saturating_sub(last));
                last = b.val;
                Self::dfs_helper(&b.right, last, res)
            }
            _ => (last, res),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0530_example_1() {
        let root = tree![4, 2, 6, 1, 3];
        let result = 1;

        assert_eq!(Solution::get_minimum_difference(root), result);
    }

    #[test]
    fn test_0530_example_2() {
        let root = tree![1, 0, 48, null, null, 12, 49];
        let result = 1;

        assert_eq!(Solution::get_minimum_difference(root), result);
    }
}

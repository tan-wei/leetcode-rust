/**
 * [0687] Longest Univalue Path
 *
 * Given the root of a binary tree, return the length of the longest path, where each node in the path has the same value. This path may or may not pass through the root.
 * The length of the path between two nodes is represented by the number of edges between them.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/ex1.jpg" style="width: 571px; height: 302px;" />
 * Input: root = [5,4,5,1,1,5]
 * Output: 2
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/ex2.jpg" style="width: 571px; height: 302px;" />
 * Input: root = [1,4,5,4,4,5]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	-1000 <= Node.val <= 1000
 * 	The depth of the tree will not exceed 1000.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/longest-univalue-path/
// discuss: https://leetcode.com/problems/longest-univalue-path/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cur = 0;
        Self::dfs_helper(&root, &mut cur);
        cur
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, cur: &mut i32) -> i32 {
        if let Some(root) = root {
            let mut left_max = Self::dfs_helper(&root.borrow().left, cur);
            let mut right_max = Self::dfs_helper(&root.borrow().right, cur);

            let (mut left_cur, mut right_cur) = (0, 0);
            if let Some(left_root) = &root.borrow().left {
                let left_root = left_root.borrow();
                if left_root.val == root.borrow().val {
                    left_cur = left_max + 1;
                }
            }

            if let Some(right_root) = &root.borrow().right {
                let right_root = right_root.borrow();
                if right_root.val == root.borrow().val {
                    right_cur = right_max + 1;
                }
            }

            *cur = *cur.max(&mut (left_cur + right_cur));
            left_cur.max(right_cur)
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0687_example_1() {
        let root = tree![5, 4, 5, 1, 1, 5];
        let result = 2;

        assert_eq!(Solution::longest_univalue_path(root), result);
    }

    #[test]
    fn test_0687_example_2() {
        let root = tree![1, 4, 5, 4, 4, 5];
        let result = 2;

        assert_eq!(Solution::longest_univalue_path(root), result);
    }
}

/**
 * [0513] Find Bottom Left Tree Value
 *
 * Given the root of a binary tree, return the leftmost value in the last row of the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/14/tree1.jpg" style="width: 302px; height: 182px;" />
 * Input: root = [2,1,3]
 * Output: 1
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/14/tree2.jpg" style="width: 432px; height: 421px;" />
 * Input: root = [1,2,3,4,null,5,6,null,null,7]
 * Output: 7
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-2^31 <= Node.val <= 2^31 - 1
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/find-bottom-left-tree-value/
// discuss: https://leetcode.com/problems/find-bottom-left-tree-value/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (max, result) = (&mut 0, &mut 0);
        Self::dfs_helper(root.clone(), 1, max, result);
        *result
    }

    fn dfs_helper(root: Option<Rc<RefCell<TreeNode>>>, height: i32, max: &mut i32, res: &mut i32) {
        match root {
            Some(root) => {
                if height > *max {
                    *max = height;
                    *res = root.borrow().val;
                }
                Self::dfs_helper(root.borrow().left.clone(), height + 1, max, res);
                Self::dfs_helper(root.borrow().right.clone(), height + 1, max, res);
            }
            _ => (),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0513_example_1() {
        let root = tree![2, 1, 3];
        let result = 1;

        assert_eq!(Solution::find_bottom_left_value(root), result);
    }

    #[test]
    fn test_0513_example_2() {
        let root = tree![1, 2, 3, 4, null, 5, 6, null, null, 7];
        let result = 7;

        assert_eq!(Solution::find_bottom_left_value(root), result);
    }
}

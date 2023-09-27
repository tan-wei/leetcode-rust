/**
 * [1325] Delete Leaves With a Given Value
 *
 * Given a binary tree root and an integer target, delete all the leaf nodes with value target.
 * Note that once you delete a leaf node with value target, if its parent node becomes a leaf node and has the value target, it should also be deleted (you need to continue doing that until you cannot).
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/09/sample_1_1684.png" style="width: 500px; height: 112px;" />
 *
 * Input: root = [1,2,3,2,null,2,4], target = 2
 * Output: [1,null,3,null,4]
 * Explanation: Leaf nodes in green with value (target = 2) are removed (Picture in left).
 * After removing, new nodes become leaf nodes with value (target = 2) (Picture in center).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/09/sample_2_1684.png" style="width: 400px; height: 154px;" />
 *
 * Input: root = [1,3,3,3,2], target = 3
 * Output: [1,3,null,null,2]
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/15/sample_3_1684.png" style="width: 500px; height: 166px;" />
 *
 * Input: root = [1,2,null,2,null,2], target = 2
 * Output: [1]
 * Explanation: Leaf nodes in green with value (target = 2) are removed at each step.
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 3000].
 * 	1 <= Node.val, target <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/delete-leaves-with-a-given-value/
// discuss: https://leetcode.com/problems/delete-leaves-with-a-given-value/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut is_target = true;
        if let Some(r) = root.clone() {
            let mut in_root = r.borrow_mut();
            in_root.left = Self::remove_leaf_nodes(in_root.left.clone(), target);
            in_root.right = Self::remove_leaf_nodes(in_root.right.clone(), target);
            is_target = in_root.left.is_none() && in_root.right.is_none() && in_root.val == target;
        }

        if is_target {
            None
        } else {
            root
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1325_example_1() {
        let root = tree![1, 2, 3, 2, null, 2, 4];
        let target = 2;

        let result = tree![1, null, 3, null, 4];

        assert_eq!(Solution::remove_leaf_nodes(root, target), result);
    }

    #[test]
    fn test_1325_example_2() {
        let root = tree![1, 3, 3, 3, 2];
        let target = 3;

        let result = tree![1, 3, null, null, 2];

        assert_eq!(Solution::remove_leaf_nodes(root, target), result);
    }

    #[test]
    fn test_1325_example_3() {
        let root = tree![1, 2, null, 2, null, 2];
        let target = 2;

        let result = tree![1];

        assert_eq!(Solution::remove_leaf_nodes(root, target), result);
    }
}

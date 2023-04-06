/**
 * [1026] Maximum Difference Between Node and Ancestor
 *
 * Given the root of a binary tree, find the maximum value v for which there exist different nodes a and b where v = |a.val - b.val| and a is an ancestor of b.
 * A node a is an ancestor of b if either: any child of a is equal to b or any child of a is an ancestor of b.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/09/tmp-tree.jpg" style="width: 400px; height: 390px;" />
 * Input: root = [8,3,10,1,6,null,14,null,null,4,7,13]
 * Output: 7
 * Explanation: We have various ancestor-node differences, some of which are given below :
 * |8 - 3| = 5
 * |3 - 7| = 4
 * |8 - 1| = 7
 * |10 - 13| = 3
 * Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/09/tmp-tree-1.jpg" style="width: 250px; height: 349px;" />
 * Input: root = [1,null,2,null,0,3]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 5000].
 * 	0 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
// discuss: https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match &root {
            Some(node) => {
                let node = node.borrow();
                Self::dfs_helper(&root, node.val, node.val)
            }
            None => 0,
        }
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, cur_min: i32, cur_max: i32) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let cur_min = std::cmp::min(cur_min, node.val);
            let cur_max = std::cmp::max(cur_max, node.val);
            let left = Self::dfs_helper(&node.left, cur_min, cur_max);
            let right = Self::dfs_helper(&node.right, cur_min, cur_max);
            return std::cmp::max(left, right);
        }
        cur_max - cur_min
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1026_example_1() {
        let root = tree![8, 3, 10, 1, 6, null, 14, null, null, 4, 7, 13];
        let result = 7;

        assert_eq!(Solution::max_ancestor_diff(root), result);
    }

    #[test]
    fn test_1026_example_2() {
        let root = tree![1, null, 2, null, 0, 3];
        let result = 3;

        assert_eq!(Solution::max_ancestor_diff(root), result);
    }
}

/**
 * [1080] Insufficient Nodes in Root to Leaf Paths
 *
 * Given the root of a binary tree and an integer limit, delete all insufficient nodes in the tree simultaneously, and return the root of the resulting binary tree.
 * A node is insufficient if every root to leaf path intersecting this node has a sum strictly less than limit.
 * A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/06/05/insufficient-11.png" style="width: 500px; height: 207px;" />
 * Input: root = [1,2,3,4,-99,-99,7,8,9,-99,-99,12,13,-99,14], limit = 1
 * Output: [1,2,3,4,null,null,7,8,9,null,14]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/06/05/insufficient-3.png" style="width: 400px; height: 274px;" />
 * Input: root = [5,4,8,11,null,17,4,7,1,null,null,5,3], limit = 22
 * Output: [5,4,8,11,null,17,4,7,null,null,null,5]
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/06/11/screen-shot-2019-06-11-at-83301-pm.png" style="width: 250px; height: 199px;" />
 * Input: root = [1,2,-3,-5,null,4,null], limit = -1
 * Output: [1,null,-3,4]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 5000].
 * 	-10^5 <= Node.val <= 10^5
 * 	-10^9 <= limit <= 10^9
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/insufficient-nodes-in-root-to-leaf-paths/
// discuss: https://leetcode.com/problems/insufficient-nodes-in-root-to-leaf-paths/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::dfs_helper(&root, limit, 0)
    }

    pub fn dfs_helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
        prev: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(node_) => {
                let mut inner = node_.borrow_mut();
                let curr = prev + inner.val;
                if inner.left.is_none() && inner.right.is_none() {
                    if curr < limit {
                        None
                    } else {
                        Some(node_.clone())
                    }
                } else {
                    inner.left = Solution::dfs_helper(&inner.left, limit, curr);
                    inner.right = Solution::dfs_helper(&inner.right, limit, curr);
                    if inner.left.is_none() && inner.right.is_none() {
                        None
                    } else {
                        Some(node_.clone())
                    }
                }
            }
            None => None,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1080_example_1() {
        let root = tree![1, 2, 3, 4, -99, -99, 7, 8, 9, -99, -99, 12, 13, -99, 14];
        let limit = 1;
        let result = tree![1, 2, 3, 4, null, null, 7, 8, 9, null, 14];

        assert_eq!(Solution::sufficient_subset(root, limit), result);
    }

    #[test]
    fn test_1080_example_2() {
        let root = tree![5, 4, 8, 11, null, 17, 4, 7, 1, null, null, 5, 3];
        let limit = 22;
        let result = tree![5, 4, 8, 11, null, 17, 4, 7, null, null, null, 5];

        assert_eq!(Solution::sufficient_subset(root, limit), result);
    }

    #[test]
    fn test_1080_example_3() {
        let root = tree![1, 2, -3, -5, null, 4, null];
        let limit = -1;
        let result = tree![1, null, -3, 4];

        assert_eq!(Solution::sufficient_subset(root, limit), result);
    }
}

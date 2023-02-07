/**
 * [0968] Binary Tree Cameras
 *
 * You are given the root of a binary tree. We install cameras on the tree nodes where each camera at a node can monitor its parent, itself, and its immediate children.
 * Return the minimum number of cameras needed to monitor all nodes of the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_01.png" style="width: 138px; height: 163px;" />
 * Input: root = [0,0,null,0,0]
 * Output: 1
 * Explanation: One camera is enough to monitor all nodes if placed as shown.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_02.png" style="width: 139px; height: 312px;" />
 * Input: root = [0,0,null,0,null,0,null,null,0]
 * Output: 2
 * Explanation: At least two cameras are needed to monitor all nodes of the tree. The above image shows one of the valid configurations of camera placement.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	Node.val == 0
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-cameras/
// discuss: https://leetcode.com/problems/binary-tree-cameras/discuss/?currentPage=1&orderBy=most_votes&query=

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

enum Status {
    Camera,
    Covered,
    NotCovered,
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut camera_count = 0;

        if let Status::NotCovered = Self::dfs_helper(&root, &mut camera_count) {
            camera_count += 1;
        }

        camera_count
    }

    fn dfs_helper(node: &Option<Rc<RefCell<TreeNode>>>, camera_count: &mut i32) -> Status {
        match node {
            None => Status::Covered,
            Some(node) => {
                let node = node.borrow();

                let left_status = Self::dfs_helper(&node.left, camera_count);
                let right_status = Self::dfs_helper(&node.right, camera_count);

                match (left_status, right_status) {
                    (Status::Covered, Status::Covered) => Status::NotCovered,
                    (Status::Camera, Status::Camera)
                    | (Status::Camera, Status::Covered)
                    | (Status::Covered, Status::Camera) => Status::Covered,
                    (Status::NotCovered, _) | (_, Status::NotCovered) => {
                        *camera_count += 1;
                        Status::Camera
                    }
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0968_example_1() {
        let root = tree![0, 0, null, 0, 0];
        let result = 1;

        assert_eq!(Solution::min_camera_cover(root), result);
    }

    #[test]
    fn test_0968_example_2() {
        let root = tree![0, 0, null, 0, null, 0, null, null, 0];
        let result = 2;

        assert_eq!(Solution::min_camera_cover(root), result);
    }
}

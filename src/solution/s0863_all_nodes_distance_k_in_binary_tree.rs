/**
 * [0863] All Nodes Distance K in Binary Tree
 *
 * Given the root of a binary tree, the value of a target node target, and an integer k, return an array of the values of all nodes that have a distance k from the target node.
 * You can return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/06/28/sketch0.png" style="width: 500px; height: 429px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], target = 5, k = 2
 * Output: [7,4,1]
 * Explanation: The nodes that are a distance 2 from the target node (with value 5) have values 7, 4, and 1.
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1], target = 1, k = 3
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 500].
 * 	0 <= Node.val <= 500
 * 	All the values Node.val are unique.
 * 	target is the value of one of the nodes in the tree.
 * 	0 <= k <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/
// discuss: https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/solutions/2704306/rust-yars-yet-another-recursive-solution/
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut result = Vec::new();
        Self::find_target_dist(&root, &target, k, &mut result);
        result
    }

    fn find_target_dist(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target: &Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        result: &mut Vec<i32>,
    ) -> Option<i32> {
        if node == target {
            Self::add_kth(node, k, result);
            return Some(0);
        }

        let b = node.as_ref()?.borrow();
        if let Some(dist) = Self::find_target_dist(&b.left, target, k, result) {
            if dist + 1 == k {
                result.push(b.val);
            } else if dist + 1 < k {
                Self::add_kth(&b.right, k - dist - 2, result);
            }
            return Some(dist + 1);
        }
        if let Some(dist) = Self::find_target_dist(&b.right, target, k, result) {
            if dist + 1 == k {
                result.push(b.val);
            } else if dist + 1 < k {
                Self::add_kth(&b.left, k - dist - 2, result);
            }
            return Some(dist + 1);
        }
        None
    }

    fn add_kth(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, result: &mut Vec<i32>) {
        if let Some(n) = node.as_ref() {
            let b = n.borrow();
            if k == 0 {
                result.push(b.val);
            } else {
                Self::add_kth(&b.left, k - 1, result);
                Self::add_kth(&b.right, k - 1, result);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0863_example_1() {
        let root = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let target = tree![5];
        let k = 2;
        let result = vec![7, 4, 1];

        assert_eq!(Solution::distance_k(root, target, k), result);
    }

    #[test]
    fn test_0863_example_2() {
        let root = tree![1];
        let target = tree![1];
        let k = 3;
        let result: Vec<i32> = vec![];

        assert_eq!(Solution::distance_k(root, target, k), result);
    }
}

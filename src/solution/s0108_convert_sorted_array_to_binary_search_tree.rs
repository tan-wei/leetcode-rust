/**
 * [108] Convert Sorted Array to Binary Search Tree
 *
 * Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.
 * A height-balanced binary tree is a binary tree in which the depth of the two subtrees of every node never differs by more than one.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree1.jpg" style="width: 302px; height: 222px;" />
 * Input: nums = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: [0,-10,5,null,-3,null,9] is also accepted:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree2.jpg" style="width: 302px; height: 222px;" />
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/18/btree.jpg" style="width: 342px; height: 142px;" />
 * Input: nums = [1,3]
 * Output: [3,1]
 * Explanation: [1,3] and [3,1] are both a height-balanced BSTs.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	nums is sorted in a strictly increasing order.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let mid = nums.len() / 2;
        let mut result = Some(Rc::new(RefCell::new(TreeNode::new(nums[mid]))));
        result.as_mut().unwrap().borrow_mut().left =
            Self::sorted_array_to_bst(nums[..mid].to_vec());
        result.as_mut().unwrap().borrow_mut().right =
            Self::sorted_array_to_bst(nums[mid + 1..].to_vec());

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0108_example_1() {
        let nums = vec![-10, -3, 0, 5, 9];
        let result = tree![0, -3, 9, -10, null, 5];

        assert_eq!(Solution::sorted_array_to_bst(nums), result);
    }

    #[test]
    #[ignore]
    fn test_0108_example_2() {
        let nums = vec![1, 3];
        let result = tree![3, 1];

        assert_eq!(Solution::sorted_array_to_bst(nums), result);
    }
}

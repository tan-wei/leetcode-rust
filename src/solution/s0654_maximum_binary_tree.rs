/**
 * [0654] Maximum Binary Tree
 *
 * You are given an integer array nums with no duplicates. A maximum binary tree can be built recursively from nums using the following algorithm:
 * <ol>
 * 	Create a root node whose value is the maximum value in nums.
 * 	Recursively build the left subtree on the subarray prefix to the left of the maximum value.
 * 	Recursively build the right subtree on the subarray suffix to the right of the maximum value.
 * </ol>
 * Return the maximum binary tree built from nums.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/tree1.jpg" style="width: 302px; height: 421px;" />
 * Input: nums = [3,2,1,6,0,5]
 * Output: [6,3,5,null,2,0,null,null,1]
 * Explanation: The recursive calls are as follow:
 * - The largest value in [3,2,1,6,0,5] is 6. Left prefix is [3,2,1] and right suffix is [0,5].
 *     - The largest value in [3,2,1] is 3. Left prefix is [] and right suffix is [2,1].
 *         - Empty array, so no child.
 *         - The largest value in [2,1] is 2. Left prefix is [] and right suffix is [1].
 *             - Empty array, so no child.
 *             - Only one element, so child is a node with value 1.
 *     - The largest value in [0,5] is 5. Left prefix is [0] and right suffix is [].
 *         - Only one element, so child is a node with value 0.
 *         - Empty array, so no child.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/12/24/tree2.jpg" style="width: 182px; height: 301px;" />
 * Input: nums = [3,2,1]
 * Output: [3,null,2,null,1]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	0 <= nums[i] <= 1000
 * 	All integers in nums are unique.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/maximum-binary-tree/
// discuss: https://leetcode.com/problems/maximum-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match nums.len() {
            0 => None,
            1 => Some(Rc::new(RefCell::new(TreeNode {
                val: nums[0],
                left: None,
                right: None,
            }))),
            _ => {
                let mut index = 0;
                let mut max = -1;
                for (i, val) in nums.iter().enumerate() {
                    if *val > max {
                        max = *val;
                        index = i;
                    }
                }
                let left = Solution::construct_maximum_binary_tree(nums[..index].to_vec());
                let right = Solution::construct_maximum_binary_tree(nums[index + 1..].to_vec());

                Some(Rc::new(RefCell::new(TreeNode {
                    val: max,
                    left,
                    right,
                })))
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0654_example_1() {
        let nums = vec![3, 2, 1, 6, 0, 5];
        let result = tree![6, 3, 5, null, 2, 0, null, null, 1];

        assert_eq!(Solution::construct_maximum_binary_tree(nums), result);
    }

    #[test]
    fn test_0654_example_2() {
        let nums = vec![3, 2, 1];
        let result = tree![3, null, 2, null, 1];

        assert_eq!(Solution::construct_maximum_binary_tree(nums), result);
    }
}

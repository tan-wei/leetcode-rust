/**
 * [0437] Path Sum III
 *
 * Given the root of a binary tree and an integer targetSum, return the number of paths where the sum of the values along the path equals targetSum.
 * The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/pathsum3-1-tree.jpg" style="width: 450px; height: 386px;" />
 * Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
 * Output: 3
 * Explanation: The paths that sum to 8 are shown.
 *
 * Example 2:
 *
 * Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 1000].
 * 	-10^9 <= Node.val <= 10^9
 * 	-1000 <= targetSum <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/path-sum-iii/
// discuss: https://leetcode.com/problems/path-sum-iii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let left = Self::path_sum(root.as_ref().borrow().left.clone(), target_sum);
                let right = Self::path_sum(root.as_ref().borrow().right.clone(), target_sum);
                let with_root = Self::with_root_path_sum(Some(root), target_sum);

                left + right + with_root
            }
        }
    }

    fn with_root_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let mut result = 0;
                let root_val = root.as_ref().borrow().val;
                if root_val == target_sum {
                    result += 1;
                }

                result += Self::with_root_path_sum(
                    root.as_ref().borrow().left.clone(),
                    target_sum - root_val,
                ) + Self::with_root_path_sum(
                    root.as_ref().borrow().right.clone(),
                    target_sum - root_val,
                );

                result
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0437_example_1() {
        let root = tree![10, 5, -3, 3, 2, null, 11, 3, -2, null, 1];
        let target_sum = 8;
        let result = 3;

        assert_eq!(Solution::path_sum(root, target_sum), result);
    }

    #[test]
    fn test_0437_example_2() {
        let root = tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1];
        let target_sum = 22;
        let result = 3;

        assert_eq!(Solution::path_sum(root, target_sum), result);
    }
}

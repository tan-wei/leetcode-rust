/**
 * [2236] Root Equals Sum of Children
 *
 * You are given the root of a binary tree that consists of exactly 3 nodes: the root, its left child, and its right child.
 * Return true if the value of the root is equal to the sum of the values of its two children, or false otherwise.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/04/08/graph3drawio.png" style="width: 281px; height: 199px;" />
 * Input: root = [10,4,6]
 * Output: true
 * Explanation: The values of the root, its left child, and its right child are 10, 4, and 6, respectively.
 * 10 is equal to 4 + 6, so we return true.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/04/08/graph3drawio-1.png" style="width: 281px; height: 199px;" />
 * Input: root = [5,3,1]
 * Output: false
 * Explanation: The values of the root, its left child, and its right child are 5, 3, and 1, respectively.
 * 5 is not equal to 3 + 1, so we return false.
 *
 *  
 * Constraints:
 *
 * 	The tree consists only of the root, its left child, and its right child.
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/root-equals-sum-of-children/
// discuss: https://leetcode.com/problems/root-equals-sum-of-children/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.and_then(|root| {
            let root = root.borrow();
            let left = root.left.as_ref();
            let right = root.right.as_ref();
            left.zip(right)
                .map(|(left, right)| left.borrow().val + right.borrow().val == root.val)
        })
        .unwrap_or(false)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2236_example_1() {
        let root = tree![10, 4, 6];

        let result = true;

        assert_eq!(Solution::check_tree(root), result);
    }

    #[test]
    fn test_2236_example_2() {
        let root = tree![5, 3, 1];

        let result = false;

        assert_eq!(Solution::check_tree(root), result);
    }
}

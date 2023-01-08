/**
 * [0938] Range Sum of BST
 *
 * Given the root node of a binary search tree and two integers low and high, return the sum of values of all nodes with a value in the inclusive range [low, high].
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/bst1.jpg" style="width: 400px; height: 222px;" />
 * Input: root = [10,5,15,3,7,null,18], low = 7, high = 15
 * Output: 32
 * Explanation: Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/bst2.jpg" style="width: 400px; height: 335px;" />
 * Input: root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
 * Output: 23
 * Explanation: Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 2 * 10^4].
 * 	1 <= Node.val <= 10^5
 * 	1 <= low <= high <= 10^5
 * 	All Node.val are unique.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/range-sum-of-bst/
// discuss: https://leetcode.com/problems/range-sum-of-bst/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let val = root.borrow().val.clone();
                if val > high {
                    Self::range_sum_bst(root.borrow().left.clone(), low, high)
                } else if val < low {
                    Self::range_sum_bst(root.borrow().right.clone(), low, high)
                } else {
                    val + Self::range_sum_bst(root.borrow().left.clone(), low, val)
                        + Self::range_sum_bst(root.borrow().right.clone(), val, high)
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
    fn test_0938_example_1() {
        let root = tree![10, 5, 15, 3, 7, null, 18];
        let low = 7;
        let high = 15;
        let result = 32;

        assert_eq!(Solution::range_sum_bst(root, low, high), result);
    }

    #[test]
    fn test_0938_example_2() {
        let root = tree![10, 5, 15, 3, 7, 13, 18, 1, null, 6];
        let low = 6;
        let high = 10;
        let result = 23;

        assert_eq!(Solution::range_sum_bst(root, low, high), result);
    }
}

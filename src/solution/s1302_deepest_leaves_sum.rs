/**
 * [1302] Deepest Leaves Sum
 *
 * Given the root of a binary tree, return the sum of values of its deepest leaves.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/07/31/1483_ex1.png" style="width: 273px; height: 265px;" />
 * Input: root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
 * Output: 15
 *
 * Example 2:
 *
 * Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
 * Output: 19
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	1 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/deepest-leaves-sum/
// discuss: https://leetcode.com/problems/deepest-leaves-sum/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vd = std::collections::VecDeque::new();
        if let Some(r) = root {
            vd.push_back(r);
        }
        let mut result = 0;
        while !vd.is_empty() {
            result = 0;
            for _ in 0..vd.len() {
                if let Some(n) = vd.pop_front() {
                    result += n.borrow().val;
                    if let Some(l) = n.borrow_mut().left.take() {
                        vd.push_back(l);
                    }
                    if let Some(r) = n.borrow_mut().right.take() {
                        vd.push_back(r);
                    }
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1302_example_1() {
        let root = tree![1, 2, 3, 4, 5, null, 6, 7, null, null, null, null, 8];
        let result = 15;

        assert_eq!(Solution::deepest_leaves_sum(root), result);
    }

    #[test]
    fn test_1302_example_2() {
        let root = tree![6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5];
        let result = 19;

        assert_eq!(Solution::deepest_leaves_sum(root), result);
    }
}

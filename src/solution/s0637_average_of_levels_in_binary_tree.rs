/**
 * [0637] Average of Levels in Binary Tree
 *
 * Given the root of a binary tree, return the average value of the nodes on each level in the form of an array. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/avg1-tree.jpg" style="width: 277px; height: 302px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [3.00000,14.50000,11.00000]
 * Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, and on level 2 is 11.
 * Hence return [3, 14.5, 11].
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/avg2-tree.jpg" style="width: 292px; height: 302px;" />
 * Input: root = [3,9,20,15,7]
 * Output: [3.00000,14.50000,11.00000]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-2^31 <= Node.val <= 2^31 - 1
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/average-of-levels-in-binary-tree/
// discuss: https://leetcode.com/problems/average-of-levels-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut vd = std::collections::VecDeque::new();

        if let Some(r) = root {
            vd.push_back(r);
        }

        let mut result = Vec::new();
        while !vd.is_empty() {
            let len = vd.len();
            let mut sum = 0;
            for _ in 0..vd.len() {
                if let Some(node) = vd.pop_front() {
                    sum += i64::from(node.borrow().val);
                    if let Some(n) = node.borrow_mut().left.take() {
                        vd.push_back(n);
                    }
                    if let Some(n) = node.borrow_mut().right.take() {
                        vd.push_back(n);
                    }
                }
            }
            result.push(sum as f64 / len as f64);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0637_example_1() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        let result = vec![3.00000, 14.50000, 11.00000];

        assert_eq!(Solution::average_of_levels(root), result);
    }
}

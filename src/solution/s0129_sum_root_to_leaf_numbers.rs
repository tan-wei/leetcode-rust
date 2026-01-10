/**
 * [129] Sum Root to Leaf Numbers
 *
 * You are given the root of a binary tree containing digits from 0 to 9 only.
 * Each root-to-leaf path in the tree represents a number.
 *
 * 	For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
 *
 * Return the total sum of all root-to-leaf numbers. Test cases are generated so that the answer will fit in a 32-bit integer.
 * A leaf node is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/num1tree.jpg" style="width: 212px; height: 182px;" />
 * Input: root = [1,2,3]
 * Output: 25
 * Explanation:
 * The root-to-leaf path 1->2 represents the number 12.
 * The root-to-leaf path 1->3 represents the number 13.
 * Therefore, sum = 12 + 13 = 25.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/num2tree.jpg" style="width: 292px; height: 302px;" />
 * Input: root = [4,9,0,5,1]
 * Output: 1026
 * Explanation:
 * The root-to-leaf path 4->9->5 represents the number 495.
 * The root-to-leaf path 4->9->1 represents the number 491.
 * The root-to-leaf path 4->0 represents the number 40.
 * Therefore, sum = 495 + 491 + 40 = 1026.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	0 <= Node.val <= 9
 * 	The depth of the tree will not exceed 10.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/sum-root-to-leaf-numbers/
// discuss: https://leetcode.com/problems/sum-root-to-leaf-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
            match node {
                Some(node) => {
                    if node.borrow().left.is_none() && node.borrow().right.is_none() {
                        sum * 10 + node.borrow().val
                    } else {
                        helper(node.borrow().left.clone(), sum * 10 + node.borrow().val)
                            + helper(node.borrow().right.clone(), sum * 10 + node.borrow().val)
                    }
                }
                None => 0,
            }
        }

        helper(root, 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0129_example_1() {
        let root = tree![1, 2, 3];
        let result = 25;

        assert_eq!(Solution::sum_numbers(root), result);
    }

    #[test]
    fn test_0129_example_2() {
        let root = tree![4, 9, 0, 5, 1];
        let result = 1026;

        assert_eq!(Solution::sum_numbers(root), result);
    }
}

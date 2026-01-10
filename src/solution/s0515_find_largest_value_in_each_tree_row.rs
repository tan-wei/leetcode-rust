/**
 * [0515] Find Largest Value in Each Tree Row
 *
 * Given the root of a binary tree, return an array of the largest value in each row of the tree (0-indexed).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/21/largest_e1.jpg" style="width: 300px; height: 172px;" />
 * Input: root = [1,3,2,5,3,null,9]
 * Output: [1,3,9]
 *
 * Example 2:
 *
 * Input: root = [1,2,3]
 * Output: [1,3]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree will be in the range [0, 10^4].
 * 	-2^31 <= Node.val <= 2^31 - 1
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/find-largest-value-in-each-tree-row/
// discuss: https://leetcode.com/problems/find-largest-value-in-each-tree-row/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root.is_none() {
            true => vec![],
            false => {
                let mut result = vec![];
                if root.is_none() {
                    return result;
                }
                let mut stack = vec![root];
                while !stack.is_empty() {
                    let mut max = i32::MIN;
                    let mut new_stack = vec![];
                    for node in stack {
                        if let Some(n) = node {
                            max = n.borrow().val.max(max);
                            if n.borrow().left.is_some() {
                                new_stack.push(n.borrow().left.clone());
                            }
                            if n.borrow().right.is_some() {
                                new_stack.push(n.borrow().right.clone());
                            }
                        }
                    }
                    stack = new_stack;
                    result.push(max);
                }
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
    fn test_0515_example_1() {
        let root = tree![1, 3, 2, 5, 3, null, 9];
        let result = vec![1, 3, 9];

        assert_eq!(Solution::largest_values(root), result);
    }

    #[test]
    fn test_0515_example_2() {
        let root = tree![1, 2, 3];
        let result = vec![1, 3];

        assert_eq!(Solution::largest_values(root), result);
    }
}

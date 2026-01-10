/**
 * [0671] Second Minimum Node In a Binary Tree
 *
 * Given a non-empty special binary tree consisting of nodes with the non-negative value, where each node in this tree has exactly two or zero sub-node. If the node has two sub-nodes, then this node's value is the smaller value among its two sub-nodes. More formally, the property root.val = min(root.left.val, root.right.val) always holds.
 * Given such a binary tree, you need to output the second minimum value in the set made of all the nodes' value in the whole tree.
 * If no such second minimum value exists, output -1 instead.
 *  
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/smbt1.jpg" style="width: 431px; height: 302px;" />
 * Input: root = [2,2,5,null,null,5,7]
 * Output: 5
 * Explanation: The smallest value is 2, the second smallest value is 5.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/15/smbt2.jpg" style="width: 321px; height: 182px;" />
 * Input: root = [2,2,2]
 * Output: -1
 * Explanation: The smallest value is 2, but there isn't any second smallest value.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 25].
 * 	1 <= Node.val <= 2^31 - 1
 * 	root.val == min(root.left.val, root.right.val) for each internal node of the tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/
// discuss: https://leetcode.com/problems/second-minimum-node-in-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => -1,
            Some(root) => match ((root.borrow().left.clone(), root.borrow().right.clone())) {
                (None, None) => -1,
                (left @ _, right @ _) => {
                    // Note the problem is given this:
                    // "this tree has exactly two or zero sub-node"
                    let mut left_val = left.as_ref().unwrap().borrow().val;
                    let mut right_val = right.as_ref().unwrap().borrow().val;

                    if (left_val == root.borrow().val) {
                        left_val = Self::find_second_minimum_value(left);
                    }

                    if (right_val == root.borrow().val) {
                        right_val = Self::find_second_minimum_value(right);
                    }

                    match (left_val, right_val) {
                        (-1, -1) => -1,
                        (_, -1) => left_val,
                        (-1, _) => right_val,
                        (_, _) => std::cmp::min(left_val, right_val),
                    }
                }
            },
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0671_example_1() {
        let root = tree![2, 2, 5, null, null, 5, 7];
        let result = 5;

        assert_eq!(Solution::find_second_minimum_value(root), result);
    }

    #[test]
    fn test_0671_example_2() {
        let root = tree![2, 2, 2];
        let result = -1;

        assert_eq!(Solution::find_second_minimum_value(root), result);
    }
}

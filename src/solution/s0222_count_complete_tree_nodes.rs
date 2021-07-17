/**
 * [222] Count Complete Tree Nodes
 *
 * Given the root of a complete binary tree, return the number of the nodes in the tree.
 * According to <a href="http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees" target="_blank">Wikipedia</a>, every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between 1 and 2^h nodes inclusive at the last level h.
 * Design an algorithm that runs in less than <code data-stringify-type="code">O(n) time complexity.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/14/complete.jpg" style="width: 372px; height: 302px;" />
 * Input: root = [1,2,3,4,5,6]
 * Output: 6
 *
 * Example 2:
 *
 * Input: root = []
 * Output: 0
 *
 * Example 3:
 *
 * Input: root = [1]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 5 * 10^4].
 * 	0 <= Node.val <= 5 * 10^4
 * 	The tree is guaranteed to be complete.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/count-complete-tree-nodes/
// discuss: https://leetcode.com/problems/count-complete-tree-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(n: Option<Rc<RefCell<TreeNode>>>, c: &mut i32) {
            match n {
                Some(n) => {
                    *c += 1;
                    helper(n.borrow().left.clone(), c);
                    helper(n.borrow().right.clone(), c);
                }
                _ => (),
            }
        }
        let result = &mut 0i32;
        helper(root.clone(), result);
        *result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0222_example_1() {
        let root = tree![1, 2, 3, 4, 5, 6];
        let result = 6;

        assert_eq!(Solution::count_nodes(root), result);
    }

    #[test]
    fn test_0222_example_2() {
        let root = tree![];
        let result = 0;

        assert_eq!(Solution::count_nodes(root), result);
    }

    #[test]
    fn test_0222_example_3() {
        let root = tree![1];
        let result = 1;

        assert_eq!(Solution::count_nodes(root), result);
    }
}

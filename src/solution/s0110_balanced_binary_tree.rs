/**
 * [110] Balanced Binary Tree
 *
 * Given a binary tree, determine if it is height-balanced.
 * For this problem, a height-balanced binary tree is defined as:
 * <blockquote>
 * a binary tree in which the left and right subtrees of every node differ in height by no more than 1.
 * </blockquote>
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_1.jpg" style="width: 342px; height: 221px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_2.jpg" style="width: 452px; height: 301px;" />
 * Input: root = [1,2,2,3,3,null,null,4,4]
 * Output: false
 *
 * Example 3:
 *
 * Input: root = []
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-10^4 <= Node.val <= 10^4
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/balanced-binary-tree/
// discuss: https://leetcode.com/problems/balanced-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
            let root = match root {
                Some(a) => a,
                None => return (true, 0),
            };

            let (l_valid, l_height) = helper(root.borrow().left.clone());
            let (r_valid, r_height) = helper(root.borrow().right.clone());
            let height = l_height.max(r_height) + 1;
            let valid = l_valid && r_valid && (l_height - r_height).abs() <= 1;
            (valid, height)
        }

        helper(root).0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0110_example_1() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        let result = true;

        assert_eq!(Solution::is_balanced(root), result);
    }

    #[test]
    fn test_0110_example_2() {
        let root = tree![1, 2, 2, 3, 3, null, null, 4, 4];
        let result = false;

        assert_eq!(Solution::is_balanced(root), result);
    }

    #[test]
    fn test_0110_example_3() {
        let root = tree![];
        let result = true;

        assert_eq!(Solution::is_balanced(root), result);
    }
}

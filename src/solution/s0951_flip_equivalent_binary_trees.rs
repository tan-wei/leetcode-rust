/**
 * [0951] Flip Equivalent Binary Trees
 *
 * For a binary tree T, we can define a flip operation as follows: choose any node, and swap the left and right child subtrees.
 * A binary tree X is flip equivalent to a binary tree Y if and only if we can make X equal to Y after some number of flip operations.
 * Given the roots of two binary trees root1 and root2, return true if the two trees are flip equivalent or false otherwise.
 *  
 * Example 1:
 * <img alt="Flipped Trees Diagram" src="https://assets.leetcode.com/uploads/2018/11/29/tree_ex.png" style="width: 500px; height: 220px;" />
 * Input: root1 = [1,2,3,4,5,6,null,null,null,7,8], root2 = [1,3,2,null,6,4,5,null,null,null,null,8,7]
 * Output: true
 * Explanation: We flipped at nodes with values 1, 3, and 5.
 *
 * Example 2:
 *
 * Input: root1 = [], root2 = []
 * Output: true
 *
 * Example 3:
 *
 * Input: root1 = [], root2 = [1]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in each tree is in the range [0, 100].
 * 	Each tree will have unique node values in the range [0, 99].
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/flip-equivalent-binary-trees/
// discuss: https://leetcode.com/problems/flip-equivalent-binary-trees/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (None, _) | (_, None) => false,
            (Some(root1), Some(root2)) => {
                if root1.as_ref().borrow().val != root2.as_ref().borrow().val {
                    false
                } else {
                    (Self::flip_equiv(
                        root1.as_ref().borrow().left.clone(),
                        root2.as_ref().borrow().left.clone(),
                    ) && Self::flip_equiv(
                        root1.as_ref().borrow().right.clone(),
                        root2.as_ref().borrow().right.clone(),
                    )) || (Self::flip_equiv(
                        root1.as_ref().borrow().right.clone(),
                        root2.as_ref().borrow().left.clone(),
                    ) && Self::flip_equiv(
                        root1.as_ref().borrow().left.clone(),
                        root2.as_ref().borrow().right.clone(),
                    ))
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
    fn test_0951_example_1() {
        let root1 = tree![1, 2, 3, 4, 5, 6, null, null, null, 7, 8];
        let root2 = tree![1, 3, 2, null, 6, 4, 5, null, null, null, null, 8, 7];
        let result = true;

        assert_eq!(Solution::flip_equiv(root1, root2), result);
    }

    #[test]
    fn test_0951_example_2() {
        let root1 = tree![];
        let root2 = tree![];
        let result = true;

        assert_eq!(Solution::flip_equiv(root1, root2), result);
    }

    #[test]
    fn test_0951_example_3() {
        let root1 = tree![];
        let root2 = tree![1];
        let result = false;

        assert_eq!(Solution::flip_equiv(root1, root2), result);
    }
}

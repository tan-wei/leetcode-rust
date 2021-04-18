/**
 * [101] Symmetric Tree
 *
 * Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree1.jpg" style="width: 354px; height: 291px;" />
 * Input: root = [1,2,2,3,4,4,3]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/symtree2.jpg" style="width: 308px; height: 258px;" />
 * Input: root = [1,2,2,null,3,null,3]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	-100 <= Node.val <= 100
 *
 *  
 * Follow up: Could you solve it both recursively and iteratively?
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/symmetric-tree/
// discuss: https://leetcode.com/problems/symmetric-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn helper(
            left: &Option<Rc<RefCell<TreeNode>>>,
            right: &Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (left, right) {
                (None, None) => true,
                (None, _) => false,
                (_, None) => false,
                (Some(left), Some(right)) if left.borrow().val != right.borrow().val => false,
                (Some(left), Some(right)) => {
                    helper(&left.borrow().left, &right.borrow().right)
                        && helper(&right.borrow().left, &left.borrow().right)
                }
            }
        }

        match root {
            None => true,
            Some(root) => helper(&root.as_ref().borrow().left, &root.as_ref().borrow().right),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0101_example_1() {
        let root = tree![1, 2, 2, 3, 4, 4, 3];
        let result = true;

        assert_eq!(Solution::is_symmetric(root), result);
    }

    #[test]
    fn test_0101_example_2() {
        let root = tree![1, 2, 2, null, 3, null, 3];
        let result = false;

        assert_eq!(Solution::is_symmetric(root), result);
    }
}

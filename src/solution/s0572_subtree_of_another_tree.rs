/**
 * [0572] Subtree of Another Tree
 *
 * Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.
 * A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/subtree1-tree.jpg" style="width: 532px; height: 400px;" />
 * Input: root = [3,4,5,1,2], subRoot = [4,1,2]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/subtree2-tree.jpg" style="width: 502px; height: 458px;" />
 * Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the root tree is in the range [1, 2000].
 * 	The number of nodes in the subRoot tree is in the range [1, 1000].
 * 	-10^4 <= root.val <= 10^4
 * 	-10^4 <= subRoot.val <= 10^4
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/subtree-of-another-tree/
// discuss: https://leetcode.com/problems/subtree-of-another-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root == sub_root {
            return true;
        }

        if let Some(root) = root {
            Self::is_subtree(root.borrow().left.clone(), sub_root.clone())
                || Self::is_subtree(root.borrow().right.clone(), sub_root.clone())
        } else {
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0572_example_1() {
        let root = tree![3, 4, 5, 1, 2];
        let sub_root = tree![4, 1, 2];
        let result = true;

        assert_eq!(Solution::is_subtree(root, sub_root), result);
    }

    #[test]
    fn test_0572_example_2() {
        let root = tree![3, 4, 5, 1, 2, null, null, null, null, 0];
        let sub_root = tree![4, 1, 2];
        let result = false;

        assert_eq!(Solution::is_subtree(root, sub_root), result);
    }
}

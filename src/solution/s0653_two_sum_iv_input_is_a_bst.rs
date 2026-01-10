/**
 * [0653] Two Sum IV - Input is a BST
 *
 * Given the root of a Binary Search Tree and a target number k, return true if there exist two elements in the BST such that their sum is equal to the given target.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/21/sum_tree_1.jpg" style="width: 400px; height: 229px;" />
 * Input: root = [5,3,6,2,4,null,7], k = 9
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/21/sum_tree_2.jpg" style="width: 400px; height: 229px;" />
 * Input: root = [5,3,6,2,4,null,7], k = 28
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-10^4 <= Node.val <= 10^4
 * 	root is guaranteed to be a valid binary search tree.
 * 	-10^5 <= k <= 10^5
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/two-sum-iv-input-is-a-bst/
// discuss: https://leetcode.com/problems/two-sum-iv-input-is-a-bst/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut seen = std::collections::HashSet::new();
        Self::dfs_helper(&root, k, &mut seen)
    }

    fn dfs_helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        k: i32,
        s: &mut std::collections::HashSet<i32>,
    ) -> bool {
        if let Some(node) = root {
            let val = node.borrow().val;
            let diff = k - val;
            if s.contains(&diff) {
                return true;
            }
            s.insert(val);
            return Self::dfs_helper(&node.borrow().left, k, s)
                || Self::dfs_helper(&node.borrow().right, k, s);
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0653_example_1() {
        let root = tree![5, 3, 6, 2, 4, null, 7];
        let k = 9;
        let result = true;

        assert_eq!(Solution::find_target(root, k), result);
    }

    #[test]
    fn test_0653_example_2() {
        let root = tree![5, 3, 6, 2, 4, null, 7];
        let k = 28;
        let result = false;

        assert_eq!(Solution::find_target(root, k), result);
    }
}

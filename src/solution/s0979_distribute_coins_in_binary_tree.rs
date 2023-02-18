/**
 * [0979] Distribute Coins in Binary Tree
 *
 * You are given the root of a binary tree with n nodes where each node in the tree has node.val coins. There are n coins in total throughout the whole tree.
 * In one move, we may choose two adjacent nodes and move one coin from one node to another. A move may be from parent to child, or from child to parent.
 * Return the minimum number of moves required to make every node have exactly one coin.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/18/tree1.png" style="width: 250px; height: 236px;" />
 * Input: root = [3,0,0]
 * Output: 2
 * Explanation: From the root of the tree, we move one coin to its left child, and one coin to its right child.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/18/tree2.png" style="width: 250px; height: 236px;" />
 * Input: root = [0,3,0]
 * Output: 3
 * Explanation: From the left child of the root, we move two coins to the root [taking two moves]. Then, we move one coin from the root of the tree to the right child.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is n.
 * 	1 <= n <= 100
 * 	0 <= Node.val <= n
 * 	The sum of all Node.val is n.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/distribute-coins-in-binary-tree/
// discuss: https://leetcode.com/problems/distribute-coins-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        Self::dfs_helper(root.as_ref(), &mut result);
        result
    }

    fn dfs_helper(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut i32) -> i32 {
        if let Some(r) = node {
            let n = r.borrow();
            let l = Self::dfs_helper(n.left.as_ref(), result);
            let r = Self::dfs_helper(n.right.as_ref(), result);
            *result += l.abs() + r.abs();
            n.val + l + r - 1
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0979_example_1() {
        let root = tree![3, 0, 0];
        let result = 2;

        assert_eq!(Solution::distribute_coins(root), result);
    }

    #[test]
    fn test_0979_example_2() {
        let root = tree![0, 3, 0];
        let result = 3;

        assert_eq!(Solution::distribute_coins(root), result);
    }
}

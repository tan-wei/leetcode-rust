/**
 * [1382] Balance a Binary Search Tree
 *
 * Given the root of a binary search tree, return a balanced binary search tree with the same node values. If there is more than one answer, return any of them.
 * A binary search tree is balanced if the depth of the two subtrees of every node never differs by more than 1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/balance1-tree.jpg" style="width: 500px; height: 319px;" />
 * Input: root = [1,null,2,null,3,null,4,null,null]
 * Output: [2,1,3,null,null,null,4]
 * Explanation: This is not the only correct answer, [3,1,4,null,2] is also correct.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/balanced2-tree.jpg" style="width: 224px; height: 145px;" />
 * Input: root = [2,1,3]
 * Output: [2,1,3]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	1 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/balance-a-binary-search-tree/
// discuss: https://leetcode.com/problems/balance-a-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut order: Vec<i32> = Vec::new();
        Self::dfs_helper(&root, &mut order);
        Self::construct_bst(&order, 0, order.len())
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) {
        if let Some(root_ref) = root {
            let root_node = root_ref.borrow();
            Self::dfs_helper(&root_node.left, order);
            order.push(root_node.val);
            Self::dfs_helper(&root_node.right, order);
        }
    }

    fn construct_bst(order: &[i32], start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start < end {
            let root_index = (start + end) / 2;
            let mut root = TreeNode::new(order[root_index]);
            root.left = Self::construct_bst(order, start, root_index);
            root.right = Self::construct_bst(order, root_index + 1, end);
            Some(Rc::new(RefCell::new(root)))
        } else {
            None
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1382_example_1() {
        let root = tree![1, null, 2, null, 3, null, 4, null, null];

        let result = tree![2, 1, 3, null, null, null, 4];

        assert_eq!(Solution::balance_bst(root), result);
    }

    #[test]
    #[ignore]
    fn test_1382_example_2() {
        let root = tree![2, 1, 3];

        let result = tree![2, 1, 3];

        assert_eq!(Solution::balance_bst(root), result);
    }
}

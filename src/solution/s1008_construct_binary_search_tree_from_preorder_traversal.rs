/**
 * [1008] Construct Binary Search Tree from Preorder Traversal
 *
 * Given an array of integers preorder, which represents the preorder traversal of a BST (i.e., binary search tree), construct the tree and return its root.
 * It is guaranteed that there is always possible to find a binary search tree with the given requirements for the given test cases.
 * A binary search tree is a binary tree where for every node, any descendant of Node.left has a value strictly less than Node.val, and any descendant of Node.right has a value strictly greater than Node.val.
 * A preorder traversal of a binary tree displays the value of the node first, then traverses Node.left, then traverses Node.right.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/03/06/1266.png" style="height: 386px; width: 590px;" />
 * Input: preorder = [8,5,1,7,10,12]
 * Output: [8,5,10,1,7,null,12]
 *
 * Example 2:
 *
 * Input: preorder = [1,3]
 * Output: [1,null,3]
 *
 *  
 * Constraints:
 *
 * 	1 <= preorder.length <= 100
 * 	1 <= preorder[i] <= 1000
 * 	All the values of preorder are unique.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::dfs_helper(&preorder)
    }

    fn dfs_helper(v: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(first) = v.first() {
            let node = Rc::new(RefCell::new(TreeNode::new(*first)));
            let i = (1..v.len())
                .find(|&i| v[i] > v[0])
                .unwrap_or_else(|| v.len());
            node.borrow_mut().left = Solution::dfs_helper(&v[1..i]);
            node.borrow_mut().right = Solution::dfs_helper(&v[i..]);
            Some(node)
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
    fn test_1008_example_1() {
        let preorder = vec![8, 5, 1, 7, 10, 12];
        let result = tree![8, 5, 10, 1, 7, null, 12];

        assert_eq!(Solution::bst_from_preorder(preorder), result);
    }

    #[test]
    fn test_1008_example_2() {
        let preorder = vec![1, 3];
        let result = tree![1, null, 3];

        assert_eq!(Solution::bst_from_preorder(preorder), result);
    }
}

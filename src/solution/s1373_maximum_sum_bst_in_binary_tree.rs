/**
 * [1373] Maximum Sum BST in Binary Tree
 *
 * Given a binary tree root, return the maximum sum of all keys of any sub-tree which is also a Binary Search Tree (BST).
 * Assume a BST is defined as follows:
 *
 * 	The left subtree of a node contains only nodes with keys less than the node's key.
 * 	The right subtree of a node contains only nodes with keys greater than the node's key.
 * 	Both the left and right subtrees must also be binary search trees.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/30/sample_1_1709.png" style="width: 320px; height: 250px;" />
 *
 * Input: root = [1,4,3,2,4,2,5,null,null,null,null,null,null,4,6]
 * Output: 20
 * Explanation: Maximum sum in a valid Binary search tree is obtained in root node with key equal to 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/01/30/sample_2_1709.png" style="width: 134px; height: 180px;" />
 *
 * Input: root = [4,3,null,1,2]
 * Output: 2
 * Explanation: Maximum sum in a valid Binary search tree is obtained in a single root node with key equal to 2.
 *
 * Example 3:
 *
 * Input: root = [-4,-2,-5]
 * Output: 0
 * Explanation: All values are negatives. Return an empty BST.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 4 * 10^4].
 * 	-4 * 10^4 <= Node.val <= 4 * 10^4
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/
// discuss: https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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

// Credit: https://leetcode.com/problems/maximum-sum-bst-in-binary-tree/solutions/791596/rust-translated-28ms-10-5m-100/

#[derive(Debug, Clone)]
pub struct Status {
    is_bst: bool,
    sum: i32,
    max_left: i32,
    min_right: i32,
}

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = vec![];
        Self::dfs_helper(&root, &mut v);
        v.iter().fold(0, |acc, &x| if x > acc { x } else { acc })
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) -> Status {
        match root {
            None => Status {
                is_bst: true,
                sum: 0,
                max_left: std::i32::MIN,
                min_right: std::i32::MAX,
            },
            Some(node) => {
                let node = node.borrow();
                let left = Self::dfs_helper(&node.left, v);
                let right = Self::dfs_helper(&node.right, v);
                let val = node.val;
                let s = if val > left.max_left && val < right.min_right {
                    Status {
                        is_bst: left.is_bst && right.is_bst,
                        sum: val + left.sum + right.sum,
                        max_left: val.max(right.max_left),
                        min_right: val.min(left.min_right),
                    }
                } else {
                    Status {
                        is_bst: false,
                        sum: 0,
                        max_left: std::i32::MIN,
                        min_right: std::i32::MAX,
                    }
                };
                if s.is_bst {
                    v.push(s.sum)
                }
                s
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1373_example_1() {
        let root = tree![
            1, 4, 3, 2, 4, 2, 5, null, null, null, null, null, null, 4, 6
        ];

        let result = 20;

        assert_eq!(Solution::max_sum_bst(root), result);
    }

    #[test]
    fn test_1373_example_2() {
        let root = tree![4, 3, null, 1, 2];

        let result = 2;

        assert_eq!(Solution::max_sum_bst(root), result);
    }

    #[test]
    fn test_1373_example_3() {
        let root = tree![-4, -2, -5];

        let result = 0;

        assert_eq!(Solution::max_sum_bst(root), result);
    }
}

/**
 * [0971] Flip Binary Tree To Match Preorder Traversal
 *
 * You are given the root of a binary tree with n nodes, where each node is uniquely assigned a value from 1 to n. You are also given a sequence of n values voyage, which is the desired <a href="https://en.wikipedia.org/wiki/Tree_traversal#Pre-order" target="_blank">pre-order traversal</a> of the binary tree.
 * Any node in the binary tree can be flipped by swapping its left and right subtrees. For example, flipping node 1 will have the following effect:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/15/fliptree.jpg" style="width: 400px; height: 187px;" />
 * Flip the smallest number of nodes so that the pre-order traversal of the tree matches voyage.
 * Return a list of the values of all flipped nodes. You may return the answer in any order. If it is impossible to flip the nodes in the tree to make the pre-order traversal match voyage, return the list [-1].
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/02/1219-01.png" style="width: 150px; height: 205px;" />
 * Input: root = [1,2], voyage = [2,1]
 * Output: [-1]
 * Explanation: It is impossible to flip the nodes such that the pre-order traversal matches voyage.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/02/1219-02.png" style="width: 150px; height: 142px;" />
 * Input: root = [1,2,3], voyage = [1,3,2]
 * Output: [1]
 * Explanation: Flipping node 1 swaps nodes 2 and 3, so the pre-order traversal matches voyage.
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/02/1219-02.png" style="width: 150px; height: 142px;" />
 * Input: root = [1,2,3], voyage = [1,2,3]
 * Output: []
 * Explanation: The tree's pre-order traversal already matches voyage, so no nodes need to be flipped.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is n.
 * 	n == voyage.length
 * 	1 <= n <= 100
 * 	1 <= Node.val, voyage[i] <= n
 * 	All the values in the tree are unique.
 * 	All the values in voyage are unique.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/flip-binary-tree-to-match-preorder-traversal/
// discuss: https://leetcode.com/problems/flip-binary-tree-to-match-preorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn flip_match_voyage(root: Option<Rc<RefCell<TreeNode>>>, voyage: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut result = vec![];
        if Self::dfs_helper(&root, &voyage, &mut i, &mut result) {
            result
        } else {
            vec![-1]
        }
    }

    fn dfs_helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        voyage: &[i32],
        i: &mut usize,
        result: &mut Vec<i32>,
    ) -> bool {
        if let Some(n) = node {
            if n.borrow().val != voyage[*i] {
                return false;
            }
            *i += 1;
            let left = &n.borrow().left;
            if left.is_some() && left.as_ref().map(|l| l.borrow().val) != Some(voyage[*i]) {
                result.push(n.borrow().val);
                Self::dfs_helper(&n.borrow().right, voyage, i, result)
                    && Self::dfs_helper(&n.borrow().left, voyage, i, result)
            } else {
                Self::dfs_helper(&n.borrow().left, voyage, i, result)
                    && Self::dfs_helper(&n.borrow().right, voyage, i, result)
            }
        } else {
            true
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0971_example_1() {
        let root = tree![1, 2];
        let voyage = vec![2, 1];
        let result = vec![-1];

        assert_eq!(Solution::flip_match_voyage(root, voyage), result);
    }

    #[test]
    fn test_0971_example_2() {
        let root = tree![1, 2, 3];
        let voyage = vec![1, 3, 2];
        let result = vec![1];

        assert_eq!(Solution::flip_match_voyage(root, voyage), result);
    }

    #[test]
    fn test_0971_example_3() {
        let root = tree![1, 2, 3];
        let voyage = vec![1, 2, 3];
        let result = vec![];

        assert_eq!(Solution::flip_match_voyage(root, voyage), result);
    }
}

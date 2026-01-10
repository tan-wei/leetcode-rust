/**
 * [0655] Print Binary Tree
 *
 * Given the root of a binary tree, construct a 0-indexed m x n string matrix res that represents a formatted layout of the tree. The formatted layout matrix should be constructed using the following rules:
 *
 * 	The height of the tree is height and the number of rows m should be equal to height + 1.
 * 	The number of columns n should be equal to 2^height+1 - 1.
 * 	Place the root node in the middle of the top row (more formally, at location res[0][(n-1)/2]).
 * 	For each node that has been placed in the matrix at position res[r][c], place its left child at res[r+1][c-2^height-r-1] and its right child at res[r+1][c+2^height-r-1].
 * 	Continue this process until all the nodes in the tree have been placed.
 * 	Any empty cells should contain the empty string "".
 *
 * Return the constructed matrix res.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/print1-tree.jpg" style="width: 141px; height: 181px;" />
 * Input: root = [1,2]
 * Output:
 * [["","1",""],
 *  ["2","",""]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/print2-tree.jpg" style="width: 207px; height: 302px;" />
 * Input: root = [1,2,3,null,4]
 * Output:
 * [["","","","1","","",""],
 *  ["","2","","","","3",""],
 *  ["","","4","","","",""]]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 2^10].
 * 	-99 <= Node.val <= 99
 * 	The depth of the tree will be in the range [1, 10].
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/print-binary-tree/
// discuss: https://leetcode.com/problems/print-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        let height = Self::height(root.clone());
        Self::print_tree_internal(root, height)
    }

    fn print_tree_internal(root: Option<Rc<RefCell<TreeNode>>>, height: i32) -> Vec<Vec<String>> {
        let val = if root.is_none() {
            String::from("")
        } else {
            format!("{}", root.clone().unwrap().borrow().val)
        };

        if height == 1 {
            return vec![vec![String::from(val)]];
        }

        let mut result = Vec::new();
        let len = (1 << (height - 1)) - 1;
        let mut this_level = vec!["".to_string(); len * 2 + 1];
        this_level[len] = val;
        result.push(this_level);
        let (left_node, right_node) = if root.is_none() {
            (None, None)
        } else {
            (
                root.clone().unwrap().borrow().left.clone(),
                root.clone().unwrap().borrow().right.clone(),
            )
        };
        let left = Self::print_tree_internal(left_node, height - 1);
        let right = Self::print_tree_internal(right_node, height - 1);

        for i in 0..left.len() {
            let mut v = Vec::new();
            v.extend_from_slice(&left[i]);
            v.push(String::from(""));
            v.extend_from_slice(&right[i]);
            result.push(v);
        }

        result
    }

    fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let left = Self::height(root.borrow().left.clone());
                let height = Self::height(root.borrow().right.clone());
                left.max(height) + 1
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0655_example_1() {
        let root = tree![1, 2];
        let result = vec![vec_string!["", "1", ""], vec_string!["2", "", ""]];

        assert_eq!(Solution::print_tree(root), result);
    }

    #[test]
    fn test_0655_example_2() {
        let root = tree![1, 2, 3, null, 4];
        let result = vec![
            vec_string!["", "", "", "1", "", "", ""],
            vec_string!["", "2", "", "", "", "3", ""],
            vec_string!["", "", "4", "", "", "", ""],
        ];

        assert_eq!(Solution::print_tree(root), result);
    }
}

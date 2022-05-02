/**
 * [0662] Maximum Width of Binary Tree
 *
 * Given the root of a binary tree, return the maximum width of the given tree.
 * The maximum width of a tree is the maximum width among all levels.
 * The width of one level is defined as the length between the end-nodes (the leftmost and rightmost non-null nodes), where the null nodes between the end-nodes that would be present in a complete binary tree extending down to that level are also counted into the length calculation.
 * It is guaranteed that the answer will in the range of a 32-bit signed integer.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/width1-tree.jpg" style="width: 359px; height: 302px;" />
 * Input: root = [1,3,2,5,3,null,9]
 * Output: 4
 * Explanation: The maximum width exists in the third level with length 4 (5,3,null,9).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/14/maximum-width-of-binary-tree-v3.jpg" style="width: 442px; height: 422px;" />
 * Input: root = [1,3,2,5,null,null,9,6,null,7]
 * Output: 7
 * Explanation: The maximum width exists in the fourth level with length 7 (6,null,null,null,null,null,7).
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/width3-tree.jpg" style="width: 289px; height: 299px;" />
 * Input: root = [1,3,2,5]
 * Output: 2
 * Explanation: The maximum width exists in the second level with length 2 (3,2).
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 3000].
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/maximum-width-of-binary-tree/
// discuss: https://leetcode.com/problems/maximum-width-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v: Vec<usize> = Vec::new();
        Solution::dfs_helper(&root, 0, 0, &mut v) as i32
    }

    fn dfs_helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
        index: usize,
        v: &mut Vec<usize>,
    ) -> usize {
        if let Some(n) = node {
            if depth >= v.len() {
                v.push(index);
            }
            std::cmp::max(
                index - v[depth] + 1,
                std::cmp::max(
                    Solution::dfs_helper(&n.borrow().left, depth + 1, index * 2, v),
                    Solution::dfs_helper(&n.borrow().right, depth + 1, index * 2 + 1, v),
                ),
            )
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
    fn test_0662_example_1() {
        let root = tree![1, 3, 2, 5, 3, null, 9];
        let result = 4;

        assert_eq!(Solution::width_of_binary_tree(root), result);
    }

    #[test]
    fn test_0662_example_2() {
        let root = tree![1, 3, 2, 5, null, null, 9, 6, null, 7];
        let result = 7;

        assert_eq!(Solution::width_of_binary_tree(root), result);
    }

    #[test]
    fn test_0662_example_3() {
        let root = tree![1, 3, 2, 5];
        let result = 2;

        assert_eq!(Solution::width_of_binary_tree(root), result);
    }
}

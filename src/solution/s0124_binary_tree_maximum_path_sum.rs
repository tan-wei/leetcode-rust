/**
 * [124] Binary Tree Maximum Path Sum
 *
 * A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.
 * The path sum of a path is the sum of the node's values in the path.
 * Given the root of a binary tree, return the maximum path sum of any path.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx1.jpg" style="width: 322px; height: 182px;" />
 * Input: root = [1,2,3]
 * Output: 6
 * Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/13/exx2.jpg" />
 * Input: root = [-10,9,20,null,null,15,7]
 * Output: 42
 * Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 3 * 10^4].
 * 	-1000 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-maximum-path-sum/
// discuss: https://leetcode.com/problems/binary-tree-maximum-path-sum/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: &Option<Rc<RefCell<TreeNode>>>, max_in_the_tree: &mut i32) -> i32 {
            match root {
                None => 0,
                Some(root) => {
                    let mut left_sum = helper(&root.borrow().left, max_in_the_tree);
                    let mut right_sum = helper(&root.borrow().right, max_in_the_tree);
                    let root_val = root.borrow().val;

                    if left_sum < 0 {
                        left_sum = 0;
                    }

                    if right_sum < 0 {
                        right_sum = 0;
                    }

                    if left_sum + right_sum + root_val > *max_in_the_tree {
                        *max_in_the_tree = left_sum + right_sum + root_val;
                    }
                    root_val + left_sum.max(right_sum)
                }
            }
        }

        let mut max_in_the_tree = i32::MIN;
        helper(&root, &mut max_in_the_tree);
        max_in_the_tree
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0124_example_1() {
        let root = tree![1, 2, 3];
        let result = 6;

        assert_eq!(Solution::max_path_sum(root), result);
    }

    #[test]
    fn test_0124_example_2() {
        let root = tree![-10, 9, 20, null, null, 15, 7];
        let result = 42;

        assert_eq!(Solution::max_path_sum(root), result);
    }
}

/** [1457] Pseudo-Palindromic Paths in a Binary Tree
 *
 * Given a binary tree where node values are digits from 1 to 9. A path in the binary tree is said to be pseudo-palindromic if at least one permutation of the node values in the path is a palindrome.
 * Return the number of pseudo-palindromic paths going from the root node to leaf nodes.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/06/palindromic_paths_1.png" style="width: 300px; height: 201px;" />
 *
 * Input: root = [2,3,1,3,1,null,1]
 * Output: 2
 * Explanation: The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the red path [2,3,3], the green path [2,1,1], and the path [2,3,1]. Among these paths only red path and green path are pseudo-palindromic paths since the red path [2,3,3] can be rearranged in [3,2,3] (palindrome) and the green path [2,1,1] can be rearranged in [1,2,1] (palindrome).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/palindromic_paths_2.png" style="width: 300px; height: 314px;" />
 *
 * Input: root = [2,1,1,1,3,null,null,null,null,null,1]
 * Output: 1
 * Explanation: The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the green path [2,1,1], the path [2,1,3,1], and the path [2,1]. Among these paths only the green path is pseudo-palindromic since [2,1,1] can be rearranged in [1,2,1] (palindrome).
 *
 * Example 3:
 *
 * Input: root = [9]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^5].
 * 	1 <= Node.val <= 9
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/
// discuss: https://leetcode.com/problems/pseudo-palindromic-paths-in-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![];
        if let Some(node) = root {
            stack.push((node, 0usize));
        }

        let mut count = 0;

        while let Some((node, freq)) = stack.pop() {
            let mut node_ref = node.borrow_mut();
            let freq = freq ^ (1 << node_ref.val);

            if node_ref.left.is_none() && node_ref.right.is_none() {
                count += ((freq & (freq - 1)) == 0) as i32;
                continue;
            }

            if let Some(right) = node_ref.right.take() {
                stack.push((right, freq));
            }
            if let Some(left) = node_ref.left.take() {
                stack.push((left, freq));
            }
        }

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1457_example_1() {
        let root = tree![2, 3, 1, 3, 1, null, 1];

        let result = 2;

        assert_eq!(Solution::pseudo_palindromic_paths(root), result);
    }

    #[test]
    fn test_1457_example_2() {
        let root = tree![2, 1, 1, 1, 3, null, null, null, null, null, 1];

        let result = 1;

        assert_eq!(Solution::pseudo_palindromic_paths(root), result);
    }

    #[test]
    fn test_1457_example_3() {
        let root = tree![9];

        let result = 1;

        assert_eq!(Solution::pseudo_palindromic_paths(root), result);
    }
}

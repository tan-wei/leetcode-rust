/**
 * [0783] Minimum Distance Between BST Nodes
 *
 * Given the root of a Binary Search Tree (BST), return the minimum difference between the values of any two different nodes in the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/bst1.jpg" style="width: 292px; height: 301px;" />
 * Input: root = [4,2,6,1,3]
 * Output: 1
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/bst2.jpg" style="width: 282px; height: 301px;" />
 * Input: root = [1,0,48,null,null,12,49]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 100].
 * 	0 <= Node.val <= 10^5
 *
 *  
 * Note: This question is the same as 530: <a href="https://leetcode.com/problems/minimum-absolute-difference-in-bst/" target="_blank">https://leetcode.com/problems/minimum-absolute-difference-in-bst/</a>
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/minimum-distance-between-bst-nodes/
// discuss: https://leetcode.com/problems/minimum-distance-between-bst-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vals = vec![];
        Self::traverse(root, &mut vals);

        vals.windows(2).map(|c| c[1] - c[0]).min().unwrap()
    }

    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::traverse(node.borrow().left.clone(), vals);
            vals.push(node.borrow().val);
            Self::traverse(node.borrow().right.clone(), vals);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0783_example_1() {
        let root = tree![4, 2, 6, 1, 3];
        let result = 1;

        assert_eq!(Solution::min_diff_in_bst(root), result);
    }

    #[test]
    fn test_0783_example_2() {
        let root = tree![1, 0, 48, null, null, 12, 49];
        let result = 1;

        assert_eq!(Solution::min_diff_in_bst(root), result);
    }
}

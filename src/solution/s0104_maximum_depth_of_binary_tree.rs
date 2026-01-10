/**
 * [104] Maximum Depth of Binary Tree
 *
 * Given the root of a binary tree, return its maximum depth.
 *
 * A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/26/tmp-tree.jpg" style="width: 400px; height: 277px;" />
 *
 * Input: root = [3,9,20,null,null,15,7]
 * Output: 3
 *
 *
 * Example 2:
 *
 *
 * Input: root = [1,null,2]
 * Output: 2
 *
 *
 * Example 3:
 *
 *
 * Input: root = []
 * Output: 0
 *
 *
 * Example 4:
 *
 *
 * Input: root = [0]
 * Output: 1
 *
 *
 *  
 * Constraints:
 *
 *
 * 	The number of nodes in the tree is in the range [0, 10^4].
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/maximum-depth-of-binary-tree/
// discuss: https://leetcode.com/problems/maximum-depth-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |n| {
            1 + Self::max_depth(n.borrow().left.clone())
                .max(Self::max_depth(n.borrow().right.clone()))
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0104_example_1() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        let result = 3;

        assert_eq!(Solution::max_depth(root), result);
    }

    #[test]
    fn test_0104_example_2() {
        let root = tree![1, null, 2];
        let result = 2;

        assert_eq!(Solution::max_depth(root), result);
    }

    #[test]
    fn test_0104_example_3() {
        let root = tree![];
        let result = 0;

        assert_eq!(Solution::max_depth(root), result);
    }

    #[test]
    fn test_0104_example_4() {
        let root = tree![0];
        let result = 1;

        assert_eq!(Solution::max_depth(root), result);
    }
}

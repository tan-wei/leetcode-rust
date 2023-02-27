/**
 * [0988] Smallest String Starting From Leaf
 *
 * You are given the root of a binary tree where each node has a value in the range [0, 25] representing the letters 'a' to 'z'.
 * Return the lexicographically smallest string that starts at a leaf of this tree and ends at the root.
 * As a reminder, any shorter prefix of a string is lexicographically smaller.
 *
 * 	For example, "ab" is lexicographically smaller than "aba".
 *
 * A leaf of a node is a node that has no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/30/tree1.png" style="width: 534px; height: 358px;" />
 * Input: root = [0,1,2,3,4,3,4]
 * Output: "dba"
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/30/tree2.png" style="width: 534px; height: 358px;" />
 * Input: root = [25,1,3,1,3,0,2]
 * Output: "adz"
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/01/tree3.png" style="height: 490px; width: 468px;" />
 * Input: root = [2,2,1,null,1,0,null,0]
 * Output: "abc"
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 8500].
 * 	0 <= Node.val <= 25
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/smallest-string-starting-from-leaf/
// discuss: https://leetcode.com/problems/smallest-string-starting-from-leaf/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            None => "|".to_string(),
            Some(root) => {
                let s = char::from_u32((root.as_ref().borrow().val + 'a' as i32) as u32)
                    .unwrap()
                    .to_string();

                match (
                    root.as_ref().borrow().left.clone(),
                    root.as_ref().borrow().right.clone(),
                ) {
                    (None, None) => s,
                    (left, right) => std::cmp::min(
                        Self::smallest_from_leaf(left) + &s,
                        Self::smallest_from_leaf(right) + &s,
                    ),
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0988_example_1() {
        let root = tree![0, 1, 2, 3, 4, 3, 4];
        let result = "dba".to_string();

        assert_eq!(Solution::smallest_from_leaf(root), result);
    }

    #[test]
    fn test_0988_example_2() {
        let root = tree![25, 1, 3, 1, 3, 0, 2];
        let result = "adz".to_string();

        assert_eq!(Solution::smallest_from_leaf(root), result);
    }

    #[test]
    fn test_0988_example_3() {
        let root = tree![2, 2, 1, null, 1, 0, null, 0];
        let result = "abc".to_string();

        assert_eq!(Solution::smallest_from_leaf(root), result);
    }

    #[test]
    #[ignore]
    fn test_0988_additional_1() {
        let root = tree![25, 1, null, 0, 0, 1, null, null, null, 0];
        let result = "ababz".to_string();

        assert_eq!(Solution::smallest_from_leaf(root), result);
    }
}

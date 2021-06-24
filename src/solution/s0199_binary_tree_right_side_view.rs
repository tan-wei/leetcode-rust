/**
 * [199] Binary Tree Right Side View
 *
 * Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/14/tree.jpg" style="width: 401px; height: 301px;" />
 * Input: root = [1,2,3,null,5,null,4]
 * Output: [1,3,4]
 *
 * Example 2:
 *
 * Input: root = [1,null,3]
 * Output: [1,3]
 *
 * Example 3:
 *
 * Input: root = []
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-right-side-view/
// discuss: https://leetcode.com/problems/binary-tree-right-side-view/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            Some(a) => a,
            None => return vec![],
        };

        let mut res = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        while queue.len() > 0 {
            let len = queue.len() - 1;
            for i in 0..=len {
                let node = queue.pop_front().unwrap();
                if i == len {
                    res.push(node.borrow().val.clone());
                }
                if let Some(ref l) = node.clone().borrow().left {
                    queue.push_back(l.clone());
                }
                if let Some(ref r) = node.clone().borrow().right {
                    queue.push_back(r.clone());
                }
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0199_example_1() {
        let root = tree![1, 2, 3, null, 5, null, 4];
        let result = vec![1, 3, 4];

        assert_eq!(Solution::right_side_view(root), result);
    }

    #[test]
    fn test_0199_example_2() {
        let root = tree![1, null, 3];
        let result = vec![1, 3];

        assert_eq!(Solution::right_side_view(root), result);
    }

    #[test]
    fn test_0199_example_3() {
        let root = tree![];
        let result = vec![];

        assert_eq!(Solution::right_side_view(root), result);
    }
}

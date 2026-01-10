/**
 * [236] Lowest Common Ancestor of a Binary Tree
 *
 * Given a binary tree, find the lowest common ancestor (LCA) of two given nodes in the tree.
 * According to the <a href="https://en.wikipedia.org/wiki/Lowest_common_ancestor" target="_blank">definition of LCA on Wikipedia</a>: &ldquo;The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).&rdquo;
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
 * Output: 3
 * Explanation: The LCA of nodes 5 and 1 is 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/14/binarytree.png" style="width: 200px; height: 190px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
 * Output: 5
 * Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant of itself according to the LCA definition.
 *
 * Example 3:
 *
 * Input: root = [1,2], p = 1, q = 2
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 10^5].
 * 	-10^9 <= Node.val <= 10^9
 * 	All Node.val are unique.
 * 	p != q
 * 	p and q will exist in the tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
// discuss: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/discuss/963346/Rust-recursive
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root = match root {
            Some(a) => a,
            None => return None,
        };
        let v1 = p.unwrap().borrow().val;
        let v2 = q.unwrap().borrow().val;
        Self::helper(root, v1, v2)
    }

    fn helper(root: Rc<RefCell<TreeNode>>, v1: i32, v2: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let root_v = root.borrow().val;
        if root_v == v1 || root_v == v2 {
            return Some(root);
        }
        let l = root
            .borrow()
            .left
            .as_ref()
            .and_then(|a| Self::helper(a.clone(), v1, v2));
        let r = root
            .borrow()
            .right
            .as_ref()
            .and_then(|a| Self::helper(a.clone(), v1, v2));
        match (l, r) {
            (Some(_), Some(_)) => Some(root),
            (None, None) => None,
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0236_example_1() {
        let root = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let p = tree![5];
        let q = tree![1];
        let result = tree![3];

        assert_eq!(
            Solution::lowest_common_ancestor(root, p, q)
                .unwrap()
                .borrow()
                .val,
            result.unwrap().borrow().val
        );
    }

    #[test]
    fn test_0236_example_2() {
        let root = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let p = tree![5];
        let q = tree![4];
        let result = tree![5];

        assert_eq!(
            Solution::lowest_common_ancestor(root, p, q)
                .unwrap()
                .borrow()
                .val,
            result.unwrap().borrow().val
        );
    }

    #[test]
    fn test_0236_example_3() {
        let root = tree![1, 2];
        let p = tree![1];
        let q = tree![2];
        let result = tree![1];

        assert_eq!(
            Solution::lowest_common_ancestor(root, p, q)
                .unwrap()
                .borrow()
                .val,
            result.unwrap().borrow().val
        );
    }
}

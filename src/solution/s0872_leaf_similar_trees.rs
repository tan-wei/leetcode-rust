/**
 * [0872] Leaf-Similar Trees
 *
 * Consider all the leaves of a binary tree, from left to right order, the values of those leaves form a leaf value sequence.
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/16/tree.png" style="width: 400px; height: 336px;" />
 * For example, in the given tree above, the leaf value sequence is (6, 7, 4, 9, 8).
 * Two binary trees are considered leaf-similar if their leaf value sequence is the same.
 * Return true if and only if the two given trees with head nodes root1 and root2 are leaf-similar.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/leaf-similar-1.jpg" style="width: 600px; height: 237px;" />
 * Input: root1 = [3,5,1,6,2,9,8,null,null,7,4], root2 = [3,5,1,6,7,4,2,null,null,null,null,null,null,9,8]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/leaf-similar-2.jpg" style="width: 300px; height: 110px;" />
 * Input: root1 = [1,2,3], root2 = [1,3,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in each tree will be in the range [1, 200].
 * 	Both of the given trees will have values in the range [0, 200].
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/leaf-similar-trees/
// discuss: https://leetcode.com/problems/leaf-similar-trees/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::collect_leaves(root1) == Self::collect_leaves(root2)
    }

    fn collect_leaves(n: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match n {
            None => vec![],
            Some(n) => {
                if n.borrow().left.is_none() && n.borrow().right.is_none() {
                    return vec![n.borrow().val];
                }
                let mut list = Self::collect_leaves(n.borrow().left.clone());
                list.extend(Self::collect_leaves(n.borrow().right.clone()));
                list
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0872_example_1() {
        let root1 = tree![3, 5, 1, 6, 2, 9, 8, null, null, 7, 4];
        let root2 = tree![3, 5, 1, 6, 7, 4, 2, null, null, null, null, null, null, 9, 8];
        let result = true;

        assert_eq!(Solution::leaf_similar(root1, root2), result);
    }

    #[test]
    fn test_0872_example_2() {
        let root1 = tree![1, 2, 3];
        let root2 = tree![1, 3, 2];
        let result = false;

        assert_eq!(Solution::leaf_similar(root1, root2), result);
    }
}

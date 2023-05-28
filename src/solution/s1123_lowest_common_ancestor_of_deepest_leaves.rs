/**
 * [1123] Lowest Common Ancestor of Deepest Leaves
 *
 * Given the root of a binary tree, return the lowest common ancestor of its deepest leaves.
 * Recall that:
 *
 * 	The node of a binary tree is a leaf if and only if it has no children
 * 	The depth of the root of the tree is 0. if the depth of a node is d, the depth of each of its children is d + 1.
 * 	The lowest common ancestor of a set S of nodes, is the node A with the largest depth such that every node in S is in the subtree with root A.
 *
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/01/sketch1.png" style="width: 600px; height: 510px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4]
 * Output: [2,7,4]
 * Explanation: We return the node with value 2, colored in yellow in the diagram.
 * The nodes coloured in blue are the deepest leaf-nodes of the tree.
 * Note that nodes 6, 0, and 8 are also leaf nodes, but the depth of them is 2, but the depth of nodes 7 and 4 is 3.
 * Example 2:
 *
 * Input: root = [1]
 * Output: [1]
 * Explanation: The root is the deepest node in the tree, and it's the lca of itself.
 *
 * Example 3:
 *
 * Input: root = [0,1,3,null,2]
 * Output: [2]
 * Explanation: The deepest leaf node in the tree is 2, the lca of one node is itself.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree will be in the range [1, 1000].
 * 	0 <= Node.val <= 1000
 * 	The values of the nodes in the tree are unique.
 *
 *  
 * Note: This question is the same as 865: <a href="https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/" target="_blank">https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/</a>
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/
// discuss: https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut level = 0;
        Self::dfs_helper(root, &mut level)
    }

    fn dfs_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        level: &mut i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(root) => {
                let mut ll = *level;
                let mut rl = *level;

                let left = Self::dfs_helper(root.borrow().left.clone(), &mut ll);
                let right = Self::dfs_helper(root.borrow().right.clone(), &mut rl);

                match ll.cmp(&rl) {
                    std::cmp::Ordering::Less => {
                        *level = rl + 1;
                        right
                    }
                    std::cmp::Ordering::Greater => {
                        *level = ll + 1;
                        left
                    }
                    std::cmp::Ordering::Equal => {
                        *level = ll + 1;
                        Some(root)
                    }
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
    fn test_1123_example_1() {
        let root = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let result = tree![2, 7, 4];

        assert_eq!(Solution::lca_deepest_leaves(root), result);
    }

    #[test]
    fn test_1123_example_2() {
        let root = tree![0, 1, 3, null, 2];
        let result = tree![2];

        assert_eq!(Solution::lca_deepest_leaves(root), result);
    }
}

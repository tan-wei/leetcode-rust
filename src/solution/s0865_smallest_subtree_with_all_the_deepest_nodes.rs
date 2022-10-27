/**
 * [0865] Smallest Subtree with all the Deepest Nodes
 *
 * Given the root of a binary tree, the depth of each node is the shortest distance to the root.
 * Return the smallest subtree such that it contains all the deepest nodes in the original tree.
 * A node is called the deepest if it has the largest depth possible among any node in the entire tree.
 * The subtree of a node is a tree consisting of that node, plus the set of all descendants of that node.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/01/sketch1.png" style="width: 600px; height: 510px;" />
 * Input: root = [3,5,1,6,2,0,8,null,null,7,4]
 * Output: [2,7,4]
 * Explanation: We return the node with value 2, colored in yellow in the diagram.
 * The nodes coloured in blue are the deepest nodes of the tree.
 * Notice that nodes 5, 3 and 2 contain the deepest nodes in the tree but node 2 is the smallest subtree among them, so we return it.
 *
 * <strong class="example">Example 2:
 *
 * Input: root = [1]
 * Output: [1]
 * Explanation: The root is the deepest node in the tree.
 *
 * <strong class="example">Example 3:
 *
 * Input: root = [0,1,3,null,2]
 * Output: [2]
 * Explanation: The deepest node in the tree is 2, the valid subtrees are the subtrees of nodes 2, 1 and 0 but the subtree of node 2 is the smallest.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree will be in the range [1, 500].
 * 	0 <= Node.val <= 500
 * 	The values of the nodes in the tree are unique.
 *
 *  
 * Note: This question is the same as 1123: <a href="https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/" target="_blank">https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/</a>
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/
// discuss: https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs_helper(root).0
    }

    fn dfs_helper(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        let root = match root {
            Some(a) => a,
            None => return (None, 0),
        };
        let l = Self::dfs_helper(root.borrow().left.clone());
        let r = Self::dfs_helper(root.borrow().right.clone());
        if l.1 > r.1 {
            (l.0, l.1 + 1)
        } else if l.1 < r.1 {
            (r.0, r.1 + 1)
        } else {
            (Some(root), l.1 + 1)
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0865_example_1() {
        let root = tree![1];
        let result = tree![1];

        assert_eq!(Solution::subtree_with_all_deepest(root), result);
    }

    #[test]
    fn test_0865_example_2() {
        let root = tree![0, 1, 3, null, 2];
        let result = tree![2];

        assert_eq!(Solution::subtree_with_all_deepest(root), result);
    }
}

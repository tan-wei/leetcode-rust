/**
 * [0814] Binary Tree Pruning
 *
 * Given the root of a binary tree, return the same tree where every subtree (of the given tree) not containing a 1 has been removed.
 * A subtree of a node node is node plus every node that is a descendant of node.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_2.png" style="width: 500px; height: 140px;" />
 * Input: root = [1,null,0,0,1]
 * Output: [1,null,0,null,1]
 * Explanation:
 * Only the red nodes satisfy the property "every subtree not containing a 1".
 * The diagram on the right represents the answer.
 *
 * Example 2:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_1.png" style="width: 500px; height: 115px;" />
 * Input: root = [1,0,1,0,0,0,1]
 * Output: [1,null,1,null,1]
 *
 * Example 3:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/05/1028.png" style="width: 500px; height: 134px;" />
 * Input: root = [1,1,0,1,1,0,1,0]
 * Output: [1,1,0,1,1,null,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 200].
 * 	Node.val is either 0 or 1.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-pruning/
// discuss: https://leetcode.com/problems/binary-tree-pruning/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.and_then(|node| {
            let left = Solution::prune_tree(node.borrow_mut().left.take());
            let right = Solution::prune_tree(node.borrow_mut().right.take());

            if node.borrow().val == 0 && left.is_none() && right.is_none() {
                return None;
            }

            Some(Rc::new(RefCell::new(TreeNode {
                val: node.borrow().val,
                left,
                right,
            })))
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0814_example_1() {
        let root = tree![1, null, 0, 0, 1];
        let result = tree![1, null, 0, null, 1];

        assert_eq!(Solution::prune_tree(root), result);
    }

    #[test]
    fn test_0814_example_2() {
        let root = tree![1, 0, 1, 0, 0, 0, 1];
        let result = tree![1, null, 1, null, 1];

        assert_eq!(Solution::prune_tree(root), result);
    }

    #[test]
    fn test_0814_example_3() {
        let root = tree![1, 0, 1, 0, 0, 0, 1];
        let result = tree![1, null, 1, null, 1];

        assert_eq!(Solution::prune_tree(root), result);
    }
}

/**
 * [0617] Merge Two Binary Trees
 *
 * You are given two binary trees root1 and root2.
 * Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.
 * Return the merged tree.
 * Note: The merging process must start from the root nodes of both trees.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/05/merge.jpg" style="width: 600px; height: 163px;" />
 * Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
 * Output: [3,4,5,5,4,null,7]
 *
 * Example 2:
 *
 * Input: root1 = [1], root2 = [1,2]
 * Output: [2,2]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in both trees is in the range [0, 2000].
 * 	-10^4 <= Node.val <= 10^4
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/merge-two-binary-trees/
// discuss: https://leetcode.com/problems/merge-two-binary-trees/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1.is_none(), root2.is_none()) {
            (true, _) => root2,
            (_, true) => root1,
            (_, _) => {
                let root1 = root1.unwrap();
                let root2 = root2.unwrap();
                let mut result;
                result = TreeNode::new(root1.borrow().val + root2.borrow().val);
                result.left =
                    Self::merge_trees(root1.borrow().left.clone(), root2.borrow().left.clone());
                result.right =
                    Self::merge_trees(root1.borrow().right.clone(), root2.borrow().right.clone());

                Some(Rc::new(RefCell::new(result)))
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0617_example_1() {
        let root1 = tree![1, 3, 2, 5];
        let root2 = tree![2, 1, 3, null, 4, null, 7];
        let result = tree![3, 4, 5, 5, 4, null, 7];

        assert_eq!(Solution::merge_trees(root1, root2), result);
    }

    #[test]
    fn test_0617_example_2() {
        let root1 = tree![1];
        let root2 = tree![1, 2];
        let result = tree![2, 2];

        assert_eq!(Solution::merge_trees(root1, root2), result);
    }
}

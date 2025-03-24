/**
 * [1932] Merge BSTs to Create Single BST
 *
 * You are given n BST (binary search tree) root nodes for n separate BSTs stored in an array trees (0-indexed). Each BST in trees has at most 3 nodes, and no two roots have the same value. In one operation, you can:
 *
 * 	Select two distinct indices i and j such that the value stored at one of the leaves of trees[i] is equal to the root value of trees[j].
 * 	Replace the leaf node in trees[i] with trees[j].
 * 	Remove trees[j] from trees.
 *
 * Return the root of the resulting BST if it is possible to form a valid BST after performing n - 1 operations, or null if it is impossible to create a valid BST.
 * A BST (binary search tree) is a binary tree where each node satisfies the following property:
 *
 * 	Every node in the node's left subtree has a value strictly less than the node's value.
 * 	Every node in the node's right subtree has a value strictly greater than the node's value.
 *
 * A leaf is a node that has no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/d1.png" style="width: 450px; height: 163px;" />
 * Input: trees = [[2,1],[3,2,5],[5,4]]
 * Output: [3,2,5,1,null,4]
 * Explanation:
 * In the first operation, pick i=1 and j=0, and merge trees[0] into trees[1].
 * Delete trees[0], so trees = [[3,2,5,1],[5,4]].
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/24/diagram.png" style="width: 450px; height: 181px;" />
 * In the second operation, pick i=0 and j=1, and merge trees[1] into trees[0].
 * Delete trees[1], so trees = [[3,2,5,1,null,4]].
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/24/diagram-2.png" style="width: 220px; height: 165px;" />
 * The resulting tree, shown above, is a valid BST, so return its root.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/d2.png" style="width: 450px; height: 171px;" />
 * Input: trees = [[5,3,8],[3,2,6]]
 * Output: []
 * Explanation:
 * Pick i=0 and j=1 and merge trees[1] into trees[0].
 * Delete trees[1], so trees = [[5,3,8,2,6]].
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/24/diagram-3.png" style="width: 240px; height: 196px;" />
 * The resulting tree is shown above. This is the only valid operation that can be performed, but the resulting tree is not a valid BST, so return null.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/d3.png" style="width: 430px; height: 168px;" />
 * Input: trees = [[5,4],[3]]
 * Output: []
 * Explanation: It is impossible to perform any operations.
 *
 *  
 * Constraints:
 *
 * 	n == trees.length
 * 	1 <= n <= 5 * 10^4
 * 	The number of nodes in each tree is in the range [1, 3].
 * 	Each node in the input may have children but no grandchildren.
 * 	No two roots of trees have the same value.
 * 	All the trees in the input are valid BSTs.
 * 	1 <= TreeNode.val <= 5 * 10^4.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/merge-bsts-to-create-single-bst/
// discuss: https://leetcode.com/problems/merge-bsts-to-create-single-bst/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn can_merge(trees: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(0))))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    #[ignore]
    fn test_1932_example_1() {
        let trees = vec![tree![2, 1], tree![3, 2, 5], tree![5, 4]];

        let result = tree![3, 2, 5, 1, null, 4];

        assert_eq!(Solution::can_merge(trees), result);
    }

    #[test]
    #[ignore]
    fn test_1932_example_2() {
        let trees = vec![tree![5, 3, 8], tree![3, 2, 6]];

        let result = tree![];

        assert_eq!(Solution::can_merge(trees), result);
    }

    #[test]
    #[ignore]
    fn test_1932_example_3() {
        let trees = vec![tree![5, 4], tree![3]];

        let result = tree![];

        assert_eq!(Solution::can_merge(trees), result);
    }
}

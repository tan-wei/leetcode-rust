/**
 * [1448] Count Good Nodes in Binary Tree
 *
 * Given a binary tree root, a node X in the tree is named good if in the path from root to X there are no nodes with a value greater than X.
 *
 * Return the number of good nodes in the binary tree.
 *
 *  
 * Example 1:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/02/test_sample_1.png" style="width: 263px; height: 156px;" />
 *
 *
 * Input: root = [3,1,4,3,null,1,5]
 * Output: 4
 * Explanation: Nodes in blue are good.
 * Root Node (3) is always a good node.
 * Node 4 -> (3,4) is the maximum value in the path starting from the root.
 * Node 5 -> (3,4,5) is the maximum value in the path
 * Node 3 -> (3,1,3) is the maximum value in the path.
 *
 * Example 2:
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/02/test_sample_2.png" style="width: 157px; height: 161px;" />
 *
 *
 * Input: root = [3,3,null,4,2]
 * Output: 3
 * Explanation: Node 2 -> (3, 3, 2) is not good, because "3" is higher than it.
 *
 * Example 3:
 *
 *
 * Input: root = [1]
 * Output: 1
 * Explanation: Root is considered as good.
 *
 *  
 * Constraints:
 *
 *
 * 	The number of nodes in the binary tree is in the range [1, 10^5].
 * 	Each node's value is between [-10^4, 10^4].
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/count-good-nodes-in-binary-tree/
// discuss: https://leetcode.com/problems/count-good-nodes-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![(root, i32::MIN)];
        let mut result = 0;

        while let Some(tuple) = stack.pop() {
            if let (Some(node_rc), mut max_val) = tuple {
                let mut node_ref = node_rc.borrow_mut();
                if node_ref.val >= max_val {
                    result += 1;
                    max_val = node_ref.val;
                }
                stack.push((node_ref.left.take(), max_val));
                stack.push((node_ref.right.take(), max_val));
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1448_example_1() {
        let root = tree![3, 1, 4, 3, null, 1, 5];

        let result = 4;

        assert_eq!(Solution::good_nodes(root), result);
    }

    #[test]
    fn test_1448_example_2() {
        let root = tree![3, 3, null, 4, 2];

        let result = 3;

        assert_eq!(Solution::good_nodes(root), result);
    }

    #[test]
    fn test_1448_example_3() {
        let root = tree![1];

        let result = 1;

        assert_eq!(Solution::good_nodes(root), result);
    }
}

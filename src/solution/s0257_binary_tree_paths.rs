/**
 * [257] Binary Tree Paths
 *
 * Given the root of a binary tree, return all root-to-leaf paths in any order.
 * A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/12/paths-tree.jpg" style="width: 207px; height: 293px;" />
 * Input: root = [1,2,3,null,5]
 * Output: ["1->2->5","1->3"]
 *
 * Example 2:
 *
 * Input: root = [1]
 * Output: ["1"]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 100].
 * 	-100 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-paths/
// discuss: https://leetcode.com/problems/binary-tree-paths/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>, path: &str, paths: &mut Vec<String>) {
            if let Some(root) = root {
                let root = root.borrow();
                let val = root.val.to_string();

                let current_path = if path.len() == 0 {
                    val
                } else {
                    path.clone().to_owned() + "->" + &val
                };

                if root.left.is_none() && root.right.is_none() {
                    paths.push(current_path);
                } else {
                    helper(root.left.clone(), &current_path, paths);
                    helper(root.right.clone(), &current_path, paths);
                }
            }
        }

        let mut paths = vec![];

        helper(root, "", &mut paths);

        paths
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0257_example_1() {
        let root = tree![1, 2, 3, null, 5];
        let result = vec_string!["1->2->5", "1->3"];

        assert_eq_sorted!(Solution::binary_tree_paths(root), result);
    }

    #[test]
    fn test_0257_example_2() {
        let root = tree![1];
        let result = vec_string!["1"];

        assert_eq_sorted!(Solution::binary_tree_paths(root), result);
    }
}

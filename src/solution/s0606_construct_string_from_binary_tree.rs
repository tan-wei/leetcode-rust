/**
 * [0606] Construct String from Binary Tree
 *
 * Given the root of a binary tree, construct a string consisting of parenthesis and integers from a binary tree with the preorder traversal way, and return it.
 * Omit all the empty parenthesis pairs that do not affect the one-to-one mapping relationship between the string and the original binary tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/cons1-tree.jpg" style="width: 292px; height: 301px;" />
 * Input: root = [1,2,3,4]
 * Output: "1(2(4))(3)"
 * Explanation: Originally, it needs to be "1(2(4)())(3()())", but you need to omit all the unnecessary empty parenthesis pairs. And it will be "1(2(4))(3)"
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/cons2-tree.jpg" style="width: 207px; height: 293px;" />
 * Input: root = [1,2,3,null,4]
 * Output: "1(2()(4))(3)"
 * Explanation: Almost the same as the first example, except we cannot omit the first parenthesis pair to break the one-to-one mapping relationship between the input and the output.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-1000 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/construct-string-from-binary-tree/
// discuss: https://leetcode.com/problems/construct-string-from-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::dfs_helper(&root)
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(root) = root {
            let left = Self::dfs_helper(&root.borrow().left);
            let right = Self::dfs_helper(&root.borrow().right);
            let mut result = root.borrow().val.to_string();
            let children = match (left.is_empty(), right.is_empty()) {
                (true, true) => "".to_string(),
                (false, true) => format!("({})", &left),
                (true, false) => format!("()({})", &right),
                (false, false) => format!("({})({})", &left, &right),
            };
            result += &children;
            result
        } else {
            "".to_string()
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0606_example_1() {
        let root = tree![1, 2, 3, 4];
        let result = "1(2(4))(3)".to_string();

        assert_eq!(Solution::tree2str(root), result);
    }

    #[test]
    fn test_0606_example_2() {
        let root = tree![1, 2, 3, null, 4];
        let result = "1(2()(4))(3)".to_string();

        assert_eq!(Solution::tree2str(root), result);
    }
}

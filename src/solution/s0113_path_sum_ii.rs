/**
 * [113] Path Sum II
 *
 * Given the root of a binary tree and an integer targetSum, return all root-to-leaf paths where each path's sum equals targetSum.
 * A leaf is a node with no children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsumii1.jpg" style="width: 500px; height: 356px;" />
 * Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
 * Output: [[5,4,11,2],[5,8,4,5]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/pathsum2.jpg" style="width: 212px; height: 181px;" />
 * Input: root = [1,2,3], targetSum = 5
 * Output: []
 *
 * Example 3:
 *
 * Input: root = [1,2], targetSum = 0
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-1000 <= Node.val <= 1000
 * 	-1000 <= targetSum <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/path-sum-ii/
// discuss: https://leetcode.com/problems/path-sum-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn helper(
            root: &Option<Rc<RefCell<TreeNode>>>,
            cur: &mut i32,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
            target_sum: i32,
        ) {
            if let Some(node) = root {
                *cur += node.borrow().val;
                path.push(node.borrow().val);
                if cur == &target_sum
                    && node.borrow().left.is_none()
                    && node.borrow().right.is_none()
                {
                    result.push(path.clone());
                } else {
                    helper(&node.borrow().left, cur, path, result, target_sum);
                    helper(&node.borrow().right, cur, path, result, target_sum);
                }
                path.pop();
                *cur -= node.borrow().val;
            }
        }

        let mut result = vec![];
        helper(&root, &mut 0, &mut vec![], &mut result, target_sum);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0113_example_1() {
        let root = tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1];
        let target_sum = 22;
        let result = vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]];

        assert_eq!(Solution::path_sum(root, target_sum), result);
    }

    #[test]
    fn test_0113_example_2() {
        let root = tree![1, 2, 3];
        let target_sum = 5;
        let result: Vec<Vec<_>> = vec![];

        assert_eq!(Solution::path_sum(root, target_sum), result);
    }

    #[test]
    fn test_0113_example_3() {
        let root = tree![1, 2];
        let target_sum = 0;
        let result: Vec<Vec<_>> = vec![];

        assert_eq!(Solution::path_sum(root, target_sum), result);
    }
}

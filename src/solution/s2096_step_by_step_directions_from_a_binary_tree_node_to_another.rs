/**
 * [2096] Step-By-Step Directions From a Binary Tree Node to Another
 *
 * You are given the root of a binary tree with n nodes. Each node is uniquely assigned a value from 1 to n. You are also given an integer startValue representing the value of the start node s, and a different integer destValue representing the value of the destination node t.
 * Find the shortest path starting from node s and ending at node t. Generate step-by-step directions of such path as a string consisting of only the uppercase letters 'L', 'R', and 'U'. Each letter indicates a specific direction:
 *
 * 	'L' means to go from a node to its left child node.
 * 	'R' means to go from a node to its right child node.
 * 	'U' means to go from a node to its parent node.
 *
 * Return the step-by-step directions of the shortest path from node s to node t.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/15/eg1.png" style="width: 214px; height: 163px;" />
 * Input: root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
 * Output: "UURL"
 * Explanation: The shortest path is: 3 &rarr; 1 &rarr; 5 &rarr; 2 &rarr; 6.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/15/eg2.png" style="width: 74px; height: 102px;" />
 * Input: root = [2,1], startValue = 2, destValue = 1
 * Output: "L"
 * Explanation: The shortest path is: 2 &rarr; 1.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is n.
 * 	2 <= n <= 10^5
 * 	1 <= Node.val <= n
 * 	All the values in the tree are unique.
 * 	1 <= startValue, destValue <= n
 * 	startValue != destValue
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/
// discuss: https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2096_example_1() {
        let root = tree![5, 1, 2, 3, null, 6, 4];
        let start_value = 3;
        let dest_value = 6;

        let result = "UURL".to_string();

        assert_eq!(
            Solution::get_directions(root, start_value, dest_value),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2096_example_2() {
        let root = tree![2, 1];
        let start_value = 2;
        let dest_value = 1;

        let result = "L".to_string();

        assert_eq!(
            Solution::get_directions(root, start_value, dest_value),
            result
        );
    }
}

/**
 * [2196] Create Binary Tree From Descriptions
 *
 * You are given a 2D integer array descriptions where descriptions[i] = [parenti, childi, isLefti] indicates that parenti is the parent of childi in a binary tree of unique values. Furthermore,
 *
 * 	If isLefti == 1, then childi is the left child of parenti.
 * 	If isLefti == 0, then childi is the right child of parenti.
 *
 * Construct the binary tree described by descriptions and return its root.
 * The test cases will be generated such that the binary tree is valid.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/09/example1drawio.png" style="width: 300px; height: 236px;" />
 * Input: descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
 * Output: [50,20,80,15,17,19]
 * Explanation: The root node is the node with value 50 since it has no parent.
 * The resulting binary tree is shown in the diagram.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/09/example2drawio.png" style="width: 131px; height: 300px;" />
 * Input: descriptions = [[1,2,1],[2,3,0],[3,4,1]]
 * Output: [1,2,null,null,3,4]
 * Explanation: The root node is the node with value 1 since it has no parent.
 * The resulting binary tree is shown in the diagram.
 *
 *  
 * Constraints:
 *
 * 	1 <= descriptions.length <= 10^4
 * 	descriptions[i].length == 3
 * 	1 <= parenti, childi <= 10^5
 * 	0 <= isLefti <= 1
 * 	The binary tree described by descriptions is valid.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/create-binary-tree-from-descriptions/
// discuss: https://leetcode.com/problems/create-binary-tree-from-descriptions/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(0))))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2196_example_1() {
        let descriptions = vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ];

        let result = tree![50, 20, 80, 15, 17, 19];

        assert_eq!(Solution::create_binary_tree(descriptions), result);
    }

    #[test]
    #[ignore]
    fn test_2196_example_2() {
        let descriptions = vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]];

        let result = tree![1, 2, null, null, 3, 4];

        assert_eq!(Solution::create_binary_tree(descriptions), result);
    }
}

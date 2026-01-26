/**
 * [2331] Evaluate Boolean Binary Tree
 *
 * You are given the root of a full binary tree with the following properties:
 *
 * 	Leaf nodes have either the value 0 or 1, where 0 represents False and 1 represents True.
 * 	Non-leaf nodes have either the value 2 or 3, where 2 represents the boolean OR and 3 represents the boolean AND.
 *
 * The evaluation of a node is as follows:
 *
 * 	If the node is a leaf node, the evaluation is the value of the node, i.e. True or False.
 * 	Otherwise, evaluate the node's two children and apply the boolean operation of its value with the children's evaluations.
 *
 * Return the boolean result of evaluating the root node.
 * A full binary tree is a binary tree where each node has either 0 or 2 children.
 * A leaf node is a node that has zero children.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/16/example1drawio1.png" style="width: 700px; height: 252px;" />
 * Input: root = [2,1,3,null,null,0,1]
 * Output: true
 * Explanation: The above diagram illustrates the evaluation process.
 * The AND node evaluates to False AND True = False.
 * The OR node evaluates to True OR False = True.
 * The root node evaluates to True, so we return true.
 * Example 2:
 *
 * Input: root = [0]
 * Output: false
 * Explanation: The root node is a leaf node and it evaluates to false, so we return false.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	0 <= Node.val <= 3
 * 	Every node has either 0 or 2 children.
 * 	Leaf nodes have a value of 0 or 1.
 * 	Non-leaf nodes have a value of 2 or 3.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/evaluate-boolean-binary-tree/
// discuss: https://leetcode.com/problems/evaluate-boolean-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let node = root.as_ref().unwrap().borrow();
        match node.val {
            0 => false,
            1 => true,
            2 => Self::evaluate_tree(node.left.clone()) || Self::evaluate_tree(node.right.clone()),
            _ => Self::evaluate_tree(node.left.clone()) && Self::evaluate_tree(node.right.clone()),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2331_example_1() {
        let root = tree![2, 1, 3, null, null, 0, 1];

        let result = true;

        assert_eq!(Solution::evaluate_tree(root), result);
    }

    #[test]
    fn test_2331_example_2() {
        let root = tree![0];

        let result = false;

        assert_eq!(Solution::evaluate_tree(root), result);
    }
}

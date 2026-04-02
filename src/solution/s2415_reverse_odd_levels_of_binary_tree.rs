/**
 * [2415] Reverse Odd Levels of Binary Tree
 *
 * Given the root of a perfect binary tree, reverse the node values at each odd level of the tree.
 *
 * 	For example, suppose the node values at level 3 are [2,1,3,4,7,11,29,18], then it should become [18,29,11,7,4,3,1,2].
 *
 * Return the root of the reversed tree.
 * A binary tree is perfect if all parent nodes have two children and all leaves are on the same level.
 * The level of a node is the number of edges along the path between it and the root node.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/07/28/first_case1.png" style="width: 626px; height: 191px;" />
 * Input: root = [2,3,5,8,13,21,34]
 * Output: [2,5,3,8,13,21,34]
 * Explanation:
 * The tree has only one odd level.
 * The nodes at level 1 are 3, 5 respectively, which are reversed and become 5, 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/07/28/second_case3.png" style="width: 591px; height: 111px;" />
 * Input: root = [7,13,11]
 * Output: [7,11,13]
 * Explanation:
 * The nodes at level 1 are 13, 11, which are reversed and become 11, 13.
 *
 * Example 3:
 *
 * Input: root = [0,1,2,0,0,0,0,1,1,1,1,2,2,2,2]
 * Output: [0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]
 * Explanation:
 * The odd levels have non-zero values.
 * The nodes at level 1 were 1, 2, and are 2, 1 after the reversal.
 * The nodes at level 3 were 1, 1, 1, 1, 2, 2, 2, 2, and are 2, 2, 2, 2, 1, 1, 1, 1 after the reversal.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 2^14].
 * 	0 <= Node.val <= 10^5
 * 	root is a perfect binary tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/
// discuss: https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs_helper(root.clone(), None, 0);

        root
    }

    fn dfs_helper(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
        level: i32,
    ) {
        let rev = level & 0x01 == 1;
        match (left.clone(), right.clone()) {
            (Some(l_node), None) => {
                Self::dfs_helper(
                    l_node.borrow().left.clone(),
                    l_node.borrow().right.clone(),
                    level + 1,
                );
            }
            (Some(l_node), Some(r_node)) => {
                if rev {
                    let lval = l_node.borrow().val;
                    let rval = r_node.borrow().val;
                    l_node.borrow_mut().val = rval;
                    r_node.borrow_mut().val = lval;
                }

                Self::dfs_helper(
                    l_node.borrow().left.clone(),
                    r_node.borrow().right.clone(),
                    level + 1,
                );
                Self::dfs_helper(
                    l_node.borrow().right.clone(),
                    r_node.borrow().left.clone(),
                    level + 1,
                );
            }
            (None, Some(r_node)) => {
                Self::dfs_helper(
                    r_node.borrow().left.clone(),
                    r_node.borrow().right.clone(),
                    level + 1,
                );
            }
            _ => {}
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2415_example_1() {
        let root = tree![2, 3, 5, 8, 13, 21, 34];

        let result = tree![2, 5, 3, 8, 13, 21, 34];

        assert_eq!(Solution::reverse_odd_levels(root), result);
    }

    #[test]
    fn test_2415_example_2() {
        let root = tree![7, 13, 11];

        let result = tree![7, 11, 13];

        assert_eq!(Solution::reverse_odd_levels(root), result);
    }

    #[test]
    fn test_2415_example_3() {
        let root = tree![0, 1, 2, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2];

        let result = tree![0, 2, 1, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1];

        assert_eq!(Solution::reverse_odd_levels(root), result);
    }
}

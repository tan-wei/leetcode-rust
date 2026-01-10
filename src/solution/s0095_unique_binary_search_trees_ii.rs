/**
 * [95] Unique Binary Search Trees II
 *
 * Given an integer n, return all the structurally unique BST's (binary search trees), which has exactly n nodes of unique values from 1 to n. Return the answer in any order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/18/uniquebstn3.jpg" style="width: 600px; height: 148px;" />
 * Input: n = 3
 * Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: [[1]]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 8
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/unique-binary-search-trees-ii/
// discuss: https://leetcode.com/problems/unique-binary-search-trees-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 { vec![] } else { Self::helper(1, n) }
    }

    fn helper(l: i32, r: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l > r {
            vec![None]
        } else {
            let mut res = vec![];
            for i in l..r + 1 {
                let lnodes = Self::helper(l, i - 1);
                let rnodes = Self::helper(i + 1, r);
                for ln in lnodes.iter() {
                    for rn in rnodes.iter() {
                        let node = Some(Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: ln.clone(),
                            right: rn.clone(),
                        })));
                        res.push(node);
                    }
                }
            }
            res
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0095_example_1() {
        let n = 3;
        let result = vec![
            tree![1, null, 2, null, 3],
            tree![1, null, 3, 2],
            tree![2, 1, 3],
            tree![3, 1, null, null, 2],
            tree![3, 2, null, 1],
        ];

        assert_eq!(Solution::generate_trees(n), result);
    }

    #[test]
    fn test_0095_example_2() {
        let n = 1;
        let result = vec![tree![1]];

        assert_eq!(Solution::generate_trees(n), result);
    }
}

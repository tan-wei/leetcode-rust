/**
 * [0894] All Possible Full Binary Trees
 *
 * Given an integer n, return a list of all possible full binary trees with n nodes. Each node of each tree in the answer must have Node.val == 0.
 * Each element of the answer is the root node of one possible tree. You may return the final list of trees in any order.
 * A full binary tree is a binary tree where each node has exactly 0 or 2 children.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/22/fivetrees.png" style="width: 700px; height: 400px;" />
 * Input: n = 7
 * Output: [[0,0,0,null,null,0,0,null,null,0,0],[0,0,0,null,null,0,0,0,0],[0,0,0,0,0,0,0],[0,0,0,0,0,null,null,null,null,0,0],[0,0,0,0,0,null,null,0,0]]
 *
 * Example 2:
 *
 * Input: n = 3
 * Output: [[0,0,0]]
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 20
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/all-possible-full-binary-trees/
// discuss: https://leetcode.com/problems/all-possible-full-binary-trees/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut cache = std::collections::HashMap::<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>::new();
        Self::dfs_helper(n, &mut cache).clone()
    }

    fn dfs_helper(
        i: i32,
        cache: &mut std::collections::HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> &Vec<Option<Rc<RefCell<TreeNode>>>> {
        if !cache.contains_key(&i) {
            let mut result = vec![];
            if i == 1 {
                result.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
            } else {
                for k in (1..i - 1).step_by(2) {
                    let left = Self::dfs_helper(k, cache).clone();
                    let right = Self::dfs_helper(i - 1 - k, cache);
                    for nl in left {
                        for nr in right {
                            result.push(Some(Rc::new(RefCell::new(TreeNode {
                                val: 0,
                                left: nl.clone(),
                                right: nr.clone(),
                            }))))
                        }
                    }
                }
            }
            cache.insert(i, result.clone());
        }
        return cache.get(&i).unwrap();
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0894_example_1() {
        let n = 7;
        let result = vec![
            tree![0, 0, 0, null, null, 0, 0, null, null, 0, 0],
            tree![0, 0, 0, null, null, 0, 0, 0, 0],
            tree![0, 0, 0, 0, 0, 0, 0],
            tree![0, 0, 0, 0, 0, null, null, null, null, 0, 0],
            tree![0, 0, 0, 0, 0, null, null, 0, 0],
        ];

        assert_eq!(Solution::all_possible_fbt(n), result);
    }

    #[test]
    #[ignore]
    fn test_0894_example_2() {
        let n = 3;
        let result = vec![tree![0, 0, 0]];

        assert_eq!(Solution::all_possible_fbt(n), result);
    }
}

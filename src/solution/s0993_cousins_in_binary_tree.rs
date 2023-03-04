/**
 * [0993] Cousins in Binary Tree
 *
 * Given the root of a binary tree with unique values and the values of two different nodes of the tree x and y, return true if the nodes corresponding to the values x and y in the tree are cousins, or false otherwise.
 * Two nodes of a binary tree are cousins if they have the same depth with different parents.
 * Note that in a binary tree, the root node is at the depth 0, and children of each depth k node are at the depth k + 1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/12/q1248-01.png" style="width: 304px; height: 270px;" />
 * Input: root = [1,2,3,4], x = 4, y = 3
 * Output: false
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/12/q1248-02.png" style="width: 334px; height: 266px;" />
 * Input: root = [1,2,3,null,4,null,5], x = 5, y = 4
 * Output: true
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/02/13/q1248-03.png" style="width: 267px; height: 258px;" />
 * Input: root = [1,2,3,null,4], x = 2, y = 3
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [2, 100].
 * 	1 <= Node.val <= 100
 * 	Each node has a unique value.
 * 	x != y
 * 	x and y are exist in the tree.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/cousins-in-binary-tree/
// discuss: https://leetcode.com/problems/cousins-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut hash_map = std::collections::HashMap::new();
        Self::dfs_helper(&root, &mut hash_map, 0);

        let mut result = false;
        hash_map.values().for_each(|i| {
            let mut xi = -1;
            let mut yi = -1;
            i.iter().enumerate().for_each(|(p, q)| {
                if q == &Some(x) {
                    xi = p as i32;
                } else if q == &Some(y) {
                    yi = p as i32;
                }

                if (xi != -1) && (yi != -1) {
                    if (xi - yi).abs() == 1 {
                        if xi.max(yi) % 2 == 0 {
                            result = true;
                        }
                    } else {
                        result = true;
                    }
                }
            })
        });

        result
    }

    fn dfs_helper(
        root: &Option<Rc<RefCell<TreeNode>>>,
        hash_map: &mut std::collections::HashMap<i32, Vec<Option<i32>>>,
        depth: i32,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            if let Some(v) = hash_map.get_mut(&depth) {
                v.push(Some(r.val));
            } else {
                hash_map.insert(depth, vec![Some(r.val)]);
            }
            Self::dfs_helper(&r.left, hash_map, depth + 1);
            Self::dfs_helper(&r.right, hash_map, depth + 1);
        } else {
            if let Some(v) = hash_map.get_mut(&depth) {
                v.push(None);
            } else {
                hash_map.insert(depth, vec![None]);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0993_example_1() {
        let root = tree![1, 2, 3, 4];
        let x = 4;
        let y = 3;
        let result = false;

        assert_eq!(Solution::is_cousins(root, x, y), result);
    }

    #[test]
    fn test_0993_example_2() {
        let root = tree![1, 2, 3, null, 4, null, 5];
        let x = 5;
        let y = 4;
        let result = true;

        assert_eq!(Solution::is_cousins(root, x, y), result);
    }

    #[test]
    fn test_0993_example_3() {
        let root = tree![1, 2, 3, null, 4];
        let x = 2;
        let y = 3;
        let result = false;

        assert_eq!(Solution::is_cousins(root, x, y), result);
    }
}

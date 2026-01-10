/**
 * [0652] Find Duplicate Subtrees
 *
 * Given the root of a binary tree, return all duplicate subtrees.
 * For each kind of duplicate subtrees, you only need to return the root node of any one of them.
 * Two trees are duplicate if they have the same structure with the same node values.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e1.jpg" style="width: 450px; height: 354px;" />
 * Input: root = [1,2,3,4,null,2,4,null,null,4]
 * Output: [[2,4],[4]]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e2.jpg" style="width: 321px; height: 201px;" />
 * Input: root = [2,1,1]
 * Output: [[1]]
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/16/e33.jpg" style="width: 450px; height: 303px;" />
 * Input: root = [2,2,2,3,null,3,null]
 * Output: [[2,3],[3]]
 *
 *  
 * Constraints:
 *
 * 	The number of the nodes in the tree will be in the range [1, 10^4]
 * 	-200 <= Node.val <= 200
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/find-duplicate-subtrees/
// discuss: https://leetcode.com/problems/find-duplicate-subtrees/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/find-duplicate-subtrees/discuss/852818/Rust-cheapest-and-best
    pub fn find_duplicate_subtrees(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let (mut seen, mut result) = (std::collections::HashMap::new(), vec![]);
        Self::dfs_helper(&mut result, &mut seen, &root, false);
        result
    }

    fn dfs_helper(
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        seen: &mut std::collections::HashMap<Vec<i16>, bool>,
        root: &Option<Rc<RefCell<TreeNode>>>,
        left: bool,
    ) -> Vec<i16> {
        if let Some(node) = root {
            let mut ret = vec![];
            ret.extend(Self::dfs_helper(result, seen, &node.borrow().left, true));
            ret.push(node.borrow().val as i16);
            ret.extend(Self::dfs_helper(result, seen, &node.borrow().right, false));
            if let Some(n) = seen.get_mut(&ret) {
                if *n {
                    result.push(Some(node.clone()));
                    *n = false;
                }
            } else {
                seen.insert(ret.clone(), true);
            }
            ret
        } else {
            vec![if left { i16::MIN } else { i16::MAX }]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0652_example_1() {
        let root = tree![1, 2, 3, 4, null, 2, 4, null, null, 4];
        let result = vec![tree![2, 4], tree![4]];

        assert_eq!(Solution::find_duplicate_subtrees(root), result);
    }

    #[test]
    #[ignore]
    fn test_0652_example_2() {
        let root = tree![2, 1, 1];
        let result = vec![tree![1]];

        assert_eq!(Solution::find_duplicate_subtrees(root), result);
    }

    #[test]
    #[ignore]
    fn test_0652_example_3() {
        let root = tree![2, 2, 2, 3, null, 3, null];
        let result = vec![tree![2, 3], tree![3]];

        assert_eq!(Solution::find_duplicate_subtrees(root), result);
    }
}

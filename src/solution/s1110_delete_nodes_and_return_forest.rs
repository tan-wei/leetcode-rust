/**
 * [1110] Delete Nodes And Return Forest
 *
 * Given the root of a binary tree, each node in the tree has a distinct value.
 * After deleting all nodes with a value in to_delete, we are left with a forest (a disjoint union of trees).
 * Return the roots of the trees in the remaining forest. You may return the result in any order.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/07/01/screen-shot-2019-07-01-at-53836-pm.png" style="width: 237px; height: 150px;" />
 * Input: root = [1,2,3,4,5,6,7], to_delete = [3,5]
 * Output: [[1,2,null,4],[6],[7]]
 *
 * Example 2:
 *
 * Input: root = [1,2,4,null,3], to_delete = [3]
 * Output: [[1,2,4]]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the given tree is at most 1000.
 * 	Each node has a distinct value between 1 and 1000.
 * 	to_delete.length <= 1000
 * 	to_delete contains distinct values between 1 and 1000.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/delete-nodes-and-return-forest/
// discuss: https://leetcode.com/problems/delete-nodes-and-return-forest/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/delete-nodes-and-return-forest/solutions/2676437/rust-recursive/
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut root = root;
        let mut result = Vec::<Option<Rc<RefCell<TreeNode>>>>::new();
        if root.is_none() {
            return result;
        }

        let mut hm = std::collections::HashSet::new();
        for val in to_delete {
            hm.insert(val);
        }

        let mut root = root;
        Self::collect(&mut root, &mut result, &hm);

        let val = root.as_ref().unwrap().borrow().val;
        if hm.contains(&val) == false {
            result.push(root);
        }

        result
    }

    fn collect(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        hm: &std::collections::HashSet<i32>,
    ) {
        let mut node = root.as_ref().unwrap().borrow_mut();

        if node.left.is_some() {
            Self::collect(&mut node.left, result, hm);

            let val = node.left.as_ref().unwrap().borrow().val;

            if hm.contains(&node.val) && hm.contains(&val) == false {
                result.push(node.left.take());
            }
            if hm.contains(&val) {
                node.left = None;
            }
        }

        if node.right.is_some() {
            Self::collect(&mut node.right, result, hm);

            let val = node.right.as_ref().unwrap().borrow().val;

            if hm.contains(&node.val) && hm.contains(&val) == false {
                result.push(node.right.take());
            }
            if hm.contains(&val) {
                node.right = None;
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1110_example_1() {
        let root = tree![1, 2, 3, 4, 5, 6, 7];
        let to_delete = vec![3, 5];
        let result = vec![tree![1, 2, null, 4], tree![6], tree![7]];

        assert_eq!(Solution::del_nodes(root, to_delete), result);
    }

    #[test]
    #[ignore]
    fn test_1110_example_2() {
        let root = tree![1, 2, 4, null, 3];
        let to_delete = vec![3];
        let result = vec![tree![1, 2, 4]];

        assert_eq!(Solution::del_nodes(root, to_delete), result);
    }
}

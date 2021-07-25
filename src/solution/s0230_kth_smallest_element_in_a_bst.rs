/**
 * [230] Kth Smallest Element in a BST
 *
 * Given the root of a binary search tree, and an integer k, return the k^th (1-indexed) smallest element in the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree1.jpg" style="width: 212px; height: 301px;" />
 * Input: root = [3,1,4,null,2], k = 1
 * Output: 1
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/28/kthtree2.jpg" style="width: 382px; height: 302px;" />
 * Input: root = [5,3,6,2,4,null,null,1], k = 3
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is n.
 * 	1 <= k <= n <= 10^4
 * 	0 <= Node.val <= 10^4
 *
 *  
 * Follow up: If the BST is modified often (i.e., we can do insert and delete operations) and you need to find the kth smallest frequently, how would you optimize?
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/kth-smallest-element-in-a-bst/
// discuss: https://leetcode.com/problems/kth-smallest-element-in-a-bst/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs_helper(node: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            node.as_ref().map(|tn| {
                dfs_helper(&tn.borrow().left, nums);
                nums.push(tn.borrow().val);
                dfs_helper(&tn.borrow().right, nums);
            });
        }
        let mut nums = vec![];
        dfs_helper(&root, &mut nums);
        nums[k as usize - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0230_example_1() {
        let root = tree![3, 1, 4, null, 2];
        let k = 1;
        let result = 1;

        assert_eq!(Solution::kth_smallest(root, k), result);
    }

    #[test]
    fn test_0230_example_2() {
        let root = tree![5, 3, 6, 2, 4, null, null, 1];
        let k = 3;
        let result = 3;

        assert_eq!(Solution::kth_smallest(root, k), result);
    }
}

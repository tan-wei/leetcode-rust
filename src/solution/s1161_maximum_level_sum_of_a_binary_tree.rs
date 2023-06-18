/**
 * [1161] Maximum Level Sum of a Binary Tree
 *
 * Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.
 * Return the smallest level x such that the sum of all the values of nodes at level x is maximal.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/03/capture.JPG" style="width: 200px; height: 175px;" />
 * Input: root = [1,7,0,7,-8,null,null]
 * Output: 2
 * Explanation:
 * Level 1 sum = 1.
 * Level 2 sum = 7 + 0 = 7.
 * Level 3 sum = 7 + -8 = -1.
 * So we return the level with the maximum sum which is level 2.
 *
 * Example 2:
 *
 * Input: root = [989,null,10250,98693,-89388,null,null,null,-32127]
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	-10^5 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/
// discuss: https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut list = vec![];
        Self::collect_levels(root, 1, &mut list);
        let (mut max_index, mut max_value) = (0, i32::MIN);

        for i in 0..list.len() {
            if list[i] > max_value {
                max_value = list[i];
                max_index = i + 1;
            }
        }
        max_index as i32
    }

    fn collect_levels(n: Option<Rc<RefCell<TreeNode>>>, level: usize, list: &mut Vec<i32>) {
        match n {
            None => (),
            Some(n) => {
                if level == list.len() + 1 {
                    list.push(n.borrow().val);
                } else {
                    list[level - 1] += n.borrow().val;
                }
                Self::collect_levels(n.borrow().left.clone(), level + 1, list);
                Self::collect_levels(n.borrow().right.clone(), level + 1, list);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1161_example_1() {
        let root = tree![1, 7, 0, 7, -8, null, null];
        let result = 2;

        assert_eq!(Solution::max_level_sum(root), result);
    }
}

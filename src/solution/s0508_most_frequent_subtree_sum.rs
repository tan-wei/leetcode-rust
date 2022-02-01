/**
 * [0508] Most Frequent Subtree Sum
 *
 * Given the root of a binary tree, return the most frequent subtree sum. If there is a tie, return all the values with the highest frequency in any order.
 * The subtree sum of a node is defined as the sum of all the node values formed by the subtree rooted at that node (including the node itself).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/freq1-tree.jpg" style="width: 207px; height: 183px;" />
 * Input: root = [5,2,-3]
 * Output: [2,-3,4]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/24/freq2-tree.jpg" style="width: 207px; height: 183px;" />
 * Input: root = [5,2,-5]
 * Output: [2]
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

// problem: https://leetcode.com/problems/most-frequent-subtree-sum/
// discuss: https://leetcode.com/problems/most-frequent-subtree-sum/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/most-frequent-subtree-sum/discuss/697461/Rust-100-DFS
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(mut node) => {
                let mut max_frequency = 0;
                Solution::dfs_helper(&mut vec![], &mut node)
                    .iter()
                    .fold(std::collections::HashMap::new(), |mut map, cur| {
                        let freq = map.entry(cur).or_insert(0);
                        *freq += 1;
                        if *freq > max_frequency {
                            max_frequency = *freq
                        }
                        map
                    })
                    .iter()
                    .fold(vec![], |mut acc, (k, v)| {
                        if *v == max_frequency {
                            acc.push(**k)
                        }
                        acc
                    })
            }
            None => vec![],
        }
    }

    fn dfs_helper<'a>(sums: &'a mut Vec<i32>, root: &Rc<RefCell<TreeNode>>) -> &'a mut Vec<i32> {
        let mut sum = root.borrow().val;
        if let Some(node) = root.borrow().left.as_ref() {
            Solution::dfs_helper(sums, node);
            sum += sums[sums.len() - 1];
        }
        if let Some(node) = root.borrow().right.as_ref() {
            Solution::dfs_helper(sums, node);
            sum += sums[sums.len() - 1];
        }
        sums.push(sum);
        sums
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0508_example_1() {
        let root = tree![5, 2, -3];
        let result = vec![2, -3, 4];
    }

    #[test]
    fn test_0508_example_2() {
        let root = tree![5, 2, -5];
        let result = vec![2];
    }
}

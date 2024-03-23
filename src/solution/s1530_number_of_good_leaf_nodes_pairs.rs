/**
 * [1530] Number of Good Leaf Nodes Pairs
 *
 * You are given the root of a binary tree and an integer distance. A pair of two different leaf nodes of a binary tree is said to be good if the length of the shortest path between them is less than or equal to distance.
 * Return the number of good leaf node pairs in the tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/09/e1.jpg" style="width: 250px; height: 250px;" />
 * Input: root = [1,2,3,null,4], distance = 3
 * Output: 1
 * Explanation: The leaf nodes of the tree are 3 and 4 and the length of the shortest path between them is 3. This is the only good pair.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/09/e2.jpg" style="width: 250px; height: 182px;" />
 * Input: root = [1,2,3,4,5,6,7], distance = 3
 * Output: 2
 * Explanation: The good pairs are [4,5] and [6,7] with shortest path = 2. The pair [4,6] is not good because the length of ther shortest path between them is 4.
 *
 * Example 3:
 *
 * Input: root = [7,1,4,6,null,5,3,null,null,null,null,null,2], distance = 3
 * Output: 1
 * Explanation: The only good pair is [2,5].
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 2^10].
 * 	1 <= Node.val <= 100
 * 	1 <= distance <= 10
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/
// discuss: https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut result = 0;
        Self::dfs_helper(&root, distance, &mut result);
        result
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, distance: i32, ans: &mut i32) -> Vec<i32> {
        match root {
            None => vec![0; distance as usize + 1],
            Some(node) => {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    let mut v = vec![0; distance as usize + 1];
                    v[1] += 1;
                    return v;
                }
                let left = Self::dfs_helper(&node.left, distance, ans);
                let right = Self::dfs_helper(&node.right, distance, ans);
                for i in 0..distance {
                    for j in (0..distance).rev() {
                        if i as i32 + j <= distance {
                            *ans += left[i as usize] * right[j as usize];
                        }
                    }
                }
                let mut v = vec![0; distance as usize + 1];

                for i in (1..v.len() - 1).rev() {
                    v[i + 1] = left[i] + right[i];
                }
                v
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1530_example_1() {
        let root = tree![1, 2, 3, null, 4];
        let distance = 3;

        let result = 1;

        assert_eq!(Solution::count_pairs(root, distance), result);
    }

    #[test]
    fn test_1530_example_2() {
        let root = tree![1, 2, 3, 4, 5, 6, 7];
        let distance = 3;

        let result = 2;

        assert_eq!(Solution::count_pairs(root, distance), result);
    }

    #[test]
    fn test_1530_example_3() {
        let root = tree![7, 1, 4, 6, null, 5, 3, null, null, null, null, null, 2];
        let distance = 3;

        let result = 1;

        assert_eq!(Solution::count_pairs(root, distance), result);
    }
}

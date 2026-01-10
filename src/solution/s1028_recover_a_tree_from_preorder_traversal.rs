/**
 * [1028] Recover a Tree From Preorder Traversal
 *
 * We run a preorder depth-first search (DFS) on the root of a binary tree.
 * At each node in this traversal, we output D dashes (where D is the depth of this node), then we output the value of this node.  If the depth of a node is D, the depth of its immediate child is D + 1.  The depth of the root node is 0.
 * If a node has only one child, that child is guaranteed to be the left child.
 * Given the output traversal of this traversal, recover the tree and return its root.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/04/08/recover-a-tree-from-preorder-traversal.png" style="width: 320px; height: 200px;" />
 * Input: traversal = "1-2--3--4-5--6--7"
 * Output: [1,2,5,3,4,6,7]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/04/11/screen-shot-2019-04-10-at-114101-pm.png" style="width: 256px; height: 250px;" />
 * Input: traversal = "1-2--3---4-5--6---7"
 * Output: [1,2,5,3,null,6,null,4,null,7]
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/04/11/screen-shot-2019-04-10-at-114955-pm.png" style="width: 276px; height: 250px;" />
 * Input: traversal = "1-401--349---90--88"
 * Output: [1,401,null,349,88,90]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the original tree is in the range [1, 1000].
 * 	1 <= Node.val <= 10^9
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/
// discuss: https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vals: Vec<i32> = traversal
            .split('-')
            .filter(|x| x.len() > 0)
            .map(|x| x.parse().unwrap())
            .collect();
        let mut depths: Vec<i32> = traversal
            .split(char::is_numeric)
            .filter(|x| x.len() > 0)
            .map(|x| x.len() as i32)
            .collect();
        depths.insert(0, 0);

        Solution::dfs_helper(&vals, &depths, 0, 0).0
    }

    fn dfs_helper(
        vals: &Vec<i32>,
        depths: &Vec<i32>,
        idx: usize,
        depth: usize,
    ) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
        if idx >= vals.len() || depth as i32 > depths[idx] {
            return (None, idx);
        }

        let child = Some(Rc::new(RefCell::new(TreeNode::new(vals[idx]))));
        let (left, left_idx) = Solution::dfs_helper(vals, depths, idx + 1, depth + 1);
        let mut next_idx = left_idx;
        child.as_ref().unwrap().borrow_mut().left = left;
        if left_idx > idx {
            let (right, right_idx) = Solution::dfs_helper(vals, depths, left_idx, depth + 1);
            next_idx = right_idx;
            child.as_ref().unwrap().borrow_mut().right = right;
        }

        (child, next_idx)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1028_example_1() {
        let traversal = "1-2--3--4-5--6--7".to_string();
        let result = tree![1, 2, 5, 3, 4, 6, 7];

        assert_eq!(Solution::recover_from_preorder(traversal), result);
    }

    #[test]
    fn test_1028_example_2() {
        let traversal = "1-2--3---4-5--6---7".to_string();
        let result = tree![1, 2, 5, 3, null, 6, null, 4, null, 7];

        assert_eq!(Solution::recover_from_preorder(traversal), result);
    }

    #[test]
    fn test_1028_example_3() {
        let traversal = "1-401--349---90--88".to_string();
        let result = tree![1, 401, null, 349, 88, 90];

        assert_eq!(Solution::recover_from_preorder(traversal), result);
    }
}

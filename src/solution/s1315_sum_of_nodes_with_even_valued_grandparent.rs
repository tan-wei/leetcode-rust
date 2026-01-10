/**
 * [1315] Sum of Nodes with Even-Valued Grandparent
 *
 * Given the root of a binary tree, return the sum of values of nodes with an even-valued grandparent. If there are no nodes with an even-valued grandparent, return 0.
 * A grandparent of a node is the parent of its parent if it exists.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/even1-tree.jpg" style="width: 504px; height: 302px;" />
 * Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
 * Output: 18
 * Explanation: The red nodes are the nodes with even-value grandparent while the blue nodes are the even-value grandparents.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/even2-tree.jpg" style="width: 64px; height: 65px;" />
 * Input: root = [1]
 * Output: 0
 *
 *
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^4].
 * 	1 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent/
// discuss: https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::bfs(root, 1, 1)
    }

    fn bfs(root: Option<Rc<RefCell<TreeNode>>>, gp: i32, p: i32) -> i32 {
        let mut result = 0_i32;
        if let Some(root) = root {
            if gp % 2 == 0 {
                result += root.borrow().val;
            }
            result += Solution::bfs(root.borrow().left.clone(), p, root.borrow().val);
            result += Solution::bfs(root.borrow().right.clone(), p, root.borrow().val);
            return result;
        }

        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1315_example_1() {
        let root = tree![6, 7, 8, 2, 7, 1, 3, 9, null, 1, 4, null, null, null, 5];
        let result = 18;

        assert_eq!(Solution::sum_even_grandparent(root), result);
    }

    #[test]
    fn test_1315_example_2() {
        let root = tree![1];
        let result = 0;

        assert_eq!(Solution::sum_even_grandparent(root), result);
    }
}

/**
 * [0987] Vertical Order Traversal of a Binary Tree
 *
 * Given the root of a binary tree, calculate the vertical order traversal of the binary tree.
 * For each node at position (row, col), its left and right children will be at positions (row + 1, col - 1) and (row + 1, col + 1) respectively. The root of the tree is at (0, 0).
 * The vertical order traversal of a binary tree is a list of top-to-bottom orderings for each column index starting from the leftmost column and ending on the rightmost column. There may be multiple nodes in the same row and same column. In such a case, sort these nodes by their values.
 * Return the vertical order traversal of the binary tree.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/29/vtree1.jpg" style="width: 431px; height: 304px;" />
 * Input: root = [3,9,20,null,null,15,7]
 * Output: [[9],[3,15],[20],[7]]
 * Explanation:
 * Column -1: Only node 9 is in this column.
 * Column 0: Nodes 3 and 15 are in this column in that order from top to bottom.
 * Column 1: Only node 20 is in this column.
 * Column 2: Only node 7 is in this column.
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/29/vtree2.jpg" style="width: 512px; height: 304px;" />
 * Input: root = [1,2,3,4,5,6,7]
 * Output: [[4],[2],[1,5,6],[3],[7]]
 * Explanation:
 * Column -2: Only node 4 is in this column.
 * Column -1: Only node 2 is in this column.
 * Column 0: Nodes 1, 5, and 6 are in this column.
 *           1 is at the top, so it comes first.
 *           5 and 6 are at the same position (2, 0), so we order them by their value, 5 before 6.
 * Column 1: Only node 3 is in this column.
 * Column 2: Only node 7 is in this column.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/29/vtree3.jpg" style="width: 512px; height: 304px;" />
 * Input: root = [1,2,3,4,6,5,7]
 * Output: [[4],[2],[1,5,6],[3],[7]]
 * Explanation:
 * This case is the exact same as example 2, but with nodes 5 and 6 swapped.
 * Note that the solution remains the same since 5 and 6 are in the same location and should be ordered by their values.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	0 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/
// discuss: https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/777715/rust-dfs-solution/
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut btm: std::collections::BTreeMap<
            i32,
            std::collections::BinaryHeap<std::cmp::Reverse<(i32, i32)>>,
        > = std::collections::BTreeMap::new();
        Solution::dfs_helper(&root, (0, 0), &mut btm);
        let mut result: Vec<Vec<i32>> = Vec::new();
        for bh in btm.values_mut() {
            let mut v: Vec<i32> = Vec::new();
            while let Some(r) = bh.pop() {
                v.push((r.0).1);
            }
            result.push(v);
        }
        result
    }

    fn dfs_helper(
        node: &Option<Rc<RefCell<TreeNode>>>,
        pos: (i32, i32),
        btm: &mut std::collections::BTreeMap<
            i32,
            std::collections::BinaryHeap<std::cmp::Reverse<(i32, i32)>>,
        >,
    ) {
        if let Some(n) = node {
            btm.entry(pos.0)
                .or_insert_with(std::collections::BinaryHeap::new)
                .push(std::cmp::Reverse((pos.1, n.borrow().val)));
            Solution::dfs_helper(&n.borrow().left, (pos.0 - 1, pos.1 + 1), btm);
            Solution::dfs_helper(&n.borrow().right, (pos.0 + 1, pos.1 + 1), btm);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0987_example_1() {
        let root = tree![3, 9, 20, null, null, 15, 7];
        let result = vec![vec![9], vec![3, 15], vec![20], vec![7]];

        assert_eq!(Solution::vertical_traversal(root), result);
    }

    #[test]
    fn test_0987_example_2() {
        let root = tree![1, 2, 3, 4, 5, 6, 7];
        let result = vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]];

        assert_eq!(Solution::vertical_traversal(root), result);
    }

    #[test]
    fn test_0987_example_3() {
        let root = tree![1, 2, 3, 4, 6, 5, 7];
        let result = vec![vec![4], vec![2], vec![1, 5, 6], vec![3], vec![7]];

        assert_eq!(Solution::vertical_traversal(root), result);
    }
}

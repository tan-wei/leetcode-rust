/**
 * [1145] Binary Tree Coloring Game
 *
 * Two players play a turn based game on a binary tree. We are given the root of this binary tree, and the number of nodes n in the tree. n is odd, and each node has a distinct value from 1 to n.
 * Initially, the first player names a value x with 1 <= x <= n, and the second player names a value y with 1 <= y <= n and y != x. The first player colors the node with value x red, and the second player colors the node with value y blue.
 * Then, the players take turns starting with the first player. In each turn, that player chooses a node of their color (red if player 1, blue if player 2) and colors an uncolored neighbor of the chosen node (either the left child, right child, or parent of the chosen node.)
 * If (and only if) a player cannot choose such a node in this way, they must pass their turn. If both players pass their turn, the game ends, and the winner is the player that colored more nodes.
 * You are the second player. If it is possible to choose such a y to ensure you win the game, return true. If it is not possible, return false.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/08/01/1480-binary-tree-coloring-game.png" style="width: 500px; height: 310px;" />
 * Input: root = [1,2,3,4,5,6,7,8,9,10,11], n = 11, x = 3
 * Output: true
 * Explanation: The second player can choose the node with value 2.
 *
 * Example 2:
 *
 * Input: root = [1,2,3], n = 3, x = 1
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is n.
 * 	1 <= x <= n <= 100
 * 	n is odd.
 * 	1 <= Node.val <= n
 * 	All the values of the tree are unique.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/binary-tree-coloring-game/
// discuss: https://leetcode.com/problems/binary-tree-coloring-game/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let mut count = 0;
        Self::dfs_helper(&root, &mut count, n, x)
    }

    fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32, n: i32, x: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let node = root.as_ref().unwrap().borrow();
        let (mut count1, mut count2) = (0, 0);
        let ret1 = Self::dfs_helper(&node.left, &mut count1, n, x);
        let ret2 = Self::dfs_helper(&node.right, &mut count2, n, x);
        if ret1 || ret2 {
            return true;
        }
        *count = 1 + count1 + count2;
        if node.val == x {
            return 2 * count1 > n || 2 * count2 > n || 2 * *count < n;
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1145_example_1() {
        let root = tree![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let n = 11;
        let x = 3;
        let result = true;

        assert_eq!(Solution::btree_game_winning_move(root, n, x), result);
    }

    #[test]
    fn test_1145_example_2() {
        let root = tree![1, 2, 3];
        let n = 3;
        let x = 1;
        let result = false;

        assert_eq!(Solution::btree_game_winning_move(root, n, x), result);
    }
}

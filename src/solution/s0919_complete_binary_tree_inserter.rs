/**
 * [0919] Complete Binary Tree Inserter
 *
 * A complete binary tree is a binary tree in which every level, except possibly the last, is completely filled, and all nodes are as far left as possible.
 * Design an algorithm to insert a new node to a complete binary tree keeping it complete after the insertion.
 * Implement the CBTInserter class:
 *
 * 	CBTInserter(TreeNode root) Initializes the data structure with the root of the complete binary tree.
 * 	int insert(int v) Inserts a TreeNode into the tree with value Node.val == val so that the tree remains complete, and returns the value of the parent of the inserted TreeNode.
 * 	TreeNode get_root() Returns the root node of the tree.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/03/lc-treeinsert.jpg" style="width: 500px; height: 143px;" />
 * Input
 * ["CBTInserter", "insert", "insert", "get_root"]
 * [[[1, 2]], [3], [4], []]
 * Output
 * [null, 1, 2, [1, 2, 3, 4]]
 * Explanation
 * CBTInserter cBTInserter = new CBTInserter([1, 2]);
 * cBTInserter.insert(3);  // return 1
 * cBTInserter.insert(4);  // return 2
 * cBTInserter.get_root(); // return [1, 2, 3, 4]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree will be in the range [1, 1000].
 * 	0 <= Node.val <= 5000
 * 	root is a complete binary tree.
 * 	0 <= val <= 5000
 * 	At most 10^4 calls will be made to insert and get_root.
 *
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/complete-binary-tree-inserter/
// discuss: https://leetcode.com/problems/complete-binary-tree-inserter/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::collections::VecDeque;
use std::rc::Rc;

struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    q: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut all_nodes = Vec::new();
        let mut q = Vec::new();
        all_nodes.push(root.clone().unwrap()); //root肯定不会为空
        while all_nodes.len() > 0 {
            let r = all_nodes.remove(0);
            let left = r.borrow().left.clone();
            let right = r.borrow().right.clone();
            if left.is_none() || right.is_none() {
                q.push(r.clone());
            }
            if left.is_some() {
                all_nodes.push(left.unwrap());
            }
            if right.is_some() {
                all_nodes.push(right.unwrap());
            }
        }
        CBTInserter { root, q }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let new_node = Rc::new(RefCell::new(TreeNode {
            val: val,
            left: None,
            right: None,
        }));
        let f = self.q[0].clone();
        let val = f.borrow().val;
        if f.borrow().left.is_none() {
            f.borrow_mut().left = Some(new_node.clone());
        } else if f.borrow_mut().right.is_none() {
            f.borrow_mut().right = Some(new_node.clone());
            //右边有子节点,说明满了,移除
            self.q.remove(0);
        }
        self.q.push(new_node);
        val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }
}

/**
 * Your CBTInserter object will be instantiated and called as such:
 * let obj = CBTInserter::new(root);
 * let ret_1: i32 = obj.insert(val);
 * let ret_2: Option<Rc<RefCell<TreeNode>>> = obj.get_root();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0919_example_1() {
        let mut c_bt_inserter = CBTInserter::new(tree![1, 2]);
        assert_eq!(c_bt_inserter.insert(3), 1); // return 1
        assert_eq!(c_bt_inserter.insert(4), 2); // return 2
        assert_eq!(c_bt_inserter.get_root(), tree![1, 2, 3, 4]); // return [1, 2, 3, 4]
    }
}

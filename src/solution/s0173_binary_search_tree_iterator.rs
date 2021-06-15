/**
 * [173] Binary Search Tree Iterator
 *
 * Implement the BSTIterator class that represents an iterator over the <a href="https://en.wikipedia.org/wiki/Tree_traversal#In-order_(LNR)" target="_blank">in-order traversal</a> of a binary search tree (BST):
 *
 * 	BSTIterator(TreeNode root) Initializes an object of the BSTIterator class. The root of the BST is given as part of the constructor. The pointer should be initialized to a non-existent number smaller than any element in the BST.
 * 	boolean hasNext() Returns true if there exists a number in the traversal to the right of the pointer, otherwise returns false.
 * 	int next() Moves the pointer to the right, then returns the number at the pointer.
 *
 * Notice that by initializing the pointer to a non-existent smallest number, the first call to next() will return the smallest element in the BST.
 * You may assume that next() calls will always be valid. That is, there will be at least a next number in the in-order traversal when next() is called.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/25/bst-tree.png" style="width: 189px; height: 178px;" />
 * Input
 * ["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
 * [[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
 * Output
 * [null, 3, 7, true, 9, true, 15, true, 20, false]
 * Explanation
 * BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
 * bSTIterator.next();    // return 3
 * bSTIterator.next();    // return 7
 * bSTIterator.hasNext(); // return True
 * bSTIterator.next();    // return 9
 * bSTIterator.hasNext(); // return True
 * bSTIterator.next();    // return 15
 * bSTIterator.hasNext(); // return True
 * bSTIterator.next();    // return 20
 * bSTIterator.hasNext(); // return False
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [1, 10^5].
 * 	0 <= Node.val <= 10^6
 * 	At most 10^5 calls will be made to hasNext, and next.
 *
 *  
 * Follow up:
 *
 * 	Could you implement next() and hasNext() to run in average O(1) time and use O(h) memory, where h is the height of the tree?
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-search-tree-iterator/
// discuss: https://leetcode.com/problems/binary-search-tree-iterator/discuss/?currentPage=1&orderBy=most_votes&query=

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

struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn go_left(stack: &mut Vec<Rc<RefCell<TreeNode>>>, mut node: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(current) = node {
            stack.push(Rc::clone(&current));
            current.borrow_mut();
            node = current.borrow().left.as_ref().map(|l| Rc::clone(l));
        }
    }

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = vec![];
        Self::go_left(&mut stack, root);
        Self { stack }
    }
    fn next(&mut self) -> i32 {
        let nxt_ref = self.stack.pop().unwrap();
        let nxt = nxt_ref.borrow();
        if let Some(right) = nxt.right.as_ref() {
            Self::go_left(&mut self.stack, Some(Rc::clone(right)));
        }
        nxt.val
    }
    fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0173_example_1() {
        let mut bst_iterator = BSTIterator::new(tree![7, 3, 15, null, null, 9, 20]);

        assert_eq!(bst_iterator.next(), 3); // return 3
        assert_eq!(bst_iterator.next(), 7); // return 7
        assert_eq!(bst_iterator.has_next(), true); // return True
        assert_eq!(bst_iterator.next(), 9); // return 9
        assert_eq!(bst_iterator.has_next(), true); // return True
        assert_eq!(bst_iterator.next(), 15); // return 15
        assert_eq!(bst_iterator.has_next(), true); // return True
        assert_eq!(bst_iterator.next(), 20); // return 20
        assert_eq!(bst_iterator.has_next(), false); // return False
    }
}

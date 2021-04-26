/**
 * [109] Convert Sorted List to Binary Search Tree
 *
 * Given the head of a singly linked list where elements are sorted in ascending order, convert it to a height balanced BST.
 * For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of every node never differ by more than 1.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/17/linked.jpg" style="width: 600px; height: 466px;" />
 * Input: head = [-10,-3,0,5,9]
 * Output: [0,-3,9,-10,null,5]
 * Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
 *
 * Example 2:
 *
 * Input: head = []
 * Output: []
 *
 * Example 3:
 *
 * Input: head = [0]
 * Output: [0]
 *
 * Example 4:
 *
 * Input: head = [1,3]
 * Output: [3,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in head is in the range [0, 2 * 10^4].
 * 	-10^5 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/
// discuss: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
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
    // Credit: https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/discuss/448982/Rust-O(logN)-Space-and-O(N)-Time
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ptr = &head;
        let mut cnt = 0;
        while let Some(boxed) = ptr {
            cnt += 1;
            ptr = &boxed.next
        }
        let mut ptr = &head;
        Self::helper(&mut ptr, cnt)
    }

    fn helper(ptr: &mut &Option<Box<ListNode>>, len: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if len == 0 {
            return None;
        }
        let left = Self::helper(ptr, len / 2);
        let boxed = ptr.as_ref().unwrap();
        let mut node = TreeNode::new(boxed.val);
        node.left = left;
        *ptr = &boxed.next;
        node.right = Self::helper(ptr, len - len / 2 - 1);
        Some(Rc::new(RefCell::new(node)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0109_example_1() {
        let head = linked![-10, -3, 0, 5, 9];
        let result = tree![0, -3, 9, -10, null, 5];

        assert_eq!(Solution::sorted_list_to_bst(head), result);
    }

    #[test]
    fn test_0109_example_2() {
        let head = linked![];
        let result = tree![];

        assert_eq!(Solution::sorted_list_to_bst(head), result);
    }

    #[test]
    fn test_0109_example_3() {
        let head = linked![0];
        let result = tree![0];

        assert_eq!(Solution::sorted_list_to_bst(head), result);
    }

    #[test]
    fn test_0109_example_4() {
        let head = linked![1, 3];
        let result = tree![3, 1];

        assert_eq!(Solution::sorted_list_to_bst(head), result);
    }
}

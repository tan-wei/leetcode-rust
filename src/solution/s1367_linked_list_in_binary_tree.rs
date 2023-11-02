/**
 * [1367] Linked List in Binary Tree
 *
 * Given a binary tree root and a linked list with head as the first node.
 * Return True if all the elements in the linked list starting from the head correspond to some downward path connected in the binary tree otherwise return False.
 * In this context downward path means a path that starts at some node and goes downwards.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/12/sample_1_1720.png" style="width: 220px; height: 280px;" />
 *
 * Input: head = [4,2,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
 * Output: true
 * Explanation: Nodes in blue form a subpath in the binary Tree.  
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/12/sample_2_1720.png" style="width: 220px; height: 280px;" />
 *
 * Input: head = [1,4,2,6], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
 * Output: true
 *
 * Example 3:
 *
 * Input: head = [1,4,2,6,8], root = [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]
 * Output: false
 * Explanation: There is no path in the binary tree that contains all the elements of the linked list from head.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree will be in the range [1, 2500].
 * 	The number of nodes in the list will be in the range [1, 100].
 * 	1 <= Node.val <= 100 for each node in the linked list and binary tree.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/linked-list-in-binary-tree/
// discuss: https://leetcode.com/problems/linked-list-in-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let list_node = head.unwrap();
        let tree_node = root.unwrap();

        Self::dfs_helper(&list_node, &list_node, tree_node)
    }
    fn dfs_helper(
        list_head: &ListNode,
        list_node: &ListNode,
        tree_node: Rc<RefCell<TreeNode>>,
    ) -> bool {
        let (value, left, right) = {
            let node = tree_node.borrow();
            (node.val, node.left.clone(), node.right.clone())
        };

        if list_node.val == value {
            let next_list_node = match list_node.next.as_ref() {
                None => return true,
                Some(next) => next,
            };

            if let Some(child) = left.clone() {
                if Self::dfs_helper(list_head, next_list_node, child) {
                    return true;
                }
            }

            if let Some(child) = right.clone() {
                if Self::dfs_helper(list_head, next_list_node, child) {
                    return true;
                }
            }
        }

        if std::ptr::eq(list_head, list_node) {
            if let Some(child) = left {
                if Self::dfs_helper(list_head, list_head, child) {
                    return true;
                }
            }

            if let Some(child) = right {
                if Self::dfs_helper(list_head, list_head, child) {
                    return true;
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1367_example_1() {
        let head = linked![4, 2, 8];
        let root = tree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3];

        let result = true;

        assert_eq!(Solution::is_sub_path(head, root), result);
    }

    #[test]
    fn test_1367_example_2() {
        let head = linked![1, 4, 2, 6];
        let root = tree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3];

        let result = true;

        assert_eq!(Solution::is_sub_path(head, root), result);
    }

    #[test]
    fn test_1367_example_3() {
        let head = linked![1, 4, 2, 6, 8];
        let root = tree![1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3];

        let result = false;

        assert_eq!(Solution::is_sub_path(head, root), result);
    }
}

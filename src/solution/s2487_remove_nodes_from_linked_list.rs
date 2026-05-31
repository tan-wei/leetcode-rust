/**
 * [2487] Remove Nodes From Linked List
 *
 * You are given the head of a linked list.
 * Remove every node which has a node with a greater value anywhere to the right side of it.
 * Return the head of the modified linked list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/10/02/drawio.png" style="width: 631px; height: 51px;" />
 * Input: head = [5,2,13,3,8]
 * Output: [13,8]
 * Explanation: The nodes that should be removed are 5, 2 and 3.
 * - Node 13 is to the right of node 5.
 * - Node 13 is to the right of node 2.
 * - Node 8 is to the right of node 3.
 *
 * Example 2:
 *
 * Input: head = [1,1,1,1]
 * Output: [1,1,1,1]
 * Explanation: Every node has value 1, so no nodes are removed.
 *
 *  
 * Constraints:
 *
 * 	The number of the nodes in the given list is in the range [1, 10^5].
 * 	1 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-nodes-from-linked-list/
// discuss: https://leetcode.com/problems/remove-nodes-from-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        Self::dfs_helper(head.as_mut());
        head
    }
    fn dfs_helper(node: Option<&mut Box<ListNode>>) -> i32 {
        if let Some(node) = node {
            if Self::dfs_helper(node.next.as_mut()) > node.val {
                *node = node.next.take().unwrap();
            }
            node.val
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2487_example_1() {
        let head = linked![5, 2, 13, 3, 8];

        let result = linked![13, 8];

        assert_eq!(Solution::remove_nodes(head), result);
    }

    #[test]
    fn test_2487_example_2() {
        let head = linked![1, 1, 1, 1];

        let result = linked![1, 1, 1, 1];

        assert_eq!(Solution::remove_nodes(head), result);
    }
}

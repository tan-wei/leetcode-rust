/**
 * [1721] Swapping Nodes in a Linked List
 *
 * You are given the head of a linked list, and an integer k.
 * Return the head of the linked list after swapping the values of the k^th node from the beginning and the k^th node from the end (the list is 1-indexed).
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/21/linked1.jpg" style="width: 400px; height: 112px;" />
 * Input: head = [1,2,3,4,5], k = 2
 * Output: [1,4,3,2,5]
 *
 * Example 2:
 *
 * Input: head = [7,9,6,6,7,8,3,0,9,5], k = 5
 * Output: [7,9,6,6,8,7,3,0,9,5]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is n.
 * 	1 <= k <= n <= 10^5
 * 	0 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/swapping-nodes-in-a-linked-list/
// discuss: https://leetcode.com/problems/swapping-nodes-in-a-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = Vec::new();
        {
            let mut node = &head;
            while let Some(n) = node {
                v.push(n.val);
                node = &n.next;
            }
        }

        let len = v.len();
        v.swap(k as usize - 1, len - k as usize);

        let mut result = None;
        for &n in v.iter().rev() {
            result = Some(Box::new(ListNode {
                val: n,
                next: result,
            }));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1721_example_1() {
        let head = linked![1, 2, 3, 4, 5];
        let k = 2;

        let result = linked![1, 4, 3, 2, 5];

        assert_eq!(Solution::swap_nodes(head, k), result);
    }

    #[test]
    fn test_1721_example_2() {
        let head = linked![7, 9, 6, 6, 7, 8, 3, 0, 9, 5];
        let k = 5;

        let result = linked![7, 9, 6, 6, 8, 7, 3, 0, 9, 5];

        assert_eq!(Solution::swap_nodes(head, k), result);
    }
}

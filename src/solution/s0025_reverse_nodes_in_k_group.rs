/**
 * [25] Reverse Nodes in k-Group
 *
 * Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.
 * k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
 * Follow up:
 *
 * 	Could you solve the problem in O(1) extra memory space?
 * 	You may not alter the values in the list's nodes, only nodes itself may be changed.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], k = 2
 * Output: [2,1,4,3,5]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], k = 3
 * Output: [3,2,1,4,5]
 *
 * Example 3:
 *
 * Input: head = [1,2,3,4,5], k = 1
 * Output: [1,2,3,4,5]
 *
 * Example 4:
 *
 * Input: head = [1], k = 1
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range sz.
 * 	1 <= sz <= 5000
 * 	0 <= Node.val <= 1000
 * 	1 <= k <= sz
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/reverse-nodes-in-k-group/
// discuss: https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/reverse-nodes-in-k-group/discuss/222209/Rust-4-ms
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut head = dummy.as_mut();
        'outer: loop {
            let mut start = head.as_mut().unwrap().next.take();
            if start.is_none() {
                break 'outer;
            }
            let mut end = start.as_mut();
            for _ in 0..(k - 1) {
                end = end.unwrap().next.as_mut();
                if end.is_none() {
                    head.as_mut().unwrap().next = start;
                    break 'outer;
                }
            }
            let tail = end.as_mut().unwrap().next.take();
            // BEFORE: head -> start -> 123456... -> end   -> tail
            // AFTER:  head -> end   -> ...654321 -> start -> tail
            let end = Self::reverse(start, tail);
            head.as_mut().unwrap().next = end;
            for _ in 0..k {
                head = head.unwrap().next.as_mut()
            }
        }
        dummy.unwrap().next
    }

    #[inline(always)]
    fn reverse(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = tail;
        let mut current = head;
        while let Some(mut current_node_inner) = current {
            let next = current_node_inner.next.take();
            current_node_inner.next = prev.take();
            prev = Some(current_node_inner);
            current = next;
        }
        prev
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0025_example_1() {
        let head = linked![1, 2, 3, 4, 5];
        let k = 2;
        let result = linked![2, 1, 4, 3, 5];

        assert_eq!(Solution::reverse_k_group(head, k), result);
    }

    #[test]
    fn test_0025_example_2() {
        let head = linked![1, 2, 3, 4, 5];
        let k = 3;
        let result = linked![3, 2, 1, 4, 5];

        assert_eq!(Solution::reverse_k_group(head, k), result);
    }

    #[test]
    fn test_0025_example_3() {
        let head = linked![1, 2, 3, 4, 5];
        let k = 1;
        let result = linked![1, 2, 3, 4, 5];

        assert_eq!(Solution::reverse_k_group(head, k), result);
    }

    #[test]
    fn test_0025_example_4() {
        let head = linked![1];
        let k = 2;
        let result = linked![1];

        assert_eq!(Solution::reverse_k_group(head, k), result);
    }
}

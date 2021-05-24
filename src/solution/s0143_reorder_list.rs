/**
 * [143] Reorder List
 *
 * You are given the head of a singly linked-list. The list can be represented as:
 *
 * L0 &rarr; L1 &rarr; &hellip; &rarr; Ln - 1 &rarr; Ln
 *
 * Reorder the list to be on the following form:
 *
 * L0 &rarr; Ln &rarr; L1 &rarr; Ln - 1 &rarr; L2 &rarr; Ln - 2 &rarr; &hellip;
 *
 * You may not modify the values in the list's nodes. Only nodes themselves may be changed.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/04/reorder1linked-list.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [1,4,2,3]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/09/reorder2-linked-list.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [1,5,2,4,3]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [1, 5 * 10^4].
 * 	1 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/reorder-list/
// discuss: https://leetcode.com/problems/reorder-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/reorder-list/discuss/803111/Rust-4ms
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let len = Self::length(&head);

        let mut ptr = head.as_mut();
        for _ in 0..(len / 2) {
            if let Some(node) = ptr {
                ptr = node.next.as_mut();
            }
        }

        if let Some(node) = ptr {
            let reverse = Self::reverse(node.next.take(), None);

            if let Some(node) = head {
                node.next = Self::merge(reverse, node.next.take(), len - 1);
            }
        }
    }

    fn length(mut head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }
        count
    }

    fn reverse(
        head: Option<Box<ListNode>>,
        accumulator: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => accumulator,
            Some(mut node) => {
                let next = node.next;
                node.next = accumulator;

                Self::reverse(next, Some(node))
            }
        }
    }

    fn merge(
        mut left: Option<Box<ListNode>>,
        right: Option<Box<ListNode>>,
        count: usize,
    ) -> Option<Box<ListNode>> {
        if count == 0 {
            None
        } else {
            match (left.as_mut(), right.as_ref()) {
                (None, None) => None,
                (Some(_), None) => left,
                (None, Some(_)) => right,
                (Some(l), Some(_)) => {
                    l.next = Self::merge(right, l.next.take(), count - 1);
                    left
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0143_example_1() {
        let mut head = linked![1, 2, 3, 4];
        let result = linked![1, 4, 2, 3];

        Solution::reorder_list(&mut head);

        assert_eq!(head, result);
    }

    #[test]
    fn test_0143_example_2() {
        let mut head = linked![1, 2, 3, 4, 5];
        let result = linked![1, 5, 2, 4, 3];

        Solution::reorder_list(&mut head);

        assert_eq!(head, result);
    }
}

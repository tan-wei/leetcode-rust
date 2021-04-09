/**
 * [92] Reverse Linked List II
 *
 * Given the head of a singly linked list and two integers left and right where left <= right, reverse the nodes of the list from position left to position right, and return the reversed list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev2ex2.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], left = 2, right = 4
 * Output: [1,4,3,2,5]
 *
 * Example 2:
 *
 * Input: head = [5], left = 1, right = 1
 * Output: [5]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is n.
 * 	1 <= n <= 500
 * 	-500 <= Node.val <= 500
 * 	1 <= left <= right <= n
 *
 *  
 * Follow up: Could you do it in one pass?
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/reverse-linked-list-ii/
// discuss: https://leetcode.com/problems/reverse-linked-list-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/reverse-linked-list-ii/discuss/808421/Rust-solution-no-stack-0ms.
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == 1 {
            return Self::reverse(head, None, right - left + 1);
        }

        let mut count = 1;
        let mut head = head;
        let mut current = head.as_mut();
        while let Some(node) = current {
            count += 1;

            if count == left {
                node.next = Self::reverse(node.next.take(), None, right - left + 1);
                break;
            } else {
                current = node.next.as_mut();
            }
        }

        head
    }

    fn reverse(
        head: Option<Box<ListNode>>,
        acc: Option<Box<ListNode>>,
        count: i32,
    ) -> Option<Box<ListNode>> {
        if count == 0 {
            return Self::append(acc, head);
        }

        if let Some(mut node) = head {
            let next = node.next;
            node.next = acc;

            Self::reverse(next, Some(node), count - 1)
        } else {
            acc
        }
    }

    fn append(
        mut front: Option<Box<ListNode>>,
        back: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = front.as_mut();
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = back;
                break;
            }
            current = node.next.as_mut();
        }

        front
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0092_example_1() {
        let head = linked![1, 2, 3, 4, 5];
        let left = 2;
        let right = 4;
        let result = linked![1, 4, 3, 2, 5];

        assert_eq!(Solution::reverse_between(head, left, right), result);
    }

    #[test]
    fn test_0092_example_2() {
        let head = linked![5];
        let left = 1;
        let right = 1;
        let result = linked![5];

        assert_eq!(Solution::reverse_between(head, left, right), result);
    }
}

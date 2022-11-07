/**
 * [0876] Middle of the Linked List
 *
 * Given the head of a singly linked list, return the middle node of the linked list.
 * If there are two middle nodes, return the second middle node.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-midlist1.jpg" style="width: 544px; height: 65px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [3,4,5]
 * Explanation: The middle node of the list is node 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-midlist2.jpg" style="width: 664px; height: 65px;" />
 * Input: head = [1,2,3,4,5,6]
 * Output: [4,5,6]
 * Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [1, 100].
 * 	1 <= Node.val <= 100
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/middle-of-the-linked-list/
// discuss: https://leetcode.com/problems/middle-of-the-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ahead = head.as_deref();

        while let Some(next) = ahead.and_then(|node| node.next.as_deref()) {
            let next = next.next.as_deref().map(std::ptr::NonNull::from);

            head = head.unwrap().next;
            ahead = next.map(|ptr| unsafe { &*ptr.as_ptr() });
        }

        head
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0876_example_1() {
        let head = linked![1, 2, 3, 4, 5];
        let result = linked![3, 4, 5];

        assert_eq!(Solution::middle_node(head), result);
    }

    #[test]
    fn test_0876_example_2() {
        let head = linked![1, 2, 3, 4, 5, 6];
        let result = linked![4, 5, 6];

        assert_eq!(Solution::middle_node(head), result);
    }
}

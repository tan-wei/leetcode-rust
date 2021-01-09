/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
 *
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 *
 * Example 2:
 *
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 *
 * Example 3:
 *
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *
 *  
 * Constraints:
 *
 *
 * 	The number of nodes in each linked list is in the range [1, 100].
 * 	0 <= Node.val <= 9
 * 	It is guaranteed that the list represents a number that does not have leading zeros.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_header = ListNode::new(0);
        let mut cur = &mut dummy_header;

        let mut p = l1.as_ref();
        let mut q = l2.as_ref();
        let mut carry = false;

        while p.is_some() || q.is_some() {
            let p_val = p.map_or(0, |n| n.val);
            let q_val = q.map_or(0, |n| n.val);

            let sum = p_val + q_val + carry as i32;
            carry = sum >= 10;
            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            cur = cur.next.as_mut().unwrap();

            p = p.and_then(|n| n.next.as_ref());
            q = q.and_then(|n| n.next.as_ref());
        }

        if carry {
            cur.next = Some(Box::new(ListNode::new(carry as i32)));
        }

        dummy_header.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let l1 = linked![2, 4, 3];
        let l2 = linked![5, 6, 4];

        assert_eq!(Solution::add_two_numbers(l1, l2), linked![7, 0, 8]);
    }

    #[test]
    fn test_example_2() {
        let l1 = linked![0];
        let l2 = linked![0];

        assert_eq!(Solution::add_two_numbers(l1, l2), linked![0]);
    }

    #[test]
    fn test_example_3() {
        let l1 = linked![9, 9, 9, 9, 9, 9, 9];
        let l2 = linked![9, 9, 9, 9];

        assert_eq!(
            Solution::add_two_numbers(l1, l2),
            linked![8, 9, 9, 9, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(Solution::add_two_numbers(None, linked![]), None);
        assert_eq!(Solution::add_two_numbers(linked![], None), None);
    }
}

/**
 * [0445] Add Two Numbers II
 *
 * You are given two non-empty linked lists representing two non-negative integers. The most significant digit comes first and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/09/sumii-linked-list.jpg" style="width: 523px; height: 342px;" />
 * Input: l1 = [7,2,4,3], l2 = [5,6,4]
 * Output: [7,8,0,7]
 *
 * Example 2:
 *
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [8,0,7]
 *
 * Example 3:
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in each linked list is in the range [1, 100].
 * 	0 <= Node.val <= 9
 * 	It is guaranteed that the list represents a number that does not have leading zeros.
 *
 *  
 * Follow up: Could you solve it without reversing the input lists?
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/add-two-numbers-ii/
// discuss: https://leetcode.com/problems/add-two-numbers-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
        let mut s1 = Vec::<i32>::new();
        let mut s2 = Vec::<i32>::new();
        let mut t1 = &l1;
        while let Some(x) = t1 {
            s1.push(x.val);
            t1 = &x.next;
        }
        let mut t2 = &l2;
        while let Some(x) = t2 {
            s2.push(x.val);
            t2 = &x.next;
        }
        let mut list = ListNode::new(0);
        let mut sum = 0;
        loop {
            match (s1.pop(), s2.pop()) {
                (Some(x1), Some(x2)) => {
                    sum += x1 + x2;
                    list.val = sum % 10;
                    let mut head = ListNode::new(sum / 10);
                    head.next = Some(Box::new(list));
                    list = head;
                    sum /= 10;
                }
                (Some(x1), None) => {
                    sum += x1;
                    list.val = sum % 10;
                    let mut head = ListNode::new(sum / 10);
                    head.next = Some(Box::new(list));
                    list = head;
                    sum /= 10;
                }
                (None, Some(x2)) => {
                    sum += x2;
                    list.val = sum % 10;
                    let mut head = ListNode::new(sum / 10);
                    head.next = Some(Box::new(list));
                    list = head;
                    sum /= 10;
                }
                _ => break,
            }
        }
        if list.val == 0 {
            list.next
        } else {
            Some(Box::new(list))
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0445_example_1() {
        let l1 = linked![7, 2, 4, 3];
        let l2 = linked![5, 6, 4];
        let result = linked![7, 8, 0, 7];

        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }

    #[test]
    fn test_0445_example_2() {
        let l1 = linked![2, 4, 3];
        let l2 = linked![5, 6, 4];
        let result = linked![8, 0, 7];

        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }

    #[test]
    fn test_0445_example_3() {
        let l1 = linked![0];
        let l2 = linked![0];
        let result = linked![0];

        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }
}

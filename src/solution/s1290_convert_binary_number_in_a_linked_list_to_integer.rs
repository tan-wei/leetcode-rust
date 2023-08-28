/*
 * [1290] Convert Binary Number in a Linked List to Integer
 *
 * Given head which is a reference node to a singly-linked list. The value of each node in the linked list is either 0 or 1. The linked list holds the binary representation of a number.
 * Return the decimal value of the number in the linked list.
 * The most significant bit is at the head of the linked list.
 *
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/12/05/graph-1.png" style="width: 426px; height: 108px;" />
 * Input: head = [1,0,1]
 * Output: 5
 * Explanation: (101) in base 2 = (5) in base 10
 *
 * Example 2:
 *
 * Input: head = [0]
 * Output: 0
 *
 *
 * Constraints:
 *
 * 	The Linked List is not empty.
 * 	Number of nodes will not exceed 30.
 * 	Each node's value is either 0 or 1.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
// discuss: https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut next = head;
        let mut bin_arr = Vec::new();
        while let Some(node) = next {
            bin_arr.push(node.val.to_string());
            next = node.next;
        }
        i32::from_str_radix(&bin_arr.join(""), 2).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1290_example_1() {
        let head = linked![1, 0, 1];
        let result = 5;

        assert_eq!(Solution::get_decimal_value(head), result);
    }

    #[test]
    fn test_1290_example_2() {
        let head = linked![0];
        let result = 0;

        assert_eq!(Solution::get_decimal_value(head), result);
    }
}

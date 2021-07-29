/**
 * [234] Palindrome Linked List
 *
 * Given the head of a singly linked list, return true if it is a palindrome.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal1linked-list.jpg" style="width: 422px; height: 62px;" />
 * Input: head = [1,2,2,1]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/03/pal2linked-list.jpg" style="width: 182px; height: 62px;" />
 * Input: head = [1,2]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [1, 10^5].
 * 	0 <= Node.val <= 9
 *
 *  
 * Follow up: Could you do it in O(n) time and O(1) space?
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/palindrome-linked-list/
// discuss: https://leetcode.com/problems/palindrome-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut nums = vec![];
        let mut node = head;

        while let Some(curr) = node {
            nums.push(curr.val);
            node = curr.next;
        }

        let mut nums_rev = nums.to_owned();
        nums_rev.reverse();

        nums == nums_rev
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0234_example_1() {
        let head = linked![1, 2, 2, 1];
        let result = true;

        assert_eq!(Solution::is_palindrome(head), result);
    }

    #[test]
    fn test_0234_example_2() {
        let head = linked![1, 2];
        let result = false;

        assert_eq!(Solution::is_palindrome(head), result);
    }
}

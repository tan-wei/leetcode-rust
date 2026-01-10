/**
 * [82] Remove Duplicates from Sorted List II
 *
 * Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/linkedlist1.jpg" style="width: 500px; height: 142px;" />
 * Input: head = [1,2,3,3,4,4,5]
 * Output: [1,2,5]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/linkedlist2.jpg" style="width: 500px; height: 205px;" />
 * Input: head = [1,1,1,2,3]
 * Output: [2,3]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 300].
 * 	-100 <= Node.val <= 100
 * 	The list is guaranteed to be sorted in ascending order.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/discuss/1127047/Rust-no-allocation-iterative-O(N)-solution
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut to_remove = head.as_ref().unwrap().val - 1;
        let mut dummy = Some(Box::new(ListNode {
            next: head,
            val: to_remove,
        }));
        let mut node = &mut dummy.as_mut().unwrap().next;

        loop {
            match node {
                None => return dummy.unwrap().next,
                Some(n) if n.val == to_remove => *node = n.next.take(),
                Some(n) if n.next.is_some() && n.val == n.next.as_ref().unwrap().val => {
                    to_remove = n.val
                }
                Some(n) => {
                    node = &mut n.next;
                    if let Some(n) = node {
                        to_remove = n.val - 1;
                    }
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
    fn test_0082_example_1() {
        let head = linked![1, 2, 3, 3, 4, 4, 5];
        let result = linked![1, 2, 5];

        assert_eq!(Solution::delete_duplicates(head), result);
    }

    #[test]
    fn test_0082_example_2() {
        let head = linked![1, 1, 1, 2, 3];
        let result = linked![2, 3];

        assert_eq!(Solution::delete_duplicates(head), result);
    }
}

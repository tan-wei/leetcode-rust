/**
 * [83] Remove Duplicates from Sorted List
 *
 * Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/list1.jpg" style="width: 302px; height: 242px;" />
 * Input: head = [1,1,2]
 * Output: [1,2]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/list2.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,1,2,3,3]
 * Output: [1,2,3]
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

// problem: https://leetcode.com/problems/remove-duplicates-from-sorted-list/
// discuss: https://leetcode.com/problems/remove-duplicates-from-sorted-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|head| {
            let head = *head;
            let tail = Self::delete_duplicates(head.next);
            match &tail {
                Some(node) if node.val == head.val => tail,
                _ => Some(Box::new(ListNode {
                    val: head.val,
                    next: tail,
                })),
            }
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0083_example_1() {
        let head = linked![1, 1, 2];
        let result = linked![1, 2];

        assert_eq!(Solution::delete_duplicates(head), result);
    }

    #[test]
    fn test_0083_example_2() {
        let head = linked![1, 1, 2, 3, 3];
        let result = linked![1, 2, 3];

        assert_eq!(Solution::delete_duplicates(head), result);
    }
}

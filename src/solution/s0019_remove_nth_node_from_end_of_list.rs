/**
 * [19] Remove Nth Node From End of List
 *
 * Given the head of a linked list, remove the n^th node from the end of the list and return its head.
 * Follow up: Could you do this in one pass?
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], n = 2
 * Output: [1,2,3,5]
 *
 * Example 2:
 *
 * Input: head = [1], n = 1
 * Output: []
 *
 * Example 3:
 *
 * Input: head = [1,2], n = 1
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is sz.
 * 	1 <= sz <= 30
 * 	0 <= Node.val <= 100
 * 	1 <= n <= sz
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
// discuss: https://leetcode.com/problems/remove-nth-node-from-end-of-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Two pass solution
        let mut dummy = Some(Box::new(ListNode {
            val: Default::default(),
            next: head,
        }));

        let mut len = 0;
        {
            let mut p = dummy.as_ref();
            while p.unwrap().next.is_some() {
                len += 1;
                p = p.unwrap().next.as_ref();
            }
        }

        let idx = len - n;
        {
            let mut p = dummy.as_mut();
            for _ in 0..idx {
                p = p.unwrap().next.as_mut();
            }
            let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = next;
        }
        dummy.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0019_example_1() {
        let head = linked![1, 2, 3, 4, 5];
        let n = 2;
        let result = linked![1, 2, 3, 5];

        assert_eq!(Solution::remove_nth_from_end(head, n), result);
    }

    #[test]
    fn test_0019_example_2() {
        let head = linked![1];
        let n = 1;
        let result = linked![];

        assert_eq!(Solution::remove_nth_from_end(head, n), result);
    }

    #[test]
    fn test_0019_example_3() {
        let head = linked![1, 2];
        let n = 1;
        let result = linked![1];

        assert_eq!(Solution::remove_nth_from_end(head, n), result);
    }
}

/**
 * [206] Reverse Linked List
 *
 * Given the head of a singly linked list, reverse the list, and return the reversed list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [5,4,3,2,1]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg" style="width: 182px; height: 222px;" />
 * Input: head = [1,2]
 * Output: [2,1]
 *
 * Example 3:
 *
 * Input: head = []
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is the range [0, 5000].
 * 	-5000 <= Node.val <= 5000
 *
 *  
 * Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/reverse-linked-list/
// discuss: https://leetcode.com/problems/reverse-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;
        while let Some(mut boxed_node) = cur.take() {
            let next = boxed_node.next.take();
            boxed_node.next = pre.take();
            pre = Some(boxed_node);
            cur = next;
        }
        pre
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0206_example_1() {
        let head = linked![1, 2, 3, 4, 5];
        let result = linked![5, 4, 3, 2, 1];

        assert_eq!(Solution::reverse_list(head), result);
    }

    #[test]
    fn test_0206_example_2() {
        let head = linked![1, 2,];
        let result = linked![2, 1];

        assert_eq!(Solution::reverse_list(head), result);
    }

    #[test]
    fn test_0206_example_3() {
        let head = linked![];
        let result = linked![];

        assert_eq!(Solution::reverse_list(head), result);
    }
}

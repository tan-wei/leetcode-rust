/**
 * [203] Remove Linked List Elements
 *
 * Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/06/removelinked-list.jpg" style="width: 500px; height: 142px;" />
 * Input: head = [1,2,6,3,4,5,6], val = 6
 * Output: [1,2,3,4,5]
 *
 * Example 2:
 *
 * Input: head = [], val = 1
 * Output: []
 *
 * Example 3:
 *
 * Input: head = [7,7,7,7], val = 7
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 10^4].
 * 	1 <= Node.val <= 50
 * 	0 <= val <= 50
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-linked-list-elements/
// discuss: https://leetcode.com/problems/remove-linked-list-elements/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut dummy = ListNode { val: 0, next: None };
        let mut cur = &mut dummy;
        while let Some(mut node) = head {
            head = std::mem::replace(&mut node.next, None);
            if node.val != val {
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0203_example_1() {
        let head = linked![1, 2, 6, 3, 4, 5, 6];
        let val = 6;
        let result = linked![1, 2, 3, 4, 5];

        assert_eq!(Solution::remove_elements(head, val), result);
    }

    #[test]
    fn test_0203_example_2() {
        let head = linked![];
        let val = 1;
        let result = linked![];

        assert_eq!(Solution::remove_elements(head, val), result);
    }

    #[test]
    fn test_0203_example_3() {
        let head = linked![7, 7, 7, 7];
        let val = 7;
        let result = linked![];

        assert_eq!(Solution::remove_elements(head, val), result);
    }
}

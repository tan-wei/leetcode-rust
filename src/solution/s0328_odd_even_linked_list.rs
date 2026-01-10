/**
 * [328] Odd Even Linked List
 *
 * Given the head of a singly linked list, group all the nodes with odd indices together followed by the nodes with even indices, and return the reordered list.
 * The first node is considered odd, and the second node is even, and so on.
 * Note that the relative order inside both the even and odd groups should remain as it was in the input.
 * You must solve the problem in O(1) extra space complexity and O(n) time complexity.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/oddeven-linked-list.jpg" style="width: 300px; height: 123px;" />
 * Input: head = [1,2,3,4,5]
 * Output: [1,3,5,2,4]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/10/oddeven2-linked-list.jpg" style="width: 500px; height: 142px;" />
 * Input: head = [2,1,3,5,6,4,7]
 * Output: [2,3,6,7,1,5,4]
 *
 *  
 * Constraints:
 *
 * 	n == number of nodes in the linked list
 * 	0 <= n <= 10^4
 * 	-10^6 <= Node.val <= 10^6
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/odd-even-linked-list/
// discuss: https://leetcode.com/problems/odd-even-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut odd_dummy_head = ListNode::new(-1);
        let mut even_dummy_head = ListNode::new(-1);
        let mut odd_cur = &mut odd_dummy_head;
        let mut even_cur = &mut even_dummy_head;

        let mut is_even = false;
        while let Some(mut node) = head {
            head = std::mem::replace(&mut node.next, None);
            if is_even {
                even_cur.next = Some(node);
                even_cur = even_cur.next.as_mut().unwrap();
            } else {
                odd_cur.next = Some(node);
                odd_cur = odd_cur.next.as_mut().unwrap();
            }
            is_even = !is_even;
        }

        odd_cur.next = even_dummy_head.next;
        odd_dummy_head.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0328_example_1() {
        let head = linked![1, 2, 3, 4, 5];
        let result = linked![1, 3, 5, 2, 4];

        assert_eq!(Solution::odd_even_list(head), result);
    }

    #[test]
    fn test_0328_example_2() {
        let head = linked![2, 1, 3, 5, 6, 4, 7];
        let result = linked![2, 3, 6, 7, 1, 5, 4];

        assert_eq!(Solution::odd_even_list(head), result);
    }
}

/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/swap_ex1.jpg" style="width: 422px; height: 222px;" />
 * Input: head = [1,2,3,4]
 * Output: [2,1,4,3]
 *
 * Example 2:
 *
 * Input: head = []
 * Output: []
 *
 * Example 3:
 *
 * Input: head = [1]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 100].
 * 	0 <= Node.val <= 100
 *
 *  
 * Follow up: Can you solve the problem without modifying the values in the list's nodes? (i.e., Only nodes themselves may be changed.)
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/swap-nodes-in-pairs/
// discuss: https://leetcode.com/problems/swap-nodes-in-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Recursiver verion
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::swap_pairs_recursive(head)
    }

    fn swap_pairs_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut node| match node.next {
            Some(mut next) => {
                node.next = Solution::swap_pairs_recursive(next.next);
                next.next = Some(node);
                Some(next)
            }
            None => Some(node),
        })
    }

    // Iteration version
    pub fn swap_pairs_v2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut tail = &mut dummy;

        loop {
            let mut front = if tail.next.is_none() {
                break;
            } else {
                tail.next.take().unwrap()
            };
            let mut back = if front.next.is_none() {
                tail.next = Some(front);
                break;
            } else {
                front.next.unwrap()
            };
            front.next = back.next.take();
            back.next = Some(front);
            tail.next = Some(back);
            tail = tail.next.as_mut().unwrap();
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0024_example_1() {
        let head = linked![1, 2, 3, 4];
        let result = linked![2, 1, 4, 3];

        assert_eq!(Solution::swap_pairs(head), result);

        let head = linked![1, 2, 3, 4];
        let result = linked![2, 1, 4, 3];

        assert_eq!(Solution::swap_pairs_v2(head), result);
    }

    #[test]
    fn test_0024_example_2() {
        let head = linked![];
        let result = linked![];

        assert_eq!(Solution::swap_pairs(head), result);

        let head = linked![];
        let result = linked![];

        assert_eq!(Solution::swap_pairs_v2(head), result);
    }

    #[test]
    fn test_0024_example_3() {
        let head = linked![1];
        let result = linked![1];

        assert_eq!(Solution::swap_pairs(head), result);

        let head = linked![1];
        let result = linked![1];

        assert_eq!(Solution::swap_pairs_v2(head), result);
    }
}

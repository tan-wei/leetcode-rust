/**
 * [1171] Remove Zero Sum Consecutive Nodes from Linked List
 *
 * Given the head of a linked list, we repeatedly delete consecutive sequences of nodes that sum to 0 until there are no such sequences.
 *
 * After doing so, return the head of the final linked list.  You may return any such answer.
 *  
 * (Note that in the examples below, all sequences are serializations of ListNode objects.)
 * Example 1:
 *
 * Input: head = [1,2,-3,3,1]
 * Output: [3,1]
 * Note: The answer [1,2,1] would also be accepted.
 *
 * Example 2:
 *
 * Input: head = [1,2,3,-3,4]
 * Output: [1,2,4]
 *
 * Example 3:
 *
 * Input: head = [1,2,3,-3,-2]
 * Output: [1]
 *
 *  
 * Constraints:
 *
 * 	The given linked list will contain between 1 and 1000 nodes.
 * 	Each node in the linked list has -1000 <= node.val <= 1000.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
// discuss: https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let mut tail = Self::remove_zero_sum_sublists(node.next.take());
            if node.val == 0 {
                return tail;
            }
            let mut sum = 0;
            let mut current_ref = tail.as_mut();
            while let Some(node1) = current_ref {
                sum += node1.val;
                if sum == -node.val {
                    return node1.next.take();
                }
                current_ref = node1.next.as_mut();
            }
            node.next = tail;
            Some(node)
        } else {
            None
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1171_example_1() {
        let head = linked![1, 2, -3, 3, 1];
        let result = linked![3, 1];

        assert_eq!(Solution::remove_zero_sum_sublists(head), result);
    }

    #[test]
    #[ignore]
    fn test_1171_example_2() {
        let head = linked![1, 2, 3, -3, 4];
        let result = linked![1, 2, 4];

        assert_eq!(Solution::remove_zero_sum_sublists(head), result);
    }

    #[test]
    #[ignore]
    fn test_1171_example_3() {
        let head = linked![1, 2, 3, -3, -2];
        let result = linked![1];

        assert_eq!(Solution::remove_zero_sum_sublists(head), result);
    }
}

/**
 * [86] Partition List
 *
 * Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.
 * You should preserve the original relative order of the nodes in each of the two partitions.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/partition.jpg" style="width: 662px; height: 222px;" />
 * Input: head = [1,4,3,2,5,2], x = 3
 * Output: [1,2,2,4,3,5]
 *
 * Example 2:
 *
 * Input: head = [2,1], x = 2
 * Output: [1,2]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 200].
 * 	-100 <= Node.val <= 100
 * 	-200 <= x <= 200
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/partition-list/
// discuss: https://leetcode.com/problems/partition-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut dummy = Box::new(ListNode {
            val: std::default::Default::default(),
            next: None,
        });

        let mut after_head = Box::new(ListNode::new(std::default::Default::default()));
        let mut head = head;
        let mut before = &mut dummy;
        let mut after = &mut after_head;

        while let Some(mut node) = head {
            if node.val < x {
                head = node.next.take();
                before.next = Some(node);
                before = before.next.as_mut().unwrap();
            } else {
                head = node.next.take();

                after.next = Some(node);
                after = after.next.as_mut().unwrap();
            }
        }
        after.next = None;

        before.next = after_head.next;
        dummy.next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0086_example_1() {
        let head = linked![1, 4, 3, 2, 5, 2];
        let x = 3;
        let result = linked![1, 2, 2, 4, 3, 5];

        assert_eq!(Solution::partition(head, x), result);
    }

    #[test]
    fn test_0086_example_2() {
        let head = linked![2, 1];
        let x = 2;
        let result = linked![1, 2];

        assert_eq!(Solution::partition(head, x), result);
    }
}

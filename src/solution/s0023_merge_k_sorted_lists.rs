/**
 * [23] Merge k Sorted Lists
 *
 * You are given an array of k linked-lists lists, each linked-list is sorted in ascending order.
 * Merge all the linked-lists into one sorted linked-list and return it.
 *  
 * Example 1:
 *
 * Input: lists = [[1,4,5],[1,3,4],[2,6]]
 * Output: [1,1,2,3,4,4,5,6]
 * Explanation: The linked-lists are:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * merging them into one sorted list:
 * 1->1->2->3->4->4->5->6
 *
 * Example 2:
 *
 * Input: lists = []
 * Output: []
 *
 * Example 3:
 *
 * Input: lists = [[]]
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	k == lists.length
 * 	0 <= k <= 10^4
 * 	0 <= lists[i].length <= 500
 * 	-10^4 <= lists[i][j] <= 10^4
 * 	lists[i] is sorted in ascending order.
 * 	The sum of lists[i].length won't exceed 10^4.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/merge-k-sorted-lists/
// discuss: https://leetcode.com/problems/merge-k-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

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
use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val)
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

impl Solution {
    // Credit: https://leetcode.com/problems/merge-k-sorted-lists/discuss/221538/rust-4ms-no-clone
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for mut node in lists {
            if node.is_some() {
                heap.push(node.take()?)
            }
        }
        let mut head = heap.pop()?;
        let mut pointer = &mut head;
        while !heap.is_empty() {
            if pointer.next.is_some() {
                heap.push(pointer.next.take()?);
            }
            pointer.next = Some(heap.pop()?);
            pointer = pointer.next.as_mut()?;
        }
        Some(head)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0023_example_1() {
        let lists = vec![linked![1, 4, 5], linked![1, 3, 4], linked![2, 6]];
        let result = linked![1, 1, 2, 3, 4, 4, 5, 6];

        assert_eq!(Solution::merge_k_lists(lists), result);
    }

    #[test]
    fn test_0023_example_2() {
        let lists = vec![];
        let result = linked![];

        assert_eq!(Solution::merge_k_lists(lists), result);
    }

    #[test]
    fn test_0023_example_3() {
        let lists = vec![linked![]];
        let result = linked![];

        assert_eq!(Solution::merge_k_lists(lists), result);
    }
}

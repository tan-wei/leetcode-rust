/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the nodes of the first two lists.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg" style="width: 662px; height: 302px;" />
 * Input: l1 = [1,2,4], l2 = [1,3,4]
 * Output: [1,1,2,3,4,4]
 *
 * Example 2:
 *
 * Input: l1 = [], l2 = []
 * Output: []
 *
 * Example 3:
 *
 * Input: l1 = [], l2 = [0]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in both lists is in the range [0, 50].
 * 	-100 <= Node.val <= 100
 * 	Both l1 and l2 are sorted in non-decreasing order.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/merge-two-sorted-lists/
// discuss: https://leetcode.com/problems/merge-two-sorted-lists/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = ListNode::new(0);
        let mut cur = &mut res.next;
        let mut ll1 = l1;
        let mut ll2 = l2;

        while ll1.is_some() || ll2.is_some() {
            if !ll1.is_some() {
                let val = ll2.as_mut().unwrap().val;
                *cur = Some(Box::new(ListNode::new(val)));
                ll2 = ll2.unwrap().next;
                cur = &mut cur.as_mut().unwrap().next;
                continue;
            }
            if !ll2.is_some() {
                let val = ll1.as_mut().unwrap().val;
                *cur = Some(Box::new(ListNode::new(val)));
                ll1 = ll1.unwrap().next;
                cur = &mut cur.as_mut().unwrap().next;
                continue;
            }

            let val1 = ll1.as_mut().unwrap().val;
            let val2 = ll2.as_mut().unwrap().val;
            if val1 < val2 {
                *cur = Some(Box::new(ListNode::new(val1)));
                ll1 = ll1.unwrap().next;
            } else {
                *cur = Some(Box::new(ListNode::new(val2)));
                ll2 = ll2.unwrap().next;
            }
            cur = &mut cur.as_mut().unwrap().next;
        }
        res.next
    }

    pub fn merge_two_lists_v2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::merge_two_lists_recursive(l1, l2)
    }

    fn merge_two_lists_recursive(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(l1), Some(l2)) => {
                if l1.val >= l2.val {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Self::merge_two_lists_recursive(Some(l1), l2.next),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Self::merge_two_lists_recursive(l1.next, Some(l2)),
                    }))
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
    fn test_0021_example_1() {
        let l1 = linked![1, 2, 4];
        let l2 = linked![1, 3, 4];

        let result = Solution::merge_two_lists(l1, l2);

        assert_eq!(result, linked![1, 1, 2, 3, 4, 4]);

        // Recursive solution check

        let l1 = linked![1, 2, 4];
        let l2 = linked![1, 3, 4];

        let result = Solution::merge_two_lists_v2(l1, l2);

        assert_eq!(result, linked![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_0021_example_2() {
        let l1 = linked![];
        let l2 = linked![];

        let result = Solution::merge_two_lists(l1, l2);

        assert_eq!(result, linked![]);

        // Recursive solution check

        let l1 = linked![];
        let l2 = linked![];

        let result = Solution::merge_two_lists_v2(l1, l2);

        assert_eq!(result, linked![]);
    }

    #[test]
    fn test_0021_example_3() {
        let l1 = linked![];
        let l2 = linked![0];

        let result = Solution::merge_two_lists(l1, l2);

        assert_eq!(result, linked![0]);

        // Recursive solution check

        let l1 = linked![];
        let l2 = linked![0];

        let result = Solution::merge_two_lists(l1, l2);

        assert_eq!(result, linked![0]);
    }
}

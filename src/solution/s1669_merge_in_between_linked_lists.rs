/**
 * [1669] Merge In Between Linked Lists
 *
 * You are given two linked lists: list1 and list2 of sizes n and m respectively.
 * Remove list1's nodes from the a^th node to the b^th node, and put list2 in their place.
 * The blue edges and nodes in the following figure indicate the result:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/fig1.png" style="height: 130px; width: 504px;" />
 * Build the result list and return its head.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2024/03/01/ll.png" style="width: 609px; height: 210px;" />
 * Input: list1 = [10,1,13,6,9,5], a = 3, b = 4, list2 = [1000000,1000001,1000002]
 * Output: [10,1,13,1000000,1000001,1000002,5]
 * Explanation: We remove the nodes 3 and 4 and put the entire list2 in their place. The blue edges and nodes in the above figure indicate the result.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/05/merge_linked_list_ex2.png" style="width: 463px; height: 140px;" />
 * Input: list1 = [0,1,2,3,4,5,6], a = 2, b = 5, list2 = [1000000,1000001,1000002,1000003,1000004]
 * Output: [0,1,1000000,1000001,1000002,1000003,1000004,6]
 * Explanation: The blue edges and nodes in the above figure indicate the result.
 *
 *  
 * Constraints:
 *
 * 	3 <= list1.length <= 10^4
 * 	1 <= a <= b < list1.length - 1
 * 	1 <= list2.length <= 10^4
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/merge-in-between-linked-lists/
// discuss: https://leetcode.com/problems/merge-in-between-linked-lists/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut head = list1.unwrap();

        let mut current = head.as_mut();
        for _ in 0..(a - 1) {
            current = current.next.as_mut().unwrap();
        }

        std::mem::swap(&mut current.next, &mut list2);

        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        for _ in 0..(b - a + 1) {
            list2 = list2.unwrap().next;
        }
        current.next = list2;

        Some(head)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1669_example_1() {
        let list1 = linked![10, 1, 13, 6, 9, 5];
        let a = 3;
        let b = 4;
        let list2 = linked![1000000, 1000001, 1000002];

        let result = linked![10, 1, 13, 1000000, 1000001, 1000002, 5];

        assert_eq!(Solution::merge_in_between(list1, a, b, list2), result);
    }

    #[test]
    fn test_1669_example_2() {
        let list1 = linked![0, 1, 2, 3, 4, 5, 6];
        let a = 2;
        let b = 5;
        let list2 = linked![1000000, 1000001, 1000002, 1000003, 1000004];

        let result = linked![0, 1, 1000000, 1000001, 1000002, 1000003, 1000004, 6];

        assert_eq!(Solution::merge_in_between(list1, a, b, list2), result);
    }
}

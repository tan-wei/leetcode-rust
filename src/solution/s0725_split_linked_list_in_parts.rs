/**
 * [0725] Split Linked List in Parts
 *
 * Given the head of a singly linked list and an integer k, split the linked list into k consecutive linked list parts.
 * The length of each part should be as equal as possible: no two parts should have a size differing by more than one. This may lead to some parts being null.
 * The parts should be in the order of occurrence in the input list, and parts occurring earlier should always have a size greater than or equal to parts occurring later.
 * Return an array of the k parts.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/split1-lc.jpg" style="width: 400px; height: 134px;" />
 * Input: head = [1,2,3], k = 5
 * Output: [[1],[2],[3],[],[]]
 * Explanation:
 * The first element output[0] has output[0].val = 1, output[0].next = null.
 * The last element output[4] is null, but its string representation as a ListNode is [].
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/13/split2-lc.jpg" style="width: 600px; height: 60px;" />
 * Input: head = [1,2,3,4,5,6,7,8,9,10], k = 3
 * Output: [[1,2,3,4],[5,6,7],[8,9,10]]
 * Explanation:
 * The input has been split into consecutive parts with size difference at most 1, and earlier parts are a larger size than the later parts.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 1000].
 * 	0 <= Node.val <= 1000
 * 	1 <= k <= 50
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/split-linked-list-in-parts/
// discuss: https://leetcode.com/problems/split-linked-list-in-parts/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut len = 0;
        let mut node = &head;
        while let Some(n) = node {
            len += 1;
            node = &n.next;
        }
        let mut head = head;
        let mut result = Vec::with_capacity(k as usize);
        for i in 0..k as usize {
            result.push(head);
            let mut node = &mut result[i];
            for _ in 0..len / k + if i < (len % k) as usize { 1 } else { 0 } {
                if let Some(n) = node {
                    node = &mut n.next;
                }
            }
            head = node.take();
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0725_example_1() {
        let head = linked![1, 2, 3];
        let k = 5;
        let result = vec![linked![1], linked![2], linked![3], linked![], linked![]];

        assert_eq!(Solution::split_list_to_parts(head, k), result);
    }

    #[test]
    fn test_0725_example_2() {
        let head = linked![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let k = 3;
        let result = vec![linked![1, 2, 3, 4], linked![5, 6, 7], linked![8, 9, 10]];

        assert_eq!(Solution::split_list_to_parts(head, k), result);
    }
}

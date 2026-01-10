/**
 * [0817] Linked List Components
 *
 * You are given the head of a linked list containing unique integer values and an integer array nums that is a subset of the linked list values.
 * Return the number of connected components in nums where two values are connected if they appear consecutively in the linked list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/22/lc-linkedlistcom1.jpg" style="width: 424px; height: 65px;" />
 * Input: head = [0,1,2,3], nums = [0,1,3]
 * Output: 2
 * Explanation: 0 and 1 are connected, so [0, 1] and [3] are the two connected components.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/22/lc-linkedlistcom2.jpg" style="width: 544px; height: 65px;" />
 * Input: head = [0,1,2,3,4], nums = [0,3,1,4]
 * Output: 2
 * Explanation: 0 and 1 are connected, 3 and 4 are connected, so [0, 1] and [3, 4] are the two connected components.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the linked list is n.
 * 	1 <= n <= 10^4
 * 	0 <= Node.val < n
 * 	All the values Node.val are unique.
 * 	1 <= nums.length <= n
 * 	0 <= nums[i] < n
 * 	All the values of nums are unique.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/linked-list-components/
// discuss: https://leetcode.com/problems/linked-list-components/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/linked-list-components/discuss/1496457/rust-4ms-iterator-solution
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<std::collections::HashSet<_>>();
        (0..)
            .scan(&head, |node, _| {
                node.as_deref().map(|ListNode { val, next }| {
                    *node = next;
                    nums.contains(&val)
                })
            })
            .fuse()
            .fold((0, false), |(cnt, pre), b| (i32::from(b && !pre) + cnt, b))
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use crate::util::linked_list;

    use super::*;

    #[test]
    fn test_0817_example_1() {
        let head = linked![0, 1, 2, 3];
        let nums = vec![0, 1, 3];
        let result = 2;

        assert_eq!(Solution::num_components(head, nums), result);
    }

    #[test]
    fn test_0817_example_2() {
        let head = linked![0, 1, 2, 3, 4];
        let nums = vec![0, 3, 1, 4];
        let result = 2;

        assert_eq!(Solution::num_components(head, nums), result);
    }
}

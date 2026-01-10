/**
 * [2074] Reverse Nodes in Even Length Groups
 *
 * You are given the head of a linked list.
 * The nodes in the linked list are sequentially assigned to non-empty groups whose lengths form the sequence of the natural numbers (1, 2, 3, 4, ...). The length of a group is the number of nodes assigned to it. In other words,
 *
 * 	The 1^st node is assigned to the first group.
 * 	The 2^nd and the 3^rd nodes are assigned to the second group.
 * 	The 4^th, 5^th, and 6^th nodes are assigned to the third group, and so on.
 *
 * Note that the length of the last group may be less than or equal to 1 + the length of the second to last group.
 * Reverse the nodes in each group with an even length, and return the head of the modified linked list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/25/eg1.png" style="width: 699px; height: 124px;" />
 * Input: head = [5,2,6,3,9,1,7,3,8,4]
 * Output: [5,6,2,3,9,1,4,8,3,7]
 * Explanation:
 * - The length of the first group is 1, which is odd, hence no reversal occurs.
 * - The length of the second group is 2, which is even, hence the nodes are reversed.
 * - The length of the third group is 3, which is odd, hence no reversal occurs.
 * - The length of the last group is 4, which is even, hence the nodes are reversed.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/25/eg2.png" style="width: 284px; height: 114px;" />
 * Input: head = [1,1,0,6]
 * Output: [1,0,1,6]
 * Explanation:
 * - The length of the first group is 1. No reversal occurs.
 * - The length of the second group is 2. The nodes are reversed.
 * - The length of the last group is 1. No reversal occurs.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/17/ex3.png" style="width: 348px; height: 114px;" />
 * Input: head = [1,1,0,6,5]
 * Output: [1,0,1,5,6]
 * Explanation:
 * - The length of the first group is 1. No reversal occurs.
 * - The length of the second group is 2. The nodes are reversed.
 * - The length of the last group is 2. The nodes are reversed.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [1, 10^5].
 * 	0 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/reverse-nodes-in-even-length-groups/
// discuss: https://leetcode.com/problems/reverse-nodes-in-even-length-groups/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn reverse_even_length_groups(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode::new(0)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2074_example_1() {
        let head = linked![5, 2, 6, 3, 9, 1, 7, 3, 8, 4];

        let result = linked![5, 6, 2, 3, 9, 1, 4, 8, 3, 7];

        assert_eq!(Solution::reverse_even_length_groups(head), result);
    }

    #[test]
    #[ignore]
    fn test_2074_example_2() {
        let head = linked![1, 1, 0, 6];

        let result = linked![1, 0, 1, 6];

        assert_eq!(Solution::reverse_even_length_groups(head), result);
    }

    #[test]
    #[ignore]
    fn test_2074_example_3() {
        let head = linked![1, 1, 0, 6, 5];

        let result = linked![1, 0, 1, 5, 6];

        assert_eq!(Solution::reverse_even_length_groups(head), result);
    }
}

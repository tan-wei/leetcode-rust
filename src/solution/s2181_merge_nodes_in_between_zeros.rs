/**
 * [2181] Merge Nodes in Between Zeros
 *
 * You are given the head of a linked list, which contains a series of integers separated by 0's. The beginning and end of the linked list will have Node.val == 0.
 * For every two consecutive 0's, merge all the nodes lying in between them into a single node whose value is the sum of all the merged nodes. The modified list should not contain any 0's.
 * Return the head of the modified linked list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/02/ex1-1.png" style="width: 600px; height: 41px;" />
 * Input: head = [0,3,1,0,4,5,2,0]
 * Output: [4,11]
 * Explanation:
 * The above figure represents the given linked list. The modified list contains
 * - The sum of the nodes marked in green: 3 + 1 = 4.
 * - The sum of the nodes marked in red: 4 + 5 + 2 = 11.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/02/ex2-1.png" style="width: 600px; height: 41px;" />
 * Input: head = [0,1,0,3,0,2,2,0]
 * Output: [1,3,4]
 * Explanation:
 * The above figure represents the given linked list. The modified list contains
 * - The sum of the nodes marked in green: 1 = 1.
 * - The sum of the nodes marked in red: 3 = 3.
 * - The sum of the nodes marked in yellow: 2 + 2 = 4.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [3, 2 * 10^5].
 * 	0 <= Node.val <= 1000
 * 	There are no two consecutive nodes with Node.val == 0.
 * 	The beginning and end of the linked list have Node.val == 0.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/merge-nodes-in-between-zeros/
// discuss: https://leetcode.com/problems/merge-nodes-in-between-zeros/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode::new(0)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2181_example_1() {
        let head = linked![0, 3, 1, 0, 4, 5, 2, 0];

        let result = linked![4, 11];

        assert_eq!(Solution::merge_nodes(head), result);
    }

    #[test]
    #[ignore]
    fn test_2181_example_2() {
        let head = linked![0, 1, 0, 3, 0, 2, 2, 0];

        let result = linked![1, 3, 4];

        assert_eq!(Solution::merge_nodes(head), result);
    }
}

/**
 * [2095] Delete the Middle Node of a Linked List
 *
 * You are given the head of a linked list. Delete the middle node, and return the head of the modified linked list.
 * The middle node of a linked list of size n is the &lfloor;n / 2&rfloor;^th node from the start using 0-based indexing, where &lfloor;x&rfloor; denotes the largest integer less than or equal to x.
 *
 * 	For n = 1, 2, 3, 4, and 5, the middle nodes are 0, 1, 1, 2, and 2, respectively.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/16/eg1drawio.png" style="width: 500px; height: 77px;" />
 * Input: head = [1,3,4,7,1,2,6]
 * Output: [1,3,4,1,2,6]
 * Explanation:
 * The above figure represents the given linked list. The indices of the nodes are written below.
 * Since n = 7, node 3 with value 7 is the middle node, which is marked in red.
 * We return the new list after removing this node.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/16/eg2drawio.png" style="width: 250px; height: 43px;" />
 * Input: head = [1,2,3,4]
 * Output: [1,2,4]
 * Explanation:
 * The above figure represents the given linked list.
 * For n = 4, node 2 with value 3 is the middle node, which is marked in red.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/11/16/eg3drawio.png" style="width: 150px; height: 58px;" />
 * Input: head = [2,1]
 * Output: [2]
 * Explanation:
 * The above figure represents the given linked list.
 * For n = 2, node 1 with value 1 is the middle node, which is marked in red.
 * Node 0 with value 2 is the only node remaining after removing node 1.
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [1, 10^5].
 * 	1 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/
// discuss: https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode::new(0)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2095_example_1() {
        let head = linked![1, 3, 4, 1, 2, 6];

        let result = linked![1, 3, 4, 1, 2, 6];

        assert_eq!(Solution::delete_middle(head), result);
    }

    #[test]
    #[ignore]
    fn test_2095_example_2() {
        let head = linked![1, 2, 3, 4];

        let result = linked![1, 2, 4];

        assert_eq!(Solution::delete_middle(head), result);
    }

    #[test]
    #[ignore]
    fn test_2095_example_3() {
        let head = linked![2, 1];

        let result = linked![2];

        assert_eq!(Solution::delete_middle(head), result);
    }
}

/**
 * [2130] Maximum Twin Sum of a Linked List
 *
 * In a linked list of size n, where n is even, the i^th node (0-indexed) of the linked list is known as the twin of the (n-1-i)^th node, if 0 <= i <= (n / 2) - 1.
 *
 * 	For example, if n = 4, then node 0 is the twin of node 3, and node 1 is the twin of node 2. These are the only nodes with twins for n = 4.
 *
 * The twin sum is defined as the sum of a node and its twin.
 * Given the head of a linked list with even length, return the maximum twin sum of the linked list.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/03/eg1drawio.png" style="width: 250px; height: 70px;" />
 * Input: head = [5,4,2,1]
 * Output: 6
 * Explanation:
 * Nodes 0 and 1 are the twins of nodes 3 and 2, respectively. All have twin sum = 6.
 * There are no other nodes with twins in the linked list.
 * Thus, the maximum twin sum of the linked list is 6.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/03/eg2drawio.png" style="width: 250px; height: 70px;" />
 * Input: head = [4,2,2,3]
 * Output: 7
 * Explanation:
 * The nodes with twins present in this linked list are:
 * - Node 0 is the twin of node 3 having a twin sum of 4 + 3 = 7.
 * - Node 1 is the twin of node 2 having a twin sum of 2 + 2 = 4.
 * Thus, the maximum twin sum of the linked list is max(7, 4) = 7.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/03/eg3drawio.png" style="width: 200px; height: 88px;" />
 * Input: head = [1,100000]
 * Output: 100001
 * Explanation:
 * There is only one node with a twin in the linked list having twin sum of 1 + 100000 = 100001.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is an even integer in the range [2, 10^5].
 * 	1 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/
// discuss: https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2130_example_1() {
        let head = linked![5, 4, 2, 1];

        let result = 6;

        assert_eq!(Solution::pair_sum(head), result);
    }

    #[test]
    #[ignore]
    fn test_2130_example_2() {
        let head = linked![4, 2, 2, 3];

        let result = 7;

        assert_eq!(Solution::pair_sum(head), result);
    }

    #[test]
    #[ignore]
    fn test_2130_example_3() {
        let head = linked![1, 100000];

        let result = 100001;

        assert_eq!(Solution::pair_sum(head), result);
    }
}

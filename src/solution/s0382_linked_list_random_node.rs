/**
 * [382] Linked List Random Node
 *
 * Given a singly linked list, return a random node's value from the linked list. Each node must have the same probability of being chosen.
 * Implement the Solution class:
 *
 * 	Solution(ListNode head) Initializes the object with the integer array nums.
 * 	int getRandom() Chooses a node randomly from the list and returns its value. All the nodes of the list should be equally likely to be choosen.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/16/getrand-linked-list.jpg" style="width: 302px; height: 62px;" />
 * Input
 * ["Solution", "getRandom", "getRandom", "getRandom", "getRandom", "getRandom"]
 * [[[1, 2, 3]], [], [], [], [], []]
 * Output
 * [null, 1, 3, 2, 2, 3]
 * Explanation
 * Solution solution = new Solution([1, 2, 3]);
 * solution.getRandom(); // return 1
 * solution.getRandom(); // return 3
 * solution.getRandom(); // return 2
 * solution.getRandom(); // return 2
 * solution.getRandom(); // return 3
 * // getRandom() should return either 1, 2, or 3 randomly. Each element should have equal probability of returning.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the linked list will be in the range [1, 10^4].
 * 	-10^4 <= Node.val <= 10^4
 * 	At most 10^4 calls will be made to getRandom.
 *
 *  
 * Follow up:
 *
 * 	What if the linked list is extremely large and its length is unknown to you?
 * 	Could you solve this efficiently without using extra space?
 *
 */
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/linked-list-random-node/
// discuss: https://leetcode.com/problems/linked-list-random-node/discuss/?currentPage=1&orderBy=most_votes&query=

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

// Credit: https://leetcode.com/problems/linked-list-random-node/discuss/1494410/rust-O(1)-space
use rand::{Rng, rngs::ThreadRng};
pub struct Solution {
    head: Option<Box<ListNode>>,
    len: usize,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self {
            len: (0..)
                .scan(&head, |node, _| {
                    node.as_deref().map(|next| *node = &next.next)
                })
                .fuse()
                .count(),
            rng: ThreadRng::default(),
            head,
        }
    }

    fn get_random(&mut self) -> i32 {
        (0..self.rng.gen_range(0, self.len))
            .fold(self.head.as_deref().unwrap(), |ptr, _| {
                ptr.next.as_deref().unwrap()
            })
            .val
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0382_example_1() {
        let mut solution = Solution::new(linked![1, 2, 3]);
        solution.get_random(); // return 1
        solution.get_random(); // return 3
        solution.get_random(); // return 2
        solution.get_random(); // return 2
        solution.get_random(); // return 3
    }
}

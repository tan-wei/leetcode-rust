/**
 * [61] Rotate List
 *
 * Given the head of a linked list, rotate the list to the right by k places.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/rotate1.jpg" style="width: 600px; height: 254px;" />
 * Input: head = [1,2,3,4,5], k = 2
 * Output: [4,5,1,2,3]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/roate2.jpg" style="width: 472px; height: 542px;" />
 * Input: head = [0,1,2], k = 4
 * Output: [2,0,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 500].
 * 	-100 <= Node.val <= 100
 * 	0 <= k <= 2 * 10^9
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/rotate-list/
// discuss: https://leetcode.com/problems/rotate-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = match head {
            Some(a) => a,
            None => return None,
        };

        let len = {
            let mut len = 1;
            let mut cur = &head;
            while let Some(ref next) = cur.next {
                len += 1;
                cur = next;
            }
            len
        };

        let mut target = len - k % len;

        if target == len {
            return Some(head);
        }

        let mut cur = &mut head;

        while target > 1 {
            cur = cur.next.as_mut().unwrap();
            target -= 1
        }

        // Do swap
        let mut result = std::mem::replace(&mut cur.next, None).unwrap();
        let mut cur = &mut result;

        while let Some(ref mut next) = cur.next {
            cur = next;
        }
        cur.next = Some(head);

        Some(result)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0061_example_1() {
        let head = linked![1, 2, 3, 4, 5];
        let k = 2;
        let result = linked![4, 5, 1, 2, 3];

        assert_eq!(Solution::rotate_right(head, k), result);
    }

    #[test]
    fn test_0061_example_2() {
        let head = linked![0, 1, 2];
        let k = 4;
        let result = linked![2, 0, 1];

        assert_eq!(Solution::rotate_right(head, k), result);
    }
}

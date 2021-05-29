/**
 * [148] Sort List
 *
 * Given the head of a linked list, return the list after sorting it in ascending order.
 * Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.e. constant space)?
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_1.jpg" style="width: 450px; height: 194px;" />
 * Input: head = [4,2,1,3]
 * Output: [1,2,3,4]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/14/sort_list_2.jpg" style="width: 550px; height: 184px;" />
 * Input: head = [-1,5,3,4,0]
 * Output: [-1,0,3,4,5]
 *
 * Example 3:
 *
 * Input: head = []
 * Output: []
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [0, 5 * 10^4].
 * 	-10^5 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/sort-list/
// discuss: https://leetcode.com/problems/sort-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // Credit: https://leetcode.com/problems/sort-list/discuss/477190/Rust-Solution%3A-100-faster-(4ms)-100-less-MB-(4.1-MB)-Elegant-Solution
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn helper(
            head: Option<Box<ListNode>>,
            appending_node: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match head {
                None => appending_node,
                Some(node) => {
                    let value = node.val;
                    let mut less_than_node = None;
                    let mut more_than_node = None;
                    let mut equal_node = None;
                    let mut iter_node = Some(node);
                    while let Some(mut node) = iter_node {
                        let next_node = node.next;
                        if node.val == value {
                            node.next = equal_node;
                            equal_node = Some(node);
                        } else if node.val < value {
                            node.next = less_than_node;
                            less_than_node = Some(node);
                        } else {
                            node.next = more_than_node;
                            more_than_node = Some(node);
                        }
                        iter_node = next_node
                    }

                    let sorted_more_than_node = helper(more_than_node, appending_node);
                    let mut last_iter_mut = equal_node.as_mut();
                    while let Some(last_iter) = last_iter_mut {
                        if let None = last_iter.next {
                            last_iter.next = sorted_more_than_node;
                            break;
                        }
                        last_iter_mut = last_iter.next.as_mut();
                    }

                    helper(less_than_node, equal_node)
                }
            }
        }

        helper(head, None)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0148_example_1() {
        let head = linked![4, 2, 1, 3];
        let result = linked![1, 2, 3, 4];

        assert_eq!(Solution::sort_list(head), result);
    }

    #[test]
    fn test_0148_example_2() {
        let head = linked![-1, 5, 3, 4, 0];
        let result = linked![-1, 0, 3, 4, 5];

        assert_eq!(Solution::sort_list(head), result);
    }

    #[test]
    fn test_0148_example_3() {
        let head = linked![];
        let result = linked![];

        assert_eq!(Solution::sort_list(head), result);
    }
}

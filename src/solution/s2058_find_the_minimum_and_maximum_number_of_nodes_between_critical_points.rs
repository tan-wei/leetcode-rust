/**
 * [2058] Find the Minimum and Maximum Number of Nodes Between Critical Points
 *
 * A critical point in a linked list is defined as either a local maxima or a local minima.
 * A node is a local maxima if the current node has a value strictly greater than the previous node and the next node.
 * A node is a local minima if the current node has a value strictly smaller than the previous node and the next node.
 * Note that a node can only be a local maxima/minima if there exists both a previous node and a next node.
 * Given a linked list head, return an array of length 2 containing [minDistance, maxDistance] where minDistance is the minimum distance between any two distinct critical points and maxDistance is the maximum distance between any two distinct critical points. If there are fewer than two critical points, return [-1, -1].
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/13/a1.png" style="width: 148px; height: 55px;" />
 * Input: head = [3,1]
 * Output: [-1,-1]
 * Explanation: There are no critical points in [3,1].
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/13/a2.png" style="width: 624px; height: 46px;" />
 * Input: head = [5,3,1,2,5,1,2]
 * Output: [1,3]
 * Explanation: There are three critical points:
 * - [5,3,<u>1</u>,2,5,1,2]: The third node is a local minima because 1 is less than 3 and 2.
 * - [5,3,1,2,<u>5</u>,1,2]: The fifth node is a local maxima because 5 is greater than 2 and 1.
 * - [5,3,1,2,5,<u>1</u>,2]: The sixth node is a local minima because 1 is less than 5 and 2.
 * The minimum distance is between the fifth and the sixth node. minDistance = 6 - 5 = 1.
 * The maximum distance is between the third and the sixth node. maxDistance = 6 - 3 = 3.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/10/14/a5.png" style="width: 624px; height: 39px;" />
 * Input: head = [1,3,2,2,3,2,2,2,7]
 * Output: [3,3]
 * Explanation: There are two critical points:
 * - [1,<u>3</u>,2,2,3,2,2,2,7]: The second node is a local maxima because 3 is greater than 1 and 2.
 * - [1,3,2,2,<u>3</u>,2,2,2,7]: The fifth node is a local maxima because 3 is greater than 2 and 2.
 * Both the minimum and maximum distances are between the second and the fifth node.
 * Thus, minDistance and maxDistance is 5 - 2 = 3.
 * Note that the last node is not considered a local maxima because it does not have a next node.
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the list is in the range [2, 10^5].
 * 	1 <= Node.val <= 10^5
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/
// discuss: https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2058_example_1() {
        let head = linked![3, 1];

        let result = vec![-1, -1];

        assert_eq!(Solution::nodes_between_critical_points(head), result);
    }

    #[test]
    #[ignore]
    fn test_2058_example_2() {
        let head = linked![5, 3, 1, 2, 5, 1, 2];

        let result = vec![1, 3];

        assert_eq!(Solution::nodes_between_critical_points(head), result);
    }

    #[test]
    #[ignore]
    fn test_2058_example_3() {
        let head = linked![1, 3, 2, 2, 3, 2, 2, 2, 7];

        let result = vec![3, 3];

        assert_eq!(Solution::nodes_between_critical_points(head), result);
    }
}

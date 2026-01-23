/**
 * [2326] Spiral Matrix IV
 *
 * You are given two integers m and n, which represent the dimensions of a matrix.
 * You are also given the head of a linked list of integers.
 * Generate an m x n matrix that contains the integers in the linked list presented in spiral order (clockwise), starting from the top-left of the matrix. If there are remaining empty spaces, fill them with -1.
 * Return the generated matrix.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/09/ex1new.jpg" style="width: 240px; height: 150px;" />
 * Input: m = 3, n = 5, head = [3,0,2,6,8,1,7,9,4,2,5,5,0]
 * Output: [[3,0,2,6,8],[5,0,-1,-1,1],[5,2,4,9,7]]
 * Explanation: The diagram above shows how the values are printed in the matrix.
 * Note that the remaining spaces in the matrix are filled with -1.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/05/11/ex2.jpg" style="width: 221px; height: 60px;" />
 * Input: m = 1, n = 4, head = [0,1,2]
 * Output: [[0,1,2,-1]]
 * Explanation: The diagram above shows how the values are printed from left to right in the matrix.
 * The last space in the matrix is set to -1.
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 10^5
 * 	1 <= m * n <= 10^5
 * 	The number of nodes in the list is in the range [1, m * n].
 * 	0 <= Node.val <= 1000
 *
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/spiral-matrix-iv/
// discuss: https://leetcode.com/problems/spiral-matrix-iv/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2326_example_1() {
        let m = 3;
        let n = 5;
        let head = linked![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0];

        let result = vec![
            vec![3, 0, 2, 6, 8],
            vec![5, 0, -1, -1, 1],
            vec![5, 2, 4, 9, 7],
        ];

        assert_eq!(Solution::spiral_matrix(m, n, head), result);
    }

    #[test]
    #[ignore]
    fn test_2326_example_2() {
        let m = 1;
        let n = 4;
        let head = linked![0, 1, 2];

        let result = vec![vec![0, 1, 2, -1]];

        assert_eq!(Solution::spiral_matrix(m, n, head), result);
    }
}

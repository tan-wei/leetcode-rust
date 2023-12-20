use std::collections::VecDeque;

/**
 * [1424] Diagonal Traverse II
 *
 * Given a 2D integer array nums, return all elements of nums in diagonal order as shown in the below images.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_1_1784.png" style="width: 158px; height: 143px;" />
 * Input: nums = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [1,4,2,7,5,3,8,6,9]
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/08/sample_2_1784.png" style="width: 230px; height: 177px;" />
 * Input: nums = [[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]]
 * Output: [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i].length <= 10^5
 * 	1 <= sum(nums[i].length) <= 10^5
 * 	1 <= nums[i][j] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/diagonal-traverse-ii/
// discuss: https://leetcode.com/problems/diagonal-traverse-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let mut q = std::collections::VecDeque::new();
        let mut result = vec![];

        q.push_back((0, 0));

        while let Some((i, j)) = q.pop_front() {
            result.push(nums[i][j]);

            if j == 0 && i + 1 < n {
                q.push_back((i + 1, j));
            }

            if j + 1 < nums[i].len() {
                q.push_back((i, j + 1));
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1424_example_1() {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let result = vec![1, 4, 2, 7, 5, 3, 8, 6, 9];

        assert_eq!(Solution::find_diagonal_order(nums), result);
    }

    #[test]
    fn test_1424_example_2() {
        let nums = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16],
        ];

        let result = vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16];

        assert_eq!(Solution::find_diagonal_order(nums), result);
    }
}

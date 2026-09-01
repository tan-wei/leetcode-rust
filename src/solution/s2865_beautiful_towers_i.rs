/**
 * [2865] Beautiful Towers I
 *
 * You are given an array heights of n integers representing the number of bricks in n consecutive towers. Your task is to remove some bricks to form a mountain-shaped tower arrangement. In this arrangement, the tower heights are non-decreasing, reaching a maximum peak value with one or multiple consecutive towers and then non-increasing.
 * Return the maximum possible sum of heights of a mountain-shaped tower arrangement.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">heights = [5,3,4,1,1]</span>
 * Output: <span class="example-io">13</span>
 * Explanation:
 * We remove some bricks to make heights = [5,3,3,1,1], the peak is at index 0.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">heights = [6,5,3,9,2,7]</span>
 * Output: <span class="example-io">22</span>
 * Explanation:
 * We remove some bricks to make heights = [3,3,3,9,2,2], the peak is at index 3.
 * </div>
 * Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">heights = [3,2,5,5,2,3]</span>
 * Output: <span class="example-io">18</span>
 * Explanation:
 * We remove some bricks to make heights = [2,2,5,5,2,2], the peak is at index 2 or 3.
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= n == heights.length <= 10^3
 * 	1 <= heights[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/beautiful-towers-i/
// discuss: https://leetcode.com/problems/beautiful-towers-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_sum_of_heights(heights: Vec<i32>) -> i64 {
        0i64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2865_example_1() {
        let heights = vec![5, 3, 4, 1, 1];

        let result = 13;

        assert_eq!(Solution::maximum_sum_of_heights(heights), result);
    }

    #[test]
    #[ignore]
    fn test_2865_example_2() {
        let heights = vec![6, 5, 3, 9, 2, 7];

        let result = 22;

        assert_eq!(Solution::maximum_sum_of_heights(heights), result);
    }

    #[test]
    #[ignore]
    fn test_2865_example_3() {
        let heights = vec![3, 2, 5, 5, 2, 3];

        let result = 18;

        assert_eq!(Solution::maximum_sum_of_heights(heights), result);
    }
}

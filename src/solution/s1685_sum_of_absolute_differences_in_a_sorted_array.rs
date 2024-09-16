/**
 * [1685] Sum of Absolute Differences in a Sorted Array
 *
 * You are given an integer array nums sorted in non-decreasing order.
 * Build and return an integer array result with the same length as nums such that result[i] is equal to the summation of absolute differences between nums[i] and all the other elements in the array.
 * In other words, result[i] is equal to sum(|nums[i]-nums[j]|) where 0 <= j < nums.length and j != i (0-indexed).
 *  
 * Example 1:
 *
 * Input: nums = [2,3,5]
 * Output: [4,3,5]
 * Explanation: Assuming the arrays are 0-indexed, then
 * result[0] = |2-2| + |2-3| + |2-5| = 0 + 1 + 3 = 4,
 * result[1] = |3-2| + |3-3| + |3-5| = 1 + 0 + 2 = 3,
 * result[2] = |5-2| + |5-3| + |5-5| = 3 + 2 + 0 = 5.
 *
 * Example 2:
 *
 * Input: nums = [1,4,6,8,10]
 * Output: [24,15,13,15,21]
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^5
 * 	1 <= nums[i] <= nums[i + 1] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/
// discuss: https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        nums.iter()
            .enumerate()
            .scan((0, nums.iter().sum::<i32>()), |mut x, (i, v)| {
                let result = x.1 - x.0 + -(nums.len() as i32 - 2 * i as i32) * v;
                x.1 -= v;
                x.0 += v;
                Some(result)
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1685_example_1() {
        let nums = vec![2, 3, 5];

        let result = vec![4, 3, 5];

        assert_eq!(Solution::get_sum_absolute_differences(nums), result);
    }

    #[test]
    fn test_1685_example_2() {
        let nums = vec![1, 4, 6, 8, 10];

        let result = vec![24, 15, 13, 15, 21];

        assert_eq!(Solution::get_sum_absolute_differences(nums), result);
    }
}

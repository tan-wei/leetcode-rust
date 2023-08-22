/**
 * [1283] Find the Smallest Divisor Given a Threshold
 *
 * Given an array of integers nums and an integer threshold, we will choose a positive integer divisor, divide all the array by it, and sum the division's result. Find the smallest divisor such that the result mentioned above is less than or equal to threshold.
 * Each result of the division is rounded to the nearest integer greater than or equal to that element. (For example: 7/3 = 3 and 10/2 = 5).
 * The test cases are generated so that there will be an answer.
 *
 * Example 1:
 *
 * Input: nums = [1,2,5,9], threshold = 6
 * Output: 5
 * Explanation: We can get a sum to 17 (1+2+5+9) if the divisor is 1.
 * If the divisor is 4 we can get a sum of 7 (1+1+2+3) and if the divisor is 5 the sum will be 5 (1+1+1+2).
 *
 * Example 2:
 *
 * Input: nums = [44,22,33,11,1], threshold = 5
 * Output: 44
 *
 *
 * Constraints:
 *
 * 	1 <= nums.length <= 5 * 10^4
 * 	1 <= nums[i] <= 10^6
 * 	nums.length <= threshold <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/
// discuss: https://leetcode.com/problems/find-the-smallest-divisor-given-a-threshold/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let nums = nums.into_iter().map(|v| v as usize).collect::<Vec<usize>>();
        let threshold = threshold as usize;
        let mut left = 0;
        let mut right = 10usize.pow(6) + 10;

        while left + 1 < right {
            let mid = (left + right) / 2;
            let mut tot = 0;
            for &v in &nums {
                tot += v / mid;
                if v % mid != 0 {
                    tot += 1;
                }
            }
            if threshold < tot {
                left = mid;
            } else {
                right = mid;
            }
        }
        right as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1283_example_1() {
        let nums = vec![1, 2, 5, 9];
        let threshold = 6;
        let result = 5;

        assert_eq!(Solution::smallest_divisor(nums, threshold), result);
    }

    #[test]
    fn test_1283_example_2() {
        let nums = vec![44, 22, 33, 11, 1];
        let threshold = 5;
        let result = 44;

        assert_eq!(Solution::smallest_divisor(nums, threshold), result);
    }
}

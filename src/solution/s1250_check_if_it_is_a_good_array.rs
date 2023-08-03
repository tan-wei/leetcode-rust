/**
 * [1250] Check If It Is a Good Array
 *
 * Given an array nums of positive integers. Your task is to select some subset of nums, multiply each element by an integer and add all these numbers. The array is said to be good if you can obtain a sum of 1 from the array by any possible subset and multiplicand.
 * Return True if the array is good otherwise return False.
 *
 * Example 1:
 *
 * Input: nums = [12,5,7,23]
 * Output: true
 * Explanation: Pick numbers 5 and 7.
 * 5*3 + 7*(-2) = 1
 *
 * Example 2:
 *
 * Input: nums = [29,6,10]
 * Output: true
 * Explanation: Pick numbers 29, 6 and 10.
 * 29*1 + 6*(-3) + 10*(-1) = 1
 *
 * Example 3:
 *
 * Input: nums = [3,6]
 * Output: false
 *
 *
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-it-is-a-good-array/
// discuss: https://leetcode.com/problems/check-if-it-is-a-good-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_good_array(nums: Vec<i32>) -> bool {
        fn gcd(mut m: i32, mut n: i32) -> i32 {
            while m != 0 {
                let temp = m;
                m = n % temp;
                n = temp;
            }
            n.abs()
        }

        let n = nums.len();
        let mut a = 0;
        for i in 0..n {
            a = gcd(a, nums[i]);
            if a == 1 {
                return true;
            }
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1250_example_1() {
        let nums = vec![12, 5, 7, 23];
        let result = true;

        assert_eq!(Solution::is_good_array(nums), result);
    }

    #[test]
    fn test_1250_example_2() {
        let nums = vec![29, 6, 10];
        let result = true;

        assert_eq!(Solution::is_good_array(nums), result);
    }

    #[test]
    fn test_1250_example_3() {
        let nums = vec![3, 6];
        let result = false;

        assert_eq!(Solution::is_good_array(nums), result);
    }
}

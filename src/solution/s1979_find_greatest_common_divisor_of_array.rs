/**
 * [1979] Find Greatest Common Divisor of Array
 *
 * Given an integer array nums, return the greatest common divisor of the smallest number and largest number in nums.
 * The greatest common divisor of two numbers is the largest positive integer that evenly divides both numbers.
 *  
 * Example 1:
 *
 * Input: nums = [2,5,6,9,10]
 * Output: 2
 * Explanation:
 * The smallest number in nums is 2.
 * The largest number in nums is 10.
 * The greatest common divisor of 2 and 10 is 2.
 *
 * Example 2:
 *
 * Input: nums = [7,5,6,8,3]
 * Output: 1
 * Explanation:
 * The smallest number in nums is 3.
 * The largest number in nums is 8.
 * The greatest common divisor of 3 and 8 is 1.
 *
 * Example 3:
 *
 * Input: nums = [3,3]
 * Output: 3
 * Explanation:
 * The smallest number in nums is 3.
 * The largest number in nums is 3.
 * The greatest common divisor of 3 and 3 is 3.
 *
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 1000
 * 	1 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-greatest-common-divisor-of-array/
// discuss: https://leetcode.com/problems/find-greatest-common-divisor-of-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (mn, ma) = nums.iter().fold((1000, 1), |(mn, ma), &y| {
            if (y > ma) {
                if (y < mn) {
                    (y, y)
                } else {
                    (mn, y)
                }
            } else if (y < mn) {
                (y, ma)
            } else {
                (mn, ma)
            }
        });

        (1..=mn).fold(1, |x, y| if (mn % y == 0) && (ma % y) == 0 { y } else { x })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1979_example_1() {
        let nums = vec![2, 5, 6, 9, 10];

        let result = 2;

        assert_eq!(Solution::find_gcd(nums), result);
    }

    #[test]
    fn test_1979_example_2() {
        let nums = vec![7, 5, 6, 8, 3];

        let result = 1;

        assert_eq!(Solution::find_gcd(nums), result);
    }

    #[test]
    fn test_1979_example_3() {
        let nums = vec![3, 3];

        let result = 3;

        assert_eq!(Solution::find_gcd(nums), result);
    }
}

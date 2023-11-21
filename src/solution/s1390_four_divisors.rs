/**
 * [1390] Four Divisors
 *
 * Given an integer array nums, return the sum of divisors of the integers in that array that have exactly four divisors. If there is no such integer in the array, return 0.
 *  
 * Example 1:
 *
 * Input: nums = [21,4,7]
 * Output: 32
 * Explanation:
 * 21 has 4 divisors: 1, 3, 7, 21
 * 4 has 3 divisors: 1, 2, 4
 * 7 has 2 divisors: 1, 7
 * The answer is the sum of divisors of 21 only.
 *
 * Example 2:
 *
 * Input: nums = [21,21]
 * Output: 64
 *
 * Example 3:
 *
 * Input: nums = [1,2,3,4,5]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	1 <= nums[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/four-divisors/
// discuss: https://leetcode.com/problems/four-divisors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for v in nums {
            let mut i = 2;
            let mut set = std::collections::HashSet::new();
            set.insert(1);
            set.insert(v);
            while i * i <= v {
                if v % i == 0 {
                    set.insert(v / i);
                    set.insert(i);
                }
                i += 1;
            }

            if set.len() == 4 {
                result += set.iter().sum::<i32>();
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
    fn test_1390_example_1() {
        let nums = vec![21, 4, 7];

        let result = 32;

        assert_eq!(Solution::sum_four_divisors(nums), result);
    }

    #[test]
    fn test_1390_example_2() {
        let nums = vec![21, 21];

        let result = 64;

        assert_eq!(Solution::sum_four_divisors(nums), result);
    }

    #[test]
    fn test_1390_example_3() {
        let nums = vec![1, 2, 3, 4, 5];

        let result = 0;

        assert_eq!(Solution::sum_four_divisors(nums), result);
    }
}

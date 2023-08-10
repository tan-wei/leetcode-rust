/**
 * [1262] Greatest Sum Divisible by Three
 *
 * Given an integer array nums, return the maximum possible sum of elements of the array such that it is divisible by three.
 *  
 * Example 1:
 *
 * Input: nums = [3,6,5,1,8]
 * Output: 18
 * Explanation: Pick numbers 3, 6, 1 and 8 their sum is 18 (maximum sum divisible by 3).
 * Example 2:
 *
 * Input: nums = [4]
 * Output: 0
 * Explanation: Since 4 is not divisible by 3, do not pick any number.
 *
 * Example 3:
 *
 * Input: nums = [1,2,3,4,4]
 * Output: 12
 * Explanation: Pick numbers 1, 3, 4 and 4 their sum is 12 (maximum sum divisible by 3).
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 4 * 10^4
 * 	1 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/greatest-sum-divisible-by-three/
// discuss: https://leetcode.com/problems/greatest-sum-divisible-by-three/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ones = vec![];
        let mut twos = vec![];

        let mut tot = 0;
        for v in nums {
            let rv = v % 3;
            if rv == 1 {
                if ones.len() < 2 {
                    ones.push(v);
                }
            } else if rv == 2 {
                if twos.len() < 2 {
                    twos.push(v);
                }
            }
            tot += v;
        }

        let rv = tot % 3;
        if rv == 0 {
            tot
        } else if rv == 1 {
            let mut result = tot - ones[0];
            if twos.len() > 1 {
                result = std::cmp::max(result, tot - twos[0] - twos[1]);
            }
            result
        } else {
            let mut result = 0;
            if twos.len() > 0 {
                result = tot - twos[0];
            }
            if ones.len() > 1 {
                result = std::cmp::max(result, tot - ones[0] - ones[1]);
            }
            result
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1262_example_1() {
        let nums = vec![3, 6, 5, 1, 8];
        let result = 18;

        assert_eq!(Solution::max_sum_div_three(nums), result);
    }

    #[test]
    fn test_1262_example_2() {
        let nums = vec![4];
        let result = 0;

        assert_eq!(Solution::max_sum_div_three(nums), result);
    }

    #[test]
    fn test_1262_example_3() {
        let nums = vec![1, 2, 3, 4, 4];
        let result = 12;

        assert_eq!(Solution::max_sum_div_three(nums), result);
    }
}

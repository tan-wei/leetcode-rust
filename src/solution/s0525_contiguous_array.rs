/**
 * [0525] Contiguous Array
 *
 * Given a binary array nums, return the maximum length of a contiguous subarray with an equal number of 0 and 1.
 *  
 * Example 1:
 *
 * Input: nums = [0,1]
 * Output: 2
 * Explanation: [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.
 *
 * Example 2:
 *
 * Input: nums = [0,1,0]
 * Output: 2
 * Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	nums[i] is either 0 or 1.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contiguous-array/
// discuss: https://leetcode.com/problems/contiguous-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut m = std::collections::HashMap::new();

        m.insert(0, -1);

        let mut result = 0;
        let mut sum = 0;

        nums.into_iter().enumerate().for_each(|(i, n)| {
            sum += if n == 0 { -1 } else { 1 };

            m.entry(sum)
                .and_modify(|v| {
                    result = result.max(i as i32 - *v);
                })
                .or_insert_with(|| i as i32);
        });

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0525_example_1() {
        let nums = vec![0, 1];
        let result = 2;

        assert_eq!(Solution::find_max_length(nums), result)
    }

    #[test]
    fn test_0525_example_2() {
        let nums = vec![0, 1, 0];
        let result = 2;

        assert_eq!(Solution::find_max_length(nums), result)
    }
}

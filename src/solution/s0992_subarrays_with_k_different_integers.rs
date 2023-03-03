/**
 * [0992] Subarrays with K Different Integers
 *
 * Given an integer array nums and an integer k, return the number of good subarrays of nums.
 * A good array is an array where the number of different integers in that array is exactly k.
 *
 * 	For example, [1,2,3,1,2] has 3 different integers: 1, 2, and 3.
 *
 * A subarray is a contiguous part of an array.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,1,2,3], k = 2
 * Output: 7
 * Explanation: Subarrays formed with exactly 2 different integers: [1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]
 *
 * Example 2:
 *
 * Input: nums = [1,2,1,3,4], k = 3
 * Output: 3
 * Explanation: Subarrays formed with exactly 3 different integers: [1,2,1,3], [2,1,3], [1,3,4].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	1 <= nums[i], k <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarrays-with-k-different-integers/
// discuss: https://leetcode.com/problems/subarrays-with-k-different-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/subarrays-with-k-different-integers/solutions/3226725/rust-implementation/
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let mut lookup = std::collections::HashMap::new();
        let mut result = 0;

        let mut start = 0;
        let mut start_k = 0;
        let k = k as usize;

        for &num in &nums {
            *lookup.entry(num).or_insert(0) += 1;

            if lookup.len() == k + 1 {
                lookup.remove(&nums[start_k]);
                start_k += 1;
                start = start_k;
            }

            if lookup.len() == k {
                while lookup[&nums[start_k]] > 1 {
                    *lookup.get_mut(&nums[start_k]).unwrap() -= 1;
                    start_k += 1;
                }
                result += start_k - start + 1;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0992_example_1() {
        let nums = vec![1, 2, 1, 2, 3];
        let k = 2;
        let result = 7;

        assert_eq!(Solution::subarrays_with_k_distinct(nums, k), result);
    }

    #[test]
    fn test_0992_example_2() {
        let nums = vec![1, 2, 1, 3, 4];
        let k = 3;
        let result = 3;

        assert_eq!(Solution::subarrays_with_k_distinct(nums, k), result);
    }
}

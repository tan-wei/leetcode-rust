/**
 * [2475] Number of Unequal Triplets in Array
 *
 * You are given a 0-indexed array of positive integers nums. Find the number of triplets (i, j, k) that meet the following conditions:
 *
 * 	0 <= i < j < k < nums.length
 * 	nums[i], nums[j], and nums[k] are pairwise distinct.
 *
 * 		In other words, nums[i] != nums[j], nums[i] != nums[k], and nums[j] != nums[k].
 *
 *
 *
 * Return the number of triplets that meet the conditions.
 *  
 * Example 1:
 *
 * Input: nums = [4,4,2,4,3]
 * Output: 3
 * Explanation: The following triplets meet the conditions:
 * - (0, 2, 4) because 4 != 2 != 3
 * - (1, 2, 4) because 4 != 2 != 3
 * - (2, 3, 4) because 2 != 4 != 3
 * Since there are 3 triplets, we return 3.
 * Note that (2, 0, 4) is not a valid triplet because 2 > 0.
 *
 * Example 2:
 *
 * Input: nums = [1,1,1,1,1]
 * Output: 0
 * Explanation: No triplets meet the conditions so we return 0.
 *
 *  
 * Constraints:
 *
 * 	3 <= nums.length <= 100
 * 	1 <= nums[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-unequal-triplets-in-array/
// discuss: https://leetcode.com/problems/number-of-unequal-triplets-in-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = nums.len();
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                if nums[i] == nums[j] {
                    continue;
                }
                for k in j + 1..len {
                    if nums[j] != nums[k] && nums[i] != nums[k] {
                        result += 1;
                    }
                }
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
    fn test_2475_example_1() {
        let nums = vec![4, 4, 2, 4, 3];

        let result = 3;

        assert_eq!(Solution::unequal_triplets(nums), result);
    }

    #[test]
    fn test_2475_example_2() {
        let nums = vec![1, 1, 1, 1, 1];

        let result = 0;

        assert_eq!(Solution::unequal_triplets(nums), result);
    }
}

/**
 * [1695] Maximum Erasure Value
 *
 * You are given an array of positive integers nums and want to erase a subarray containing unique elements. The score you get by erasing the subarray is equal to the sum of its elements.
 * Return the maximum score you can get by erasing exactly one subarray.
 * An array b is called to be a <span class="tex-font-style-it">subarray</span> of a if it forms a contiguous subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r] for some (l,r).
 *  
 * Example 1:
 *
 * Input: nums = [4,2,4,5,6]
 * Output: 17
 * Explanation: The optimal subarray here is [2,4,5,6].
 *
 * Example 2:
 *
 * Input: nums = [5,2,1,2,5,2,1,2,5]
 * Output: 8
 * Explanation: The optimal subarray here is [5,2,1] or [1,2,5].
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-erasure-value/
// discuss: https://leetcode.com/problems/maximum-erasure-value/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashSet::new();
        let (mut result, mut sum) = (0, 0);
        let mut lo = 0;

        for hi in 0..nums.len() {
            while map.contains(&nums[hi]) {
                sum -= nums[lo];
                map.remove(&nums[lo]);
                lo += 1
            }
            sum += nums[hi];
            result = result.max(sum);
            map.insert(nums[hi]);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1695_example_1() {
        let nums = vec![4, 2, 4, 5, 6];

        let result = 17;

        assert_eq!(Solution::maximum_unique_subarray(nums), result);
    }

    #[test]
    fn test_1695_example_2() {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];

        let result = 8;

        assert_eq!(Solution::maximum_unique_subarray(nums), result);
    }
}

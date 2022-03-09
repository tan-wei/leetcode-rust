/**
 * [0560] Subarray Sum Equals K
 *
 * Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.
 *  
 * Example 1:
 * Input: nums = [1,1,1], k = 2
 * Output: 2
 * Example 2:
 * Input: nums = [1,2,3], k = 3
 * Output: 2
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 2 * 10^4
 * 	-1000 <= nums[i] <= 1000
 * 	-10^7 <= k <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-sum-equals-k/
// discuss: https://leetcode.com/problems/subarray-sum-equals-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter()
            .fold(
                (std::collections::HashMap::from([(0, 1)]), 0, 0),
                |(mut map, sum, res), n| {
                    let sum = sum + n;
                    let res = res + map.get(&(sum - k)).unwrap_or(&0);
                    map.insert(sum, map.get(&sum).unwrap_or(&0) + 1);
                    (map, sum, res)
                },
            )
            .2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0560_example_1() {
        let nums = vec![1, 1, 1];
        let k = 2;
        let result = 2;

        assert_eq!(Solution::subarray_sum(nums, k), result);
    }

    #[test]
    fn test_0560_example_2() {
        let nums = vec![1, 2, 3];
        let k = 3;
        let result = 2;

        assert_eq!(Solution::subarray_sum(nums, k), result);
    }
}

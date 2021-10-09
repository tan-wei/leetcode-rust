/**
 * [368] Largest Divisible Subset
 *
 * Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
 *
 * 	answer[i] % answer[j] == 0, or
 * 	answer[j] % answer[i] == 0
 *
 * If there are multiple solutions, return any of them.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3]
 * Output: [1,2]
 * Explanation: [1,3] is also accepted.
 *
 * Example 2:
 *
 * Input: nums = [1,2,4,8]
 * Output: [1,2,4,8]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i] <= 2 * 10^9
 * 	All the integers in nums are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-divisible-subset/
// discuss: https://leetcode.com/problems/largest-divisible-subset/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        &nums.sort_unstable();

        let mut subsets = std::collections::HashMap::new();
        subsets.insert(-1, Vec::new());

        for n in nums.iter() {
            let mut s = subsets
                .iter()
                .filter(|(k, _)| *n % **k == 0)
                .map(|(_, v)| v)
                .max_by_key(|v| v.len())
                .unwrap()
                .to_vec();

            s.push(*n);

            subsets.insert(*n, s);
        }

        subsets.values().max_by_key(|v| v.len()).unwrap().to_vec()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0368_example_1() {
        let nums = vec![1, 2, 3];
        let result_1 = vec![1, 2];
        let result_2 = vec![1, 3];

        let result = Solution::largest_divisible_subset(nums);

        assert!(result == result_1 || result == result_2);
    }

    #[test]
    fn test_0368_example_2() {
        let nums = vec![1, 2, 4, 8];
        let result = vec![1, 2, 4, 8];

        assert_eq!(Solution::largest_divisible_subset(nums), result);
    }
}

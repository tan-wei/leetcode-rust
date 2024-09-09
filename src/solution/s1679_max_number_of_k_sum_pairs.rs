/**
 * [1679] Max Number of K-Sum Pairs
 *
 * You are given an integer array nums and an integer k.
 * In one operation, you can pick two numbers from the array whose sum equals k and remove them from the array.
 * Return the maximum number of operations you can perform on the array.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4], k = 5
 * Output: 2
 * Explanation: Starting with nums = [1,2,3,4]:
 * - Remove numbers 1 and 4, then nums = [2,3]
 * - Remove numbers 2 and 3, then nums = []
 * There are no more pairs that sum up to 5, hence a total of 2 operations.
 * Example 2:
 *
 * Input: nums = [3,1,3,4,3], k = 6
 * Output: 1
 * Explanation: Starting with nums = [3,1,3,4,3]:
 * - Remove the first two 3's, then nums = [1,4,3]
 * There are no more pairs that sum up to 6, hence a total of 1 operation.
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 * 	1 <= k <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-number-of-k-sum-pairs/
// discuss: https://leetcode.com/problems/max-number-of-k-sum-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut map = std::collections::HashMap::with_capacity(nums.len());

        nums.iter().for_each(|&x| {
            let e = map.entry(x).or_insert(0);
            match *e {
                0 => {
                    map.entry(k - x).and_modify(|x| *x += 1).or_insert(1);
                }
                _ => {
                    *e -= 1;
                    result += 1;
                }
            }
        });

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1679_example_1() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;

        let result = 2;

        assert_eq!(Solution::max_operations(nums, k), result);
    }

    #[test]
    fn test_1679_example_2() {
        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;

        let result = 1;

        assert_eq!(Solution::max_operations(nums, k), result);
    }
}

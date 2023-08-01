/**
 * [1248] Count Number of Nice Subarrays
 *
 * Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.
 *
 * Return the number of nice sub-arrays.
 *
 *  
 * Example 1:
 *
 *
 * Input: nums = [1,1,2,1,1], k = 3
 * Output: 2
 * Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [2,4,6], k = 1
 * Output: 0
 * Explanation: There is no odd numbers in the array.
 *
 *
 * Example 3:
 *
 *
 * Input: nums = [2,2,2,1,2,2,1,2,2,2], k = 2
 * Output: 16
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= nums.length <= 50000
 * 	1 <= nums[i] <= 10^5
 * 	1 <= k <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-number-of-nice-subarrays/
// discuss: https://leetcode.com/problems/count-number-of-nice-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut result: i32 = 0;
        let mut count: i32 = 0;
        map.insert(0, 1);
        for i in nums.iter() {
            if (i % 2 != 0) {
                count += 1;
            }
            if (map.contains_key(&(count - k))) {
                result += map.get(&(count - k)).unwrap();
            }
            if (map.contains_key(&count)) {
                map.insert(count, map.get(&count).unwrap() + 1);
            } else {
                map.insert(count, 1);
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
    fn test_1248_example_1() {
        let nums = vec![1, 1, 2, 1, 1];
        let k = 3;
        let result = 2;

        assert_eq!(Solution::number_of_subarrays(nums, k), result);
    }

    #[test]
    fn test_1248_example_2() {
        let nums = vec![2, 4, 6];
        let k = 1;
        let result = 0;

        assert_eq!(Solution::number_of_subarrays(nums, k), result);
    }

    #[test]
    fn test_1248_example_3() {
        let nums = vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2];
        let k = 2;
        let result = 16;

        assert_eq!(Solution::number_of_subarrays(nums, k), result);
    }
}

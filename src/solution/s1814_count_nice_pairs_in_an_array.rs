/**
 * [1814] Count Nice Pairs in an Array
 *
 * You are given an array nums that consists of non-negative integers. Let us define rev(x) as the reverse of the non-negative integer x. For example, rev(123) = 321, and rev(120) = 21. A pair of indices (i, j) is nice if it satisfies all of the following conditions:
 *
 * 	0 <= i < j < nums.length
 * 	nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])
 *
 * Return the number of nice pairs of indices. Since that number can be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: nums = [42,11,1,97]
 * Output: 2
 * Explanation: The two pairs are:
 *  - (0,3) : 42 + rev(97) = 42 + 79 = 121, 97 + rev(42) = 97 + 24 = 121.
 *  - (1,2) : 11 + rev(1) = 11 + 1 = 12, 1 + rev(11) = 1 + 11 = 12.
 *
 * Example 2:
 *
 * Input: nums = [13,10,35,24,76]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-nice-pairs-in-an-array/
// discuss: https://leetcode.com/problems/count-nice-pairs-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut hm = std::collections::HashMap::new();

        for v in nums {
            let (mut val_new, mut val) = (0, v);
            while val != 0 {
                val_new = val_new * 10 + val % 10;
                val /= 10;
            }
            *hm.entry(v - val_new).or_insert(0u64) += 1;
        }

        let mut result = 0;

        for v in hm.values() {
            if v > &1 {
                result = (result + v * (v - 1) / 2) % 1000000007;
            }
        }

        result as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1814_example_1() {
        let nums = vec![42, 11, 1, 97];

        let result = 2;

        assert_eq!(Solution::count_nice_pairs(nums), result);
    }

    #[test]
    fn test_1814_example_2() {
        let nums = vec![13, 10, 35, 24, 76];

        let result = 4;

        assert_eq!(Solution::count_nice_pairs(nums), result);
    }
}

/**
 * [1985] Find the Kth Largest Integer in the Array
 *
 * You are given an array of strings nums and an integer k. Each string in nums represents an integer without leading zeros.
 * Return the string that represents the k^th largest integer in nums.
 * Note: Duplicate numbers should be counted distinctly. For example, if nums is ["1","2","2"], "2" is the first largest integer, "2" is the second-largest integer, and "1" is the third-largest integer.
 *
 * Example 1:
 *
 * Input: nums = ["3","6","7","10"], k = 4
 * Output: "3"
 * Explanation:
 * The numbers in nums sorted in non-decreasing order are ["3","6","7","10"].
 * The 4^th largest integer in nums is "3".
 *
 * Example 2:
]*
 * Input: nums = ["2","21","12","1"], k = 3
 * Output: "2"
 * Explanation:
 * The numbers in nums sorted in non-decreasing order are ["1","2","12","21"].
 * The 3^rd largest integer in nums is "2".
 *
 * Example 3:
 *
 * Input: nums = ["0","0"], k = 2
 * Output: "0"
 * Explanation:
 * The numbers in nums sorted in non-decreasing order are ["0","0"].
 * The 2^nd largest integer in nums is "0".
 *
 *
 * Constraints:
 *
 * 	1 <= k <= nums.length <= 10^4
 * 	1 <= nums[i].length <= 100
 * 	nums[i] consists of only digits.
 * 	nums[i] will not have any leading zeros.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/
// discuss: https://leetcode.com/problems/find-the-kth-largest-integer-in-the-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut nums = nums;
        let k = (k - 1) as usize;

        nums.select_nth_unstable_by(k, |a, b| (b.len()).cmp(&a.len()).then_with(|| b.cmp(&a)));

        nums[k].clone()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1985_example_1() {
        let nums = vec_string!["3", "6", "7", "10"];
        let k = 4;

        let result = "3".to_string();

        assert_eq!(Solution::kth_largest_number(nums, k), result);
    }

    #[test]
    fn test_1985_example_2() {
        let nums = vec_string!["2", "21", "12", "1"];
        let k = 3;

        let result = "2".to_string();

        assert_eq!(Solution::kth_largest_number(nums, k), result);
    }

    #[test]
    fn test_1985_example_3() {
        let nums = vec_string!["0", "0"];
        let k = 2;

        let result = "0".to_string();

        assert_eq!(Solution::kth_largest_number(nums, k), result);
    }
}

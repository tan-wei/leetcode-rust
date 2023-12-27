/**
 * [1437] Check If All 1's Are at Least Length K Places Away
 *
 * Given an binary array nums and an integer k, return true if all 1's are at least k places away from each other, otherwise return false.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/15/sample_1_1791.png" style="width: 428px; height: 181px;" />
 * Input: nums = [1,0,0,0,1,0,0,1], k = 2
 * Output: true
 * Explanation: Each of the 1s are at least 2 places away from each other.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/15/sample_2_1791.png" style="width: 320px; height: 173px;" />
 * Input: nums = [1,0,0,1,0,1], k = 2
 * Output: false
 * Explanation: The second 1 and third 1 are only one apart from each other.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= k <= nums.length
 * 	nums[i] is 0 or 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/
// discuss: https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        (0..nums.len())
            .filter(|&i| nums[i] == 1)
            .try_fold(None, |prev, i| {
                if let Some(p) = prev {
                    if i - p <= k as usize {
                        return None;
                    }
                }
                Some(Some(i))
            })
            .is_some()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1437_example_1() {
        let nums = vec![1, 0, 0, 0, 1, 0, 0, 1];
        let k = 2;

        let result = true;

        assert_eq!(Solution::k_length_apart(nums, k), result);
    }

    #[test]
    fn test_1437_example_2() {
        let nums = vec![1, 0, 0, 1, 0, 1];
        let k = 2;

        let result = false;

        assert_eq!(Solution::k_length_apart(nums, k), result);
    }
}

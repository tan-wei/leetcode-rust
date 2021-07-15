/**
 * [220] Contains Duplicate III
 *
 * Given an integer array nums and two integers k and t, return true if there are two distinct indices i and j in the array such that abs(nums[i] - nums[j]) <= t and abs(i - j) <= k.
 *  
 * Example 1:
 * Input: nums = [1,2,3,1], k = 3, t = 0
 * Output: true
 * Example 2:
 * Input: nums = [1,0,1,1], k = 1, t = 2
 * Output: true
 * Example 3:
 * Input: nums = [1,5,9,1,5,9], k = 2, t = 3
 * Output: false
 *  
 * Constraints:
 *
 * 	0 <= nums.length <= 2 * 10^4
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 	0 <= k <= 10^4
 * 	0 <= t <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate-iii/
// discuss: https://leetcode.com/problems/contains-duplicate-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        let t = t as i64;
        nums.iter().enumerate().any(|(i, &x)| {
            let x = x as i64;
            nums[i.saturating_sub(k)..i]
                .iter()
                .any(|&y| (y as i64 - x).abs() <= t)
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0220_example_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        let t = 0;
        let result = true;

        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, k, t),
            result
        );
    }

    #[test]
    fn test_0220_example_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        let t = 2;
        let result = true;

        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, k, t),
            result
        );
    }

    #[test]
    fn test_0220_example_3() {
        let nums = vec![1, 5, 9, 1, 5, 9];
        let k = 2;
        let t = 3;
        let result = false;

        assert_eq!(
            Solution::contains_nearby_almost_duplicate(nums, k, t),
            result
        );
    }
}

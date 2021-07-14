/**
 * [219] Contains Duplicate II
 *
 * Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,1], k = 3
 * Output: true
 *
 * Example 2:
 *
 * Input: nums = [1,0,1,1], k = 1
 * Output: true
 *
 * Example 3:
 *
 * Input: nums = [1,2,3,1,2,3], k = 2
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	0 <= k <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/contains-duplicate-ii/
// discuss: https://leetcode.com/problems/contains-duplicate-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        nums.iter()
            .enumerate()
            .try_fold(
                std::collections::HashMap::with_capacity(nums.len()),
                |mut m, (i, x)| {
                    let mut prev = m.entry(x).or_insert(i);
                    let dist = i - *prev;

                    if dist != 0 && dist <= k as usize {
                        None
                    } else {
                        *prev = i;
                        Some(m)
                    }
                },
            )
            .is_none()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0219_example_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        let result = true;

        assert_eq!(Solution::contains_nearby_duplicate(nums, k), result);
    }

    #[test]
    fn test_0219_example_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        let result = true;

        assert_eq!(Solution::contains_nearby_duplicate(nums, k), result);
    }

    #[test]
    fn test_0219_example_3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        let result = false;

        assert_eq!(Solution::contains_nearby_duplicate(nums, k), result);
    }
}

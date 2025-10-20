/**
 * [2206] Divide Array Into Equal Pairs
 *
 * You are given an integer array nums consisting of 2 * n integers.
 * You need to divide nums into n pairs such that:
 *
 * 	Each element belongs to exactly one pair.
 * 	The elements present in a pair are equal.
 *
 * Return true if nums can be divided into n pairs, otherwise return false.
 *  
 * Example 1:
 *
 * Input: nums = [3,2,3,2,2,2]
 * Output: true
 * Explanation:
 * There are 6 elements in nums, so they should be divided into 6 / 2 = 3 pairs.
 * If nums is divided into the pairs (2, 2), (3, 3), and (2, 2), it will satisfy all the conditions.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4]
 * Output: false
 * Explanation:
 * There is no way to divide nums into 4 / 2 = 2 pairs such that the pairs satisfy every condition.
 *
 *  
 * Constraints:
 *
 * 	nums.length == 2 * n
 * 	1 <= n <= 500
 * 	1 <= nums[i] <= 500
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-array-into-equal-pairs/
// discuss: https://leetcode.com/problems/divide-array-into-equal-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        nums.iter()
            .fold(vec![true; 501], |mut map, e| {
                map[*e as usize] = !map[*e as usize];
                map
            })
            .iter()
            .all(|e| *e == true)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2206_example_1() {
        let nums = vec![3, 2, 3, 2, 2, 2];

        let result = true;

        assert_eq!(Solution::divide_array(nums), result);
    }

    #[test]
    fn test_2206_example_2() {
        let nums = vec![1, 2, 3, 4];

        let result = false;

        assert_eq!(Solution::divide_array(nums), result);
    }
}

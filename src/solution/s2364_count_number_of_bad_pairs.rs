/**
 * [2364] Count Number of Bad Pairs
 *
 * You are given a 0-indexed integer array nums. A pair of indices (i, j) is a bad pair if i < j and j - i != nums[j] - nums[i].
 * Return the total number of bad pairs in nums.
 *  
 * Example 1:
 *
 * Input: nums = [4,1,3,3]
 * Output: 5
 * Explanation: The pair (0, 1) is a bad pair since 1 - 0 != 1 - 4.
 * The pair (0, 2) is a bad pair since 2 - 0 != 3 - 4, 2 != -1.
 * The pair (0, 3) is a bad pair since 3 - 0 != 3 - 4, 3 != -1.
 * The pair (1, 2) is a bad pair since 2 - 1 != 3 - 1, 1 != 2.
 * The pair (2, 3) is a bad pair since 3 - 2 != 3 - 3, 1 != 0.
 * There are a total of 5 bad pairs, so we return 5.
 *
 * Example 2:
 *
 * Input: nums = [1,2,3,4,5]
 * Output: 0
 * Explanation: There are no bad pairs.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-number-of-bad-pairs/
// discuss: https://leetcode.com/problems/count-number-of-bad-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        nums.into_iter()
            .enumerate()
            .map(|(i, num)| (i, num - i as i32))
            .scan(std::collections::HashMap::new(), |map, (i, num)| {
                let same_count = map.entry(num).or_insert(0);
                let diff_count = i as i32 - *same_count;
                *same_count += 1;
                Some(diff_count as i64)
            })
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2364_example_1() {
        let nums = vec![4, 1, 3, 3];

        let result = 5;

        assert_eq!(Solution::count_bad_pairs(nums), result);
    }

    #[test]
    fn test_2364_example_2() {
        let nums = vec![1, 2, 3, 4, 5];

        let result = 0;

        assert_eq!(Solution::count_bad_pairs(nums), result);
    }
}

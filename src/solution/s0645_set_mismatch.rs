/**
 * [0645] Set Mismatch
 *
 * You have a set of integers s, which originally contains all the numbers from 1 to n. Unfortunately, due to some error, one of the numbers in s got duplicated to another number in the set, which results in repetition of one number and loss of another number.
 * You are given an integer array nums representing the data status of this set after the error.
 * Find the number that occurs twice and the number that is missing and return them in the form of an array.
 *  
 * Example 1:
 * Input: nums = [1,2,2,4]
 * Output: [2,3]
 * Example 2:
 * Input: nums = [1,1]
 * Output: [1,2]
 *  
 * Constraints:
 *
 * 	2 <= nums.length <= 10^4
 * 	1 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/set-mismatch/
// discuss: https://leetcode.com/problems/set-mismatch/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let miss_xor_dup = nums
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &x)| acc ^ (i as i32 + 1) ^ x);

        let mut set = std::collections::HashSet::new();
        nums.into_iter()
            .find(|&x| !set.insert(x))
            .map(|x| vec![x, miss_xor_dup ^ x])
            .unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0645_example_1() {
        let nums = vec![1, 2, 2, 4];
        let result = vec![2, 3];

        assert_eq!(Solution::find_error_nums(nums), result);
    }

    #[test]
    fn test_0645_example_2() {
        let nums = vec![1, 1];
        let result = vec![1, 2];

        assert_eq!(Solution::find_error_nums(nums), result);
    }
}

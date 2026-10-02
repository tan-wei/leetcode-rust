/**
 * [2615] Sum of Distances
 *
 * You are given a 0-indexed integer array nums.
 * There exists an array arr of length nums.length, where arr[i] is the sum of |i - j| over all j such that nums[j] == nums[i] and j != i. If there is no such j, set arr[i] to be 0.
 * Return the array arr.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,1,1,2]
 * Output: [5,0,3,4,0]
 * Explanation:
 * When i = 0, nums[0] == nums[2] and nums[0] == nums[3]. Therefore, arr[0] = |0 - 2| + |0 - 3| = 5.
 * When i = 1, arr[1] = 0 because there is no other index with value 3.
 * When i = 2, nums[2] == nums[0] and nums[2] == nums[3]. Therefore, arr[2] = |2 - 0| + |2 - 3| = 3.
 * When i = 3, nums[3] == nums[0] and nums[3] == nums[2]. Therefore, arr[3] = |3 - 0| + |3 - 2| = 4.
 * When i = 4, arr[4] = 0 because there is no other index with value 2.
 *
 * Example 2:
 *
 * Input: nums = [0,5,3]
 * Output: [0,0,0]
 * Explanation: Since each element in nums is distinct, arr[i] = 0 for all i.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^9
 *
 *  
 * Note: This question is the same as  2121: Intervals Between Identical Elements.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-distances/
// discuss: https://leetcode.com/problems/sum-of-distances/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sum-of-distances/solutions/8062295/rustelixir-2-passes-by-minamikaze392-34uh/
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let n = nums.len();
        let mut result = vec![0i64; n];

        let mut hash = std::collections::HashMap::<i32, (i64, i64)>::new();
        for i in 0..n {
            let entry = hash.entry(nums[i]).or_insert((0, 0));
            result[i] = i as i64 * entry.0 - entry.1;
            entry.0 += 1;
            entry.1 += i as i64;
        }
        for i in 0..n {
            let entry = hash.entry(nums[i]).or_insert((0, 0));
            entry.0 -= 1;
            entry.1 -= i as i64;
            result[i] += entry.1 - i as i64 * entry.0;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2615_example_1() {
        let nums = vec![1, 3, 1, 1, 2];

        let result = vec![5, 0, 3, 4, 0];

        assert_eq!(Solution::distance(nums), result);
    }

    #[test]
    fn test_2615_example_2() {
        let nums = vec![0, 5, 3];

        let result = vec![0, 0, 0];

        assert_eq!(Solution::distance(nums), result);
    }
}

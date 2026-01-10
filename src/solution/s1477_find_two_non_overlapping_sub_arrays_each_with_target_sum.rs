/**
 * [1477] Find Two Non-overlapping Sub-arrays Each With Target Sum
 *
 * You are given an array of integers arr and an integer target.
 * You have to find two non-overlapping sub-arrays of arr each with a sum equal target. There can be multiple answers so you have to find an answer where the sum of the lengths of the two sub-arrays is minimum.
 * Return the minimum sum of the lengths of the two required sub-arrays, or return -1 if you cannot find such two sub-arrays.
 *  
 * Example 1:
 *
 * Input: arr = [3,2,2,4,3], target = 3
 * Output: 2
 * Explanation: Only two sub-arrays have sum = 3 ([3] and [3]). The sum of their lengths is 2.
 *
 * Example 2:
 *
 * Input: arr = [7,3,4,7], target = 7
 * Output: 2
 * Explanation: Although we have three non-overlapping sub-arrays of sum = 7 ([7], [3,4] and [7]), but we will choose the first and third sub-arrays as the sum of their lengths is 2.
 *
 * Example 3:
 *
 * Input: arr = [4,3,2,6,2,3,4], target = 6
 * Output: -1
 * Explanation: We have only one sub-array of sum = 6.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^5
 * 	1 <= arr[i] <= 1000
 * 	1 <= target <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/
// discuss: https://leetcode.com/problems/find-two-non-overlapping-sub-arrays-each-with-target-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![i32::MAX; arr.len() + 1];
        let mut result = i32::MAX;
        let mut acc = 0;
        let mut i = 0;
        for j in 0..arr.len() {
            acc += arr[j];
            while acc > target {
                acc -= arr[i];
                i += 1;
            }
            if acc == target {
                let len = (j - i + 1) as i32;
                if i > 0 {
                    result = dp[i - 1].saturating_add(len).min(result);
                }
                dp[j] = dp[j].min(len);
            }
            dp[j + 1] = dp[j];
        }

        if result == i32::MAX { -1 } else { result }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1477_example_1() {
        let arr = vec![3, 2, 2, 4, 3];
        let target = 3;

        let result = 2;

        assert_eq!(Solution::min_sum_of_lengths(arr, target), result);
    }

    #[test]
    fn test_1477_example_2() {
        let arr = vec![7, 3, 4, 7];
        let target = 7;

        let result = 2;

        assert_eq!(Solution::min_sum_of_lengths(arr, target), result);
    }

    #[test]
    fn test_1477_example_3() {
        let arr = vec![4, 3, 2, 6, 2, 3, 4];
        let target = 6;

        let result = -1;

        assert_eq!(Solution::min_sum_of_lengths(arr, target), result);
    }
}

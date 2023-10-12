/**
 * [1343] Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold
 *
 * Given an array of integers arr and two integers k and threshold, return the number of sub-arrays of size k and average greater than or equal to threshold.
 *  
 * Example 1:
 *
 * Input: arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
 * Output: 3
 * Explanation: Sub-arrays [2,5,5],[5,5,5] and [5,5,8] have averages 4, 5 and 6 respectively. All other sub-arrays of size 3 have averages less than 4 (the threshold).
 *
 * Example 2:
 *
 * Input: arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
 * Output: 6
 * Explanation: The first 6 sub-arrays of size 3 have averages greater than 5. Note that averages are not integers.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^5
 * 	1 <= arr[i] <= 10^4
 * 	1 <= k <= arr.length
 * 	0 <= threshold <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/
// discuss: https://leetcode.com/problems/number-of-sub-arrays-of-size-k-and-average-greater-than-or-equal-to-threshold/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut count: i32 = 0;

        let mut window_sum: i32 = 0;
        let mut idx: usize = 0;

        while idx < k as usize {
            window_sum += arr[idx];
            idx += 1;
        }

        if window_sum / k >= threshold {
            count += 1
        }

        while idx < arr.len() {
            window_sum += arr[idx];
            window_sum -= arr[idx - k as usize];
            if window_sum / k >= threshold {
                count += 1
            }
            idx += 1;
        }

        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1343_example_1() {
        let arr = vec![2, 2, 2, 2, 5, 5, 5, 8];
        let k = 3;
        let threshold = 4;

        let result = 3;

        assert_eq!(Solution::num_of_subarrays(arr, k, threshold), result);
    }

    #[test]
    fn test_1343_example_2() {
        let arr = vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3];
        let k = 3;
        let threshold = 5;

        let result = 6;

        assert_eq!(Solution::num_of_subarrays(arr, k, threshold), result);
    }
}

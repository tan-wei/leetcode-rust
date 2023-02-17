/**
 * [0978] Longest Turbulent Subarray
 *
 * Given an integer array arr, return the length of a maximum size turbulent subarray of arr.
 * A subarray is turbulent if the comparison sign flips between each adjacent pair of elements in the subarray.
 * More formally, a subarray [arr[i], arr[i + 1], ..., arr[j]] of arr is said to be turbulent if and only if:
 *
 * 	For i <= k < j:
 *
 * 		arr[k] > arr[k + 1] when k is odd, and
 * 		arr[k] < arr[k + 1] when k is even.
 *
 *
 * 	Or, for i <= k < j:
 *
 * 		arr[k] > arr[k + 1] when k is even, and
 * 		arr[k] < arr[k + 1] when k is odd.
 *
 *
 *
 *  
 * Example 1:
 *
 * Input: arr = [9,4,2,10,7,8,8,1,9]
 * Output: 5
 * Explanation: arr[1] > arr[2] < arr[3] > arr[4] < arr[5]
 *
 * Example 2:
 *
 * Input: arr = [4,8,12,16]
 * Output: 2
 *
 * Example 3:
 *
 * Input: arr = [100]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 4 * 10^4
 * 	0 <= arr[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-turbulent-subarray/
// discuss: https://leetcode.com/problems/longest-turbulent-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let (mut inc, mut dec, mut result) = (1, 1, 1);

        for i in 1..arr.len() {
            if (arr[i] < arr[i - 1]) {
                dec = inc + 1;
                inc = 1;
            } else if (arr[i] > arr[i - 1]) {
                inc = dec + 1;
                dec = 1;
            } else {
                inc = 1;
                dec = 1;
            }
            result = std::cmp::max(result, std::cmp::max(dec, inc));
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0978_example_1() {
        let arr = vec![9, 4, 2, 10, 7, 8, 8, 1, 9];
        let result = 5;

        assert_eq!(Solution::max_turbulence_size(arr), result);
    }

    #[test]
    fn test_0978_example_2() {
        let arr = vec![4, 8, 12, 16];
        let result = 2;

        assert_eq!(Solution::max_turbulence_size(arr), result);
    }

    #[test]
    fn test_0978_example_3() {
        let arr = vec![100];
        let result = 1;

        assert_eq!(Solution::max_turbulence_size(arr), result);
    }
}

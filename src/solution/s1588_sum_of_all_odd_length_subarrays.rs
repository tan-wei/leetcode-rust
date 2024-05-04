/**
 * [1588] Sum of All Odd Length Subarrays
 *
 * Given an array of positive integers arr, return the sum of all possible odd-length subarrays of arr.
 * A subarray is a contiguous subsequence of the array.
 *  
 * Example 1:
 *
 * Input: arr = [1,4,2,5,3]
 * Output: 58
 * Explanation: The odd-length subarrays of arr and their sums are:
 * [1] = 1
 * [4] = 4
 * [2] = 2
 * [5] = 5
 * [3] = 3
 * [1,4,2] = 7
 * [4,2,5] = 11
 * [2,5,3] = 10
 * [1,4,2,5,3] = 15
 * If we add all these together we get 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58
 * Example 2:
 *
 * Input: arr = [1,2]
 * Output: 3
 * Explanation: There are only 2 subarrays of odd length, [1] and [2]. Their sum is 3.
 * Example 3:
 *
 * Input: arr = [10,11,12]
 * Output: 66
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 100
 * 	1 <= arr[i] <= 1000
 *
 *  
 * Follow up:
 * Could you solve this problem in O(n) time complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-all-odd-length-subarrays/
// discuss: https://leetcode.com/problems/sum-of-all-odd-length-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        (1..=arr.len()).step_by(2).fold(0, |mut init, le| {
            init += arr.windows(le).fold(0, |mut sum, val| {
                sum += val.iter().sum::<i32>();
                sum
            });
            init
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1588_example_1() {
        let arr = vec![1, 4, 2, 5, 3];

        let result = 58;

        assert_eq!(Solution::sum_odd_length_subarrays(arr), result);
    }

    #[test]
    fn test_1588_example_2() {
        let arr = vec![1, 2];

        let result = 3;

        assert_eq!(Solution::sum_odd_length_subarrays(arr), result);
    }

    #[test]
    fn test_1588_example_3() {
        let arr = vec![10, 11, 12];

        let result = 66;

        assert_eq!(Solution::sum_odd_length_subarrays(arr), result);
    }
}

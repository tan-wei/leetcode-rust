/**
 * [1191] K-Concatenation Maximum Sum
 *
 * Given an integer array arr and an integer k, modify the array by repeating it k times.
 * For example, if arr = [1, 2] and k = 3 then the modified array will be [1, 2, 1, 2, 1, 2].
 * Return the maximum sub-array sum in the modified array. Note that the length of the sub-array can be 0 and its sum in that case is 0.
 * As the answer can be very large, return the answer modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: arr = [1,2], k = 3
 * Output: 9
 *
 * Example 2:
 *
 * Input: arr = [1,-2,1], k = 5
 * Output: 2
 *
 * Example 3:
 *
 * Input: arr = [-1,-2], k = 7
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^5
 * 	1 <= k <= 10^5
 * 	-10^4 <= arr[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-concatenation-maximum-sum/
// discuss: https://leetcode.com/problems/k-concatenation-maximum-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let arr = arr.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mut m_sum = 0;
        let sz = arr.len() as i64;
        let mut sum = 0;
        for i in 0..std::cmp::min(2, k) * sz {
            sum = std::cmp::max(sum + arr[(i % sz) as usize], arr[(i % sz) as usize]);
            m_sum = std::cmp::max(m_sum, sum);
        }
        ((m_sum + std::cmp::max(0, arr.iter().sum::<i64>()) * std::cmp::max(0, k - 2)) % 1000000007)
            as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1191_example_1() {
        let arr = vec![1, 2];
        let k = 3;
        let result = 9;

        assert_eq!(Solution::k_concatenation_max_sum(arr, k), result);
    }

    #[test]
    fn test_1191_example_2() {
        let arr = vec![1, -2, 1];
        let k = 5;
        let result = 2;

        assert_eq!(Solution::k_concatenation_max_sum(arr, k), result);
    }

    #[test]
    fn test_1191_example_3() {
        let arr = vec![-1, -2];
        let k = 7;
        let result = 0;

        assert_eq!(Solution::k_concatenation_max_sum(arr, k), result);
    }
}

/**
 * [0873] Length of Longest Fibonacci Subsequence
 *
 * A sequence x1, x2, ..., xn is Fibonacci-like if:
 *
 * 	n >= 3
 * 	xi + xi+1 == xi+2 for all i + 2 <= n
 *
 * Given a strictly increasing array arr of positive integers forming a sequence, return the length of the longest Fibonacci-like subsequence of arr. If one does not exist, return 0.
 * A subsequence is derived from another sequence arr by deleting any number of elements (including none) from arr, without changing the order of the remaining elements. For example, [3, 5, 8] is a subsequence of [3, 4, 5, 6, 7, 8].
 *  
 * Example 1:
 *
 * Input: arr = [1,2,3,4,5,6,7,8]
 * Output: 5
 * Explanation: The longest subsequence that is fibonacci-like: [1,2,3,5,8].
 * Example 2:
 *
 * Input: arr = [1,3,7,11,12,14,18]
 * Output: 3
 * Explanation: The longest subsequence that is fibonacci-like: [1,11,12], [3,11,14] or [7,11,18].
 *  
 * Constraints:
 *
 * 	3 <= arr.length <= 1000
 * 	1 <= arr[i] < arr[i + 1] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/
// discuss: https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/solutions/828181/rust-translated/
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut dp = vec![vec![0; arr.len()]; arr.len()];
        let mut index = std::collections::HashMap::<i32, i32>::new();
        for j in 0..arr.len() {
            index.insert(arr[j], j as i32);
            for i in 0..j {
                let &mut k = index.entry(arr[j] - arr[i]).or_insert(-1);
                dp[i][j] = if arr[j] - arr[i] < arr[i] && k >= 0 {
                    dp[k as usize][i] + 1
                } else {
                    2
                };
                result = std::cmp::max(result, dp[i][j]);
            }
        }
        if result > 2 {
            result
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0873_example_1() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = 5;

        assert_eq!(Solution::len_longest_fib_subseq(arr), result);
    }

    #[test]
    fn test_0873_example_2() {
        let arr = vec![1, 3, 7, 11, 12, 14, 18];
        let result = 3;

        assert_eq!(Solution::len_longest_fib_subseq(arr), result);
    }
}

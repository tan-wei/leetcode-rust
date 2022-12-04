/**
 * [0903] Valid Permutations for DI Sequence
 *
 * You are given a string s of length n where s[i] is either:
 *
 * 	'D' means decreasing, or
 * 	'I' means increasing.
 *
 * A permutation perm of n + 1 integers of all the integers in the range [0, n] is called a valid permutation if for all valid i:
 *
 * 	If s[i] == 'D', then perm[i] > perm[i + 1], and
 * 	If s[i] == 'I', then perm[i] < perm[i + 1].
 *
 * Return the number of valid permutations perm. Since the answer may be large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: s = "DID"
 * Output: 5
 * Explanation: The 5 valid permutations of (0, 1, 2, 3) are:
 * (1, 0, 3, 2)
 * (2, 0, 3, 1)
 * (2, 1, 3, 0)
 * (3, 0, 2, 1)
 * (3, 1, 2, 0)
 *
 * Example 2:
 *
 * Input: s = "D"
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	n == s.length
 * 	1 <= n <= 200
 * 	s[i] is either 'I' or 'D'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-permutations-for-di-sequence/
// discuss: https://leetcode.com/problems/valid-permutations-for-di-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i32 = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/valid-permutations-for-di-sequence/solutions/168278/C++JavaPython-DP-Solution-O(N2)/
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![1; n + 1];
        let mut dp2 = vec![0; n];

        for i in 0..n {
            if s[i..].chars().next().unwrap() == 'I' {
                let mut cur = 0;
                for j in 0..n - i {
                    cur = (cur + dp[j]) % MOD;
                    dp2[j] = cur;
                }
            } else {
                let mut cur = 0;
                for j in (0..n - i).rev() {
                    cur = (cur + dp[j + 1]) % MOD;
                    dp2[j] = cur;
                }
            }
            dp[..n].copy_from_slice(&dp2[..n]);
        }
        dp[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0903_example_1() {
        let s = "DID".to_string();
        let result = 5;

        assert_eq!(Solution::num_perms_di_sequence(s), result);
    }

    #[test]
    fn test_0903_example_2() {
        let s = "D".to_string();
        let result = 1;

        assert_eq!(Solution::num_perms_di_sequence(s), result);
    }
}

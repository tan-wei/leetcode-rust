/**
 * [1531] String Compression II
 *
 * <a href="http://en.wikipedia.org/wiki/Run-length_encoding">Run-length encoding</a> is a string compression method that works by replacing consecutive identical characters (repeated 2 or more times) with the concatenation of the character and the number marking the count of the characters (length of the run). For example, to compress the string "aabccc" we replace <font face="monospace">"aa"</font> by <font face="monospace">"a2"</font> and replace <font face="monospace">"ccc"</font> by <font face="monospace">"c3"</font>. Thus the compressed string becomes <font face="monospace">"a2bc3".</font>
 * Notice that in this problem, we are not adding '1' after single characters.
 * Given a string s and an integer k. You need to delete at most k characters from s such that the run-length encoded version of s has minimum length.
 * Find the minimum length of the run-length encoded version of s after deleting at most k characters.
 *  
 * Example 1:
 *
 * Input: s = "aaabcccd", k = 2
 * Output: 4
 * Explanation: Compressing s without deleting anything will give us "a3bc3d" of length 6. Deleting any of the characters 'a' or 'c' would at most decrease the length of the compressed string to 5, for instance delete 2 'a' then we will have s = "abcccd" which compressed is abc3d. Therefore, the optimal way is to delete 'b' and 'd', then the compressed version of s will be "a3c3" of length 4.
 * Example 2:
 *
 * Input: s = "aabbaa", k = 2
 * Output: 2
 * Explanation: If we delete both 'b' characters, the resulting compressed string would be "a4" of length 2.
 *
 * Example 3:
 *
 * Input: s = "aaaaaaaaaaa", k = 0
 * Output: 3
 * Explanation: Since k is zero, we cannot delete anything. The compressed string is "a11" of length 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	0 <= k <= s.length
 * 	s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-compression-ii/
// discuss: https://leetcode.com/problems/string-compression-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/string-compression-ii/solutions/2709343/rust-bottom-up-dp-100-time-and-memory-25-lines-of-idiomatic-code-commented-version/
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let mut dp = [[i32::MAX - 101; 101]; 101];

        dp[0].fill(0);

        for (i, c) in s.bytes().enumerate().map(|(i, c)| (i + 1, c)) {
            for to_rem in 0..=(i).min(k as usize) {
                if to_rem > 0 {
                    dp[i][to_rem] = dp[i - 1][to_rem - 1];
                }

                let mut count = 0_i32;
                let mut removed = 0;
                for (j, c_) in s.bytes().enumerate().take(i).rev() {
                    if c_ == c {
                        count += 1;
                    } else {
                        removed += 1;
                        if removed > to_rem {
                            break;
                        }
                    }
                    let f = |c: i32| (c as f32).log10().floor() as i32 + 1 + (c != 1) as i32;
                    dp[i][to_rem] = dp[i][to_rem].min(dp[j][to_rem - removed] + f(count));
                }
            }
        }
        dp[s.len()][k as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1531_example_1() {
        let s = "aaabcccd".to_string();
        let k = 2;

        let result = 4;

        assert_eq!(Solution::get_length_of_optimal_compression(s, k), result);
    }

    #[test]
    fn test_1531_example_2() {
        let s = "aabbaa".to_string();
        let k = 2;

        let result = 2;

        assert_eq!(Solution::get_length_of_optimal_compression(s, k), result);
    }

    #[test]
    fn test_1531_example_3() {
        let s = "aaaaaaaaaaa".to_string();
        let k = 0;

        let result = 3;

        assert_eq!(Solution::get_length_of_optimal_compression(s, k), result);
    }
}

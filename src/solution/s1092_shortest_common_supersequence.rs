/**
 * [1092] Shortest Common Supersequence
 *
 * Given two strings str1 and str2, return the shortest string that has both str1 and str2 as subsequences. If there are multiple valid strings, return any of them.
 * A string s is a subsequence of string t if deleting some number of characters from t (possibly 0) results in the string s.
 *  
 * Example 1:
 *
 * Input: str1 = "abac", str2 = "cab"
 * Output: "cabac"
 * Explanation:
 * str1 = "abac" is a subsequence of "cabac" because we can delete the first "c".
 * str2 = "cab" is a subsequence of "cabac" because we can delete the last "ac".
 * The answer provided is the shortest such string that satisfies these properties.
 *
 * Example 2:
 *
 * Input: str1 = "aaaaaaaa", str2 = "aaaaaaaa"
 * Output: "aaaaaaaa"
 *
 *  
 * Constraints:
 *
 * 	1 <= str1.length, str2.length <= 1000
 * 	str1 and str2 consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-common-supersequence/
// discuss: https://leetcode.com/problems/shortest-common-supersequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/shortest-common-supersequence/solutions/494586/rust-4ms-9-7mb-100/
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let m = str1.len();
        let n = str2.len();
        match (m, n) {
            (0, _) => str2,
            (_, 0) => str1,
            (_, _) => {
                let s1 = str1.chars().collect::<Vec<char>>();
                let s2 = str2.chars().collect::<Vec<char>>();

                let mut dp = vec![vec![0; n + 1]; m + 1];
                for i in 0..m + 1 {
                    for j in 0..n + 1 {
                        if i == 0 {
                            dp[i][j] = j;
                        } else if j == 0 {
                            dp[i][j] = i;
                        } else if s1[i - 1] == s2[j - 1] {
                            dp[i][j] = 1 + dp[i - 1][j - 1];
                        } else {
                            dp[i][j] = 1 + std::cmp::min(dp[i - 1][j], dp[i][j - 1]);
                        }
                    }
                }
                let mut l = dp[m][n];
                let mut result = vec![' '; l];
                let mut i = m;
                let mut j = n;
                while i > 0 && j > 0 {
                    l -= 1;
                    if s1[i - 1] == s2[j - 1] {
                        result[l] = s1[i - 1];
                        i -= 1;
                        j -= 1;
                    } else if dp[i - 1][j] < dp[i][j - 1] {
                        result[l] = s1[i - 1];
                        i -= 1;
                    } else {
                        result[l] = s2[j - 1];
                        j -= 1;
                    }
                }
                while i > 0 {
                    l -= 1;
                    result[l] = s1[i - 1];
                    i -= 1;
                }
                while j > 0 {
                    l -= 1;
                    result[l] = s2[j - 1];
                    j -= 1;
                }
                result.into_iter().collect()
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1092_example_1() {
        let str1 = "abac".to_string();
        let str2 = "cab".to_string();
        let result = "cabac".to_string();

        assert_eq!(Solution::shortest_common_supersequence(str1, str2), result);
    }

    #[test]
    fn test_1092_example_2() {
        let str1 = "aaaaaaaa".to_string();
        let str2 = "aaaaaaaa".to_string();
        let result = "aaaaaaaa".to_string();

        assert_eq!(Solution::shortest_common_supersequence(str1, str2), result);
    }
}

/**
 * [132] Palindrome Partitioning II
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome.
 * Return the minimum cuts needed for a palindrome partitioning of s.
 *  
 * Example 1:
 *
 * Input: s = "aab"
 * Output: 1
 * Explanation: The palindrome partitioning ["aa","b"] could be produced using 1 cut.
 *
 * Example 2:
 *
 * Input: s = "a"
 * Output: 0
 *
 * Example 3:
 *
 * Input: s = "ab"
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 2000
 * 	s consists of lower-case English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning-ii/
// discuss: https://leetcode.com/problems/palindrome-partitioning-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let n = s.len();
        let s: Vec<_> = s.chars().collect();
        let mut pal = vec![vec![false; n]; n];
        let mut d = vec![0; n];
        for i in (0..=n - 1).rev() {
            d[i] = n - i - 1;
            for j in i..n {
                if (s[i] == s[j] && (j - i < 2 || pal[i + 1][j - 1])) {
                    pal[i][j] = true;
                    if j == n - 1 {
                        d[i] = 0;
                    } else if d[j + 1] + 1 < d[i] {
                        d[i] = d[j + 1] + 1;
                    }
                }
            }
        }
        return d[0] as i32;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0132_example_1() {
        let s = "aab".to_string();
        let result = 1;

        assert_eq!(Solution::min_cut(s), result);
    }

    #[test]
    fn test_0132_example_2() {
        let s = "a".to_string();
        let result = 0;

        assert_eq!(Solution::min_cut(s), result);
    }

    #[test]
    fn test_0132_example_3() {
        let s = "ab".to_string();
        let result = 1;

        assert_eq!(Solution::min_cut(s), result);
    }
}

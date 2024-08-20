/**
 * [1653] Minimum Deletions to Make String Balanced
 *
 * You are given a string s consisting only of characters 'a' and 'b'​​​​.
 * You can delete any number of characters in s to make s balanced. s is balanced if there is no pair of indices (i,j) such that i < j and s[i] = 'b' and s[j]= 'a'.
 * Return the minimum number of deletions needed to make s balanced.
 *  
 * Example 1:
 *
 * Input: s = "aababbab"
 * Output: 2
 * Explanation: You can either:
 * Delete the characters at 0-indexed positions 2 and 6 ("aa<u>b</u>abb<u>a</u>b" -> "aaabbb"), or
 * Delete the characters at 0-indexed positions 3 and 6 ("aab<u>a</u>bb<u>a</u>b" -> "aabbbb").
 *
 * Example 2:
 *
 * Input: s = "bbaaaaabb"
 * Output: 2
 * Explanation: The only solution is to delete the first two characters.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is 'a' or 'b'​​.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/
// discuss: https://leetcode.com/problems/minimum-deletions-to-make-string-balanced/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        s.chars()
            .fold((0, 0), |(mut d, mut b), c| {
                if c == 'b' {
                    b += 1;
                } else {
                    d = b.min(d + 1);
                };
                (d, b)
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1653_example_1() {
        let s = "aababbab".to_string();

        let result = 2;

        assert_eq!(Solution::minimum_deletions(s), result);
    }

    #[test]
    fn test_1653_example_2() {
        let s = "bbaaaaabb".to_string();

        let result = 2;

        assert_eq!(Solution::minimum_deletions(s), result);
    }
}

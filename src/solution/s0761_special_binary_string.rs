/**
 * [0761] Special Binary String
 *
 * Special binary strings are binary strings with the following two properties:
 *
 * 	The number of 0's is equal to the number of 1's.
 * 	Every prefix of the binary string has at least as many 1's as 0's.
 *
 * You are given a special binary string s.
 * A move consists of choosing two consecutive, non-empty, special substrings of s, and swapping them. Two strings are consecutive if the last character of the first string is exactly one index before the first character of the second string.
 * Return the lexicographically largest resulting string possible after applying the mentioned operations on the string.
 *  
 * Example 1:
 *
 * Input: s = "11011000"
 * Output: "11100100"
 * Explanation: The strings "10" [occuring at s[1]] and "1100" [at s[3]] are swapped.
 * This is the lexicographically largest string possible after some number of swaps.
 *
 * Example 2:
 *
 * Input: s = "10"
 * Output: "10"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 50
 * 	s[i] is either '0' or '1'.
 * 	s is a special binary string.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/special-binary-string/
// discuss: https://leetcode.com/problems/special-binary-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        Self::dfs_helper(&s)
    }

    fn dfs_helper(s: &str) -> String {
        let (mut count, mut i) = (0, 0);
        let mut result = Vec::<String>::new();
        for j in 0..s.len() {
            if s[j..].chars().next().unwrap() == '1' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                let mut s1 = String::new();
                s1.push('1');
                s1.push_str(Self::dfs_helper(&s[i + 1..j]).as_str());
                s1.push('0');
                result.push(s1);
                i = j + 1;
            }
        }
        result.sort_by(|a, b| b.cmp(a));
        result.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&x[..]);
            acc
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0761_example_1() {
        let s = "11011000".to_string();
        let result = "11100100".to_string();

        assert_eq!(Solution::make_largest_special(s), result);
    }

    #[test]
    fn test_0761_example_2() {
        let s = "10".to_string();
        let result = "10".to_string();

        assert_eq!(Solution::make_largest_special(s), result);
    }
}

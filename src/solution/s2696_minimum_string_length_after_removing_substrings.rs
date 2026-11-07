/**
 * [2696] Minimum String Length After Removing Substrings
 *
 * You are given a string s consisting only of uppercase English letters.
 * You can apply some operations to this string where, in one operation, you can remove any occurrence of one of the substrings "AB" or "CD" from s.
 * Return the minimum possible length of the resulting string that you can obtain.
 * Note that the string concatenates after removing the substring and could produce new "AB" or "CD" substrings.
 *  
 * Example 1:
 *
 * Input: s = "ABFCACDB"
 * Output: 2
 * Explanation: We can do the following operations:
 * - Remove the substring "ABFCACDB", so s = "FCACDB".
 * - Remove the substring "FCACDB", so s = "FCAB".
 * - Remove the substring "FCAB", so s = "FC".
 * So the resulting length of the string is 2.
 * It can be shown that it is the minimum length that we can obtain.
 * Example 2:
 *
 * Input: s = "ACBBD"
 * Output: 5
 * Explanation: We cannot do any operations on the string so the length remains the same.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s consists only of uppercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-string-length-after-removing-substrings/
// discuss: https://leetcode.com/problems/minimum-string-length-after-removing-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut result = vec![];
        for c in s.bytes() {
            match (result.last(), c) {
                (Some(b'A'), b'B') | (Some(b'C'), b'D') => {
                    result.pop();
                }
                _ => result.push(c),
            }
        }
        result.len() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2696_example_1() {
        let s = "ABFCACDB".to_string();

        let result = 2;

        assert_eq!(Solution::min_length(s), result);
    }

    #[test]
    fn test_2696_example_2() {
        let s = "ACBBD".to_string();

        let result = 5;

        assert_eq!(Solution::min_length(s), result);
    }
}

/**
 * [0686] Repeated String Match
 *
 * Given two strings a and b, return the minimum number of times you should repeat string a so that string b is a substring of it. If it is impossible for b​​​​​​ to be a substring of a after repeating it, return -1.
 * Notice: string "abc" repeated 0 times is "", repeated 1 time is "abc" and repeated 2 times is "abcabc".
 *  
 * Example 1:
 *
 * Input: a = "abcd", b = "cdabcdab"
 * Output: 3
 * Explanation: We return 3 because by repeating a three times "abcdabcdabcd", b is a substring of it.
 *
 * Example 2:
 *
 * Input: a = "a", b = "aa"
 * Output: 2
 *
 *  
 * Constraints:
 *
 * 	1 <= a.length, b.length <= 10^4
 * 	a and b consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/repeated-string-match/
// discuss: https://leetcode.com/problems/repeated-string-match/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let mut c = String::new();
        for i in 0..(b.len() / a.len() + 3) {
            if c.contains(&b) {
                return i as i32;
            }
            c = format!("{}{}", c, a);
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0686_example_1() {
        let a = "abcd".to_string();
        let b = "cdabcdab".to_string();
        let result = 3;

        assert_eq!(Solution::repeated_string_match(a, b), result);
    }

    #[test]
    fn test_0686_example_2() {
        let a = "a".to_string();
        let b = "aa".to_string();
        let result = 2;

        assert_eq!(Solution::repeated_string_match(a, b), result);
    }
}

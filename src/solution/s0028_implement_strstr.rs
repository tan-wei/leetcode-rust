/**
 * [28] Implement strStr()
 *
 * Implement <a href="http://www.cplusplus.com/reference/cstring/strstr/" target="_blank">strStr()</a>.
 * Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
 * Clarification:
 * What should we return when needle is an empty string? This is a great question to ask during an interview.
 * For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's <a href="http://www.cplusplus.com/reference/cstring/strstr/" target="_blank">strstr()</a> and Java's <a href="https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#indexOf(java.lang.String)" target="_blank">indexOf()</a>.
 *  
 * Example 1:
 * Input: haystack = "hello", needle = "ll"
 * Output: 2
 * Example 2:
 * Input: haystack = "aaaaa", needle = "bba"
 * Output: -1
 * Example 3:
 * Input: haystack = "", needle = ""
 * Output: 0
 *  
 * Constraints:
 *
 * 	0 <= haystack.length, needle.length <= 5 * 10^4
 * 	haystack and needle consist of only lower-case English characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/implement-strstr/
// discuss: https://leetcode.com/problems/implement-strstr/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(n) => n as i32,
            None => -1i32,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0028_example_1() {
        let haystack = "hello".to_string();
        let needle = "ll".to_string();

        assert_eq!(Solution::str_str(haystack, needle), 2);
    }

    #[test]
    fn test_0028_example_2() {
        let haystack = "aaaaa".to_string();
        let needle = "bba".to_string();

        assert_eq!(Solution::str_str(haystack, needle), -1);
    }

    #[test]
    fn test_0028_example_3() {
        let haystack = "".to_string();
        let needle = "".to_string();

        assert_eq!(Solution::str_str(haystack, needle), 0);
    }
}

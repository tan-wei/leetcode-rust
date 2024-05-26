/**
 * [1616] Split Two Strings to Make Palindrome
 *
 * You are given two strings a and b of the same length. Choose an index and split both strings at the same index, splitting a into two strings: aprefix and asuffix where a = aprefix + asuffix, and splitting b into two strings: bprefix and bsuffix where b = bprefix + bsuffix. Check if aprefix + bsuffix or bprefix + asuffix forms a palindrome.
 * When you split a string s into sprefix and ssuffix, either ssuffix or sprefix is allowed to be empty. For example, if s = "abc", then "" + "abc", "a" + "bc", "ab" + "c" , and "abc" + "" are valid splits.
 * Return true if it is possible to form a palindrome string, otherwise return false.
 * Notice that x + y denotes the concatenation of strings x and y.
 *  
 * Example 1:
 *
 * Input: a = "x", b = "y"
 * Output: true
 * Explaination: If either a or b are palindromes the answer is true since you can split in the following way:
 * aprefix = "", asuffix = "x"
 * bprefix = "", bsuffix = "y"
 * Then, aprefix + bsuffix = "" + "y" = "y", which is a palindrome.
 *
 * Example 2:
 *
 * Input: a = "xbdef", b = "xecab"
 * Output: false
 *
 * Example 3:
 *
 * Input: a = "ulacfd", b = "jizalu"
 * Output: true
 * Explaination: Split them at index 3:
 * aprefix = "ula", asuffix = "cfd"
 * bprefix = "jiz", bsuffix = "alu"
 * Then, aprefix + bsuffix = "ula" + "alu" = "ulaalu", which is a palindrome.
 *
 *  
 * Constraints:
 *
 * 	1 <= a.length, b.length <= 10^5
 * 	a.length == b.length
 * 	a and b consist of lowercase English letters
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-two-strings-to-make-palindrome/
// discuss: https://leetcode.com/problems/split-two-strings-to-make-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let (a, b) = (a.as_bytes(), b.as_bytes());

        Self::check(a, b) || Self::check(b, a)
    }

    fn is_palindrome(s: &[u8], i: usize, j: usize) -> bool {
        let (mut i, mut j) = (i, j);
        while i < j && s[i] == s[j] {
            i += 1;
            j -= 1;
        }
        i >= j
    }

    fn check(a: &[u8], b: &[u8]) -> bool {
        let (mut i, mut j) = (0, a.len() - 1);

        while i < j && a[i] == b[j] {
            i += 1;
            j -= 1;
        }

        Self::is_palindrome(a, i, j) || Self::is_palindrome(b, i, j)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1616_example_1() {
        let a = "x".to_string();
        let b = "y".to_string();

        let result = true;

        assert_eq!(Solution::check_palindrome_formation(a, b), result);
    }

    #[test]
    fn test_1616_example_2() {
        let a = "xbdef".to_string();
        let b = "xecab".to_string();

        let result = false;

        assert_eq!(Solution::check_palindrome_formation(a, b), result);
    }

    #[test]
    fn test_1616_example_3() {
        let a = "ulacfd".to_string();
        let b = "jizalu".to_string();

        let result = true;

        assert_eq!(Solution::check_palindrome_formation(a, b), result);
    }
}

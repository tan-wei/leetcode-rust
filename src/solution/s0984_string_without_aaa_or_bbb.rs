/**
 * [0984] String Without AAA or BBB
 *
 * Given two integers a and b, return any string s such that:
 *
 * 	s has length a + b and contains exactly a 'a' letters, and exactly b 'b' letters,
 * 	The substring 'aaa' does not occur in s, and
 * 	The substring 'bbb' does not occur in s.
 *
 *  
 * Example 1:
 *
 * Input: a = 1, b = 2
 * Output: "abb"
 * Explanation: "abb", "bab" and "bba" are all correct answers.
 *
 * Example 2:
 *
 * Input: a = 4, b = 1
 * Output: "aabaa"
 *
 *  
 * Constraints:
 *
 * 	0 <= a, b <= 100
 * 	It is guaranteed such an s exists for the given a and b.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-without-aaa-or-bbb/
// discuss: https://leetcode.com/problems/string-without-aaa-or-bbb/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let mut result = String::new();

        let (mut a, mut b) = (a, b);

        while a != 0 && b != 0 {
            if a > b {
                result += "aab";
                a -= 1;
            } else if b > a {
                result += "bba";
                b -= 1
            } else {
                result += "ab";
            }
            a -= 1;
            b -= 1;
        }
        while a != 0 {
            result += "a";
            a -= 1;
        }
        while b != 0 {
            result += "b";
            b -= 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0984_example_1() {
        let a = 1;
        let b = 2;
        let result = "abb".to_string();

        assert_eq!(Solution::str_without3a3b(a, b), result);
    }

    #[test]
    #[ignore]
    fn test_0984_example_2() {
        let a = 4;
        let b = 1;
        let result = "aabaa".to_string();

        assert_eq!(Solution::str_without3a3b(a, b), result);
    }
}

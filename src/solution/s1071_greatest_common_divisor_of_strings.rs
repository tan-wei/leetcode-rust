/**
 * [1071] Greatest Common Divisor of Strings
 *
 * For two strings s and t, we say "t divides s" if and only if s = t + ... + t (i.e., t is concatenated with itself one or more times).
 * Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.
 *  
 * Example 1:
 *
 * Input: str1 = "ABCABC", str2 = "ABC"
 * Output: "ABC"
 *
 * Example 2:
 *
 * Input: str1 = "ABABAB", str2 = "ABAB"
 * Output: "AB"
 *
 * Example 3:
 *
 * Input: str1 = "LEET", str2 = "CODE"
 * Output: ""
 *
 *  
 * Constraints:
 *
 * 	1 <= str1.length, str2.length <= 1000
 * 	str1 and str2 consist of English uppercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/greatest-common-divisor-of-strings/
// discuss: https://leetcode.com/problems/greatest-common-divisor-of-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let n = str1.len();
        let m = str2.len();

        let mut i = match n < m {
            true => n,
            false => m,
        };

        while i > 0 {
            let x = match n < m {
                true => str1.get(..i).unwrap(),
                false => str2.get(..i).unwrap(),
            };

            if str1.split(x).all(|x| x.len() == 0) && str2.split(x).all(|x| x.len() == 0) {
                return String::from(x);
            }

            i = i - 1;
        }

        "".to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1071_example_1() {
        let str1 = "ABCABC".to_string();
        let str2 = "ABC".to_string();
        let result = "ABC".to_string();

        assert_eq!(Solution::gcd_of_strings(str1, str2), result);
    }

    #[test]
    fn test_1071_example_2() {
        let str1 = "ABABAB".to_string();
        let str2 = "ABAB".to_string();
        let result = "AB".to_string();

        assert_eq!(Solution::gcd_of_strings(str1, str2), result);
    }

    #[test]
    fn test_1071_example_3() {
        let str1 = "LEET".to_string();
        let str2 = "CODE".to_string();
        let result = "".to_string();

        assert_eq!(Solution::gcd_of_strings(str1, str2), result);
    }
}

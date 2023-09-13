/**
 * [1309] Decrypt String from Alphabet to Integer Mapping
 *
 * You are given a string s formed by digits and '#'. We want to map s to English lowercase characters as follows:
 *
 * 	Characters ('a' to 'i') are represented by ('1' to '9') respectively.
 * 	Characters ('j' to 'z') are represented by ('10#' to '26#') respectively.
 *
 * Return the string formed after mapping.
 * The test cases are generated so that a unique mapping will always exist.
 *
 * Example 1:
 *
 * Input: s = "10#11#12"
 * Output: "jkab"
 * Explanation: "j" -> "10#" , "k" -> "11#" , "a" -> "1" , "b" -> "2".
 *
 * Example 2:
 *
 * Input: s = "1326#"
 * Output: "acz"
 *
 *
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of digits and the '#' letter.
 * 	s will be a valid string such that mapping is always possible.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping/
// discuss: https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut iter = s.chars().rev().peekable();
        let mut result = String::new();
        loop {
            match iter.next() {
                Some('#') => {
                    let c1 = iter.next().unwrap().to_digit(10).unwrap() as u8;
                    let c2 = iter.next().unwrap().to_digit(10).unwrap() as u8;
                    result.push((c2 * 10 + c1 - 1 + 'a' as u8) as char)
                }
                Some(x) => {
                    let c = x.to_digit(10).unwrap() as u8;
                    result.push((c - 1 + 'a' as u8) as char);
                }
                _ => {}
            }
            if iter.peek() == None {
                break;
            }
        }
        result.chars().rev().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1309_example_1() {
        let s = "10#11#12".to_string();
        let result = "jkab".to_string();

        assert_eq!(Solution::freq_alphabets(s), result);
    }

    #[test]
    fn test_1309_example_2() {
        let s = "1326#".to_string();
        let result = "acz".to_string();

        assert_eq!(Solution::freq_alphabets(s), result);
    }
}

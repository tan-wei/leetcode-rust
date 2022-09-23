/**
 * [0831] Masking Personal Information
 *
 * You are given a personal information string s, representing either an email address or a phone number. Return the masked personal information using the below rules.
 * <u>Email address:</u>
 * An email address is:
 *
 * 	A name consisting of uppercase and lowercase English letters, followed by
 * 	The '@' symbol, followed by
 * 	The domain consisting of uppercase and lowercase English letters with a dot '.' somewhere in the middle (not the first or last character).
 *
 * To mask an email:
 *
 * 	The uppercase letters in the name and domain must be converted to lowercase letters.
 * 	The middle letters of the name (i.e., all but the first and last letters) must be replaced by 5 asterisks "*****".
 *
 * <u>Phone number:</u>
 * A phone number is formatted as follows:
 *
 * 	The phone number contains 10-13 digits.
 * 	The last 10 digits make up the local number.
 * 	The remaining 0-3 digits, in the beginning, make up the country code.
 * 	Separation characters from the set {'+', '-', '(', ')', ' '} separate the above digits in some way.
 *
 * To mask a phone number:
 *
 * 	Remove all separation characters.
 * 	The masked phone number should have the form:
 *
 * 		"***-***-XXXX" if the country code has 0 digits.
 * 		"+*-***-***-XXXX" if the country code has 1 digit.
 * 		"+**-***-***-XXXX" if the country code has 2 digits.
 * 		"+***-***-***-XXXX" if the country code has 3 digits.
 *
 *
 * 	"XXXX" is the last 4 digits of the local number.
 *
 *  
 * Example 1:
 *
 * Input: s = "LeetCode@LeetCode.com"
 * Output: "l*****e@leetcode.com"
 * Explanation: s is an email address.
 * The name and domain are converted to lowercase, and the middle of the name is replaced by 5 asterisks.
 *
 * Example 2:
 *
 * Input: s = "AB@qq.com"
 * Output: "a*****b@qq.com"
 * Explanation: s is an email address.
 * The name and domain are converted to lowercase, and the middle of the name is replaced by 5 asterisks.
 * Note that even though "ab" is 2 characters, it still must have 5 asterisks in the middle.
 *
 * Example 3:
 *
 * Input: s = "1(234)567-890"
 * Output: "***-***-7890"
 * Explanation: s is a phone number.
 * There are 10 digits, so the local number is 10 digits and the country code is 0 digits.
 * Thus, the resulting masked number is "***-***-7890".
 *
 *  
 * Constraints:
 *
 * 	s is either a valid email or a phone number.
 * 	If s is an email:
 *
 * 		8 <= s.length <= 40
 * 		s consists of uppercase and lowercase English letters and exactly one '@' symbol and '.' symbol.
 *
 *
 * 	If s is a phone number:
 *
 * 		10 <= s.length <= 20
 * 		s consists of digits, spaces, and the symbols '(', ')', '-', and '+'.
 *
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/masking-personal-information/
// discuss: https://leetcode.com/problems/masking-personal-information/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(PartialEq)]
enum Information {
    Email,
    PhoneNumber,
}

impl Solution {
    pub fn mask_pii(s: String) -> String {
        if Self::determine_information(&s) == Information::Email {
            Self::mask_email(&s)
        } else {
            Self::mask_phone_number(&s)
        }
    }

    fn determine_information(s: &str) -> Information {
        if s.contains('@') {
            Information::Email
        } else {
            Information::PhoneNumber
        }
    }

    fn mask_email(s: &str) -> String {
        let parts: Vec<&str> = s.split('@').collect();
        let name1 = parts[0];

        let parts: Vec<&str> = parts[1].split('.').collect();
        let name2 = parts[0];
        let name3 = parts[1];

        format!(
            "{}*****{}@{}.{}",
            name1.chars().next().unwrap().to_lowercase(),
            name1.chars().rev().next().unwrap().to_lowercase(),
            name2.to_lowercase(),
            name3.to_lowercase()
        )
    }

    fn mask_phone_number(s: &str) -> String {
        let digits: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
        if digits.len() == 10 {
            format!("***-***-{}", digits.into_iter().skip(6).collect::<String>())
        } else {
            format!(
                "+{}-***-***-{}",
                "*".repeat(digits.len() - 10),
                digits.iter().skip(digits.len() - 4).collect::<String>()
            )
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0831_example_1() {
        let s = "LeetCode@LeetCode.com".to_string();
        let result = "l*****e@leetcode.com".to_string();

        assert_eq!(Solution::mask_pii(s), result);
    }

    #[test]
    fn test_0831_example_2() {
        let s = "AB@qq.com".to_string();
        let result = "a*****b@qq.com".to_string();

        assert_eq!(Solution::mask_pii(s), result);
    }

    #[test]
    fn test_0831_example_3() {
        let s = "1(234)567-890".to_string();
        let result = "***-***-7890".to_string();

        assert_eq!(Solution::mask_pii(s), result);
    }
}

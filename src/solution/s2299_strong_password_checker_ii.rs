/**
 * [2299] Strong Password Checker II
 *
 * A password is said to be strong if it satisfies all the following criteria:
 *
 * 	It has at least 8 characters.
 * 	It contains at least one lowercase letter.
 * 	It contains at least one uppercase letter.
 * 	It contains at least one digit.
 * 	It contains at least one special character. The special characters are the characters in the following string: "!@#$%^&amp;*()-+".
 * 	It does not contain 2 of the same character in adjacent positions (i.e., "aab" violates this condition, but "aba" does not).
 *
 * Given a string password, return true if it is a strong password. Otherwise, return false.
 *  
 * Example 1:
 *
 * Input: password = "IloveLe3tcode!"
 * Output: true
 * Explanation: The password meets all the requirements. Therefore, we return true.
 *
 * Example 2:
 *
 * Input: password = "Me+You--IsMyDream"
 * Output: false
 * Explanation: The password does not contain a digit and also contains 2 of the same character in adjacent positions. Therefore, we return false.
 *
 * Example 3:
 *
 * Input: password = "1aB!"
 * Output: false
 * Explanation: The password does not meet the length requirement. Therefore, we return false.
 *  
 * Constraints:
 *
 * 	1 <= password.length <= 100
 * 	password consists of letters, digits, and special characters: "!@#$%^&amp;*()-+".
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/strong-password-checker-ii/
// discuss: https://leetcode.com/problems/strong-password-checker-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        if !password.chars().any(|c| c.is_ascii_lowercase()) {
            return false;
        }

        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            return false;
        }

        if !password.chars().any(|c| c.is_ascii_digit()) {
            return false;
        }

        if !password.chars().any(|c| "!@#$%^&*()-+".contains(c)) {
            return false;
        }

        password
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] != w[1])
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2299_example_1() {
        let password = "IloveLe3tcode!".to_string();

        let result = true;

        assert_eq!(Solution::strong_password_checker_ii(password), result);
    }

    #[test]
    fn test_2299_example_2() {
        let password = "Me+You--IsMyDream".to_string();

        let result = false;

        assert_eq!(Solution::strong_password_checker_ii(password), result);
    }

    #[test]
    fn test_2299_example_3() {
        let password = "1aB!".to_string();

        let result = false;

        assert_eq!(Solution::strong_password_checker_ii(password), result);
    }
}

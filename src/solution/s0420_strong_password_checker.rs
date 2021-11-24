/**
 * [0420] Strong Password Checker
 *
 * A password is considered strong if the below conditions are all met:
 *
 * 	It has at least 6 characters and at most 20 characters.
 * 	It contains at least one lowercase letter, at least one uppercase letter, and at least one digit.
 * 	It does not contain three repeating characters in a row (i.e., "...aaa..." is weak, but "...aa...a..." is strong, assuming other conditions are met).
 *
 * Given a string password, return the minimum number of steps required to make password strong. if password is already strong, return 0.
 * In one step, you can:
 *
 * 	Insert one character to password,
 * 	Delete one character from password, or
 * 	Replace one character of password with another character.
 *
 *  
 * Example 1:
 * Input: password = "a"
 * Output: 5
 * Example 2:
 * Input: password = "aA1"
 * Output: 3
 * Example 3:
 * Input: password = "1337C0d3"
 * Output: 0
 *  
 * Constraints:
 *
 * 	1 <= password.length <= 50
 * 	password consists of letters, digits, dot '.' or exclamation mark '!'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/strong-password-checker/
// discuss: https://leetcode.com/problems/strong-password-checker/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/strong-password-checker/discuss/1482857/Rust%3A-0-ms-faster-than-100.00or2-MB-less-than-100.00
    pub fn strong_password_checker(password: String) -> i32 {
        let new_length = password.len() as i32;
        if new_length > 20 {
            let rep = 3 - Self::compu_type_count(&password);
            let del = new_length - 20;
            let vec_count = Self::repeat_count(&password);
            rep + del + Self::replace_count(vec_count, del, 0, rep, true)
        } else if new_length < 6 {
            let add = 6 - new_length;
            let type_add = 3 - Self::compu_type_count(&password);
            let rep = if type_add <= add { 0 } else { type_add - add };
            add + rep + Self::replace_count(Self::repeat_count(&password), 0, add, rep, true)
        } else {
            let rep = 3 - Self::compu_type_count(&password);
            let vec_count = Self::repeat_count(&password);
            rep + Self::replace_count(vec_count, 0, 0, rep, true)
        }
    }

    pub fn repeat_count(password: &String) -> Vec<i32> {
        let mut result = Vec::new();
        let mut currect = 1;
        for i in 0..password.len() - 1 {
            if password[i..i + 1] == password[i + 1..i + 2] {
                currect += 1;
            } else {
                if currect >= 3 {
                    result.push(currect);
                }
                currect = 1;
            }
        }
        if currect >= 3 {
            result.push(currect);
        }
        result
    }

    pub fn replace_count(
        mut vec_count: Vec<i32>,
        mut del: i32,
        add: i32,
        rep: i32,
        sort_switch: bool,
    ) -> i32 {
        if vec_count.len() == 0 {
            return 0;
        }
        if sort_switch {
            vec_count.sort();
        }
        if add >= 2 || (add == 1 && rep == 1) || (add == 1 && rep == 0 && vec_count[0] < 5) {
            return 0;
        }
        if add == 1 && rep == 0 && vec_count[0] == 5 {
            return 1;
        }
        if del == 0 {
            let mut result_count = 0;
            for i in vec_count {
                result_count += i / 3;
            }
            if result_count >= rep {
                return result_count - rep;
            }
            return 0;
        }

        'loop1: for currcet_index in 1..=3 {
            while del >= currcet_index && vec_count[0] == 2 + currcet_index {
                del -= currcet_index;
                vec_count.remove(0);
                if vec_count.len() == 0 {
                    return 0;
                }
            }
            if del < currcet_index {
                break;
            }
            for i in 0..vec_count.len() {
                while vec_count[i] % 3 == (currcet_index - 1) {
                    if vec_count[i] == 2 + currcet_index {
                        break;
                    }
                    vec_count[i] -= currcet_index;
                    del -= currcet_index;
                    if del < currcet_index {
                        break 'loop1;
                    }
                }
            }
        }
        let mut result_count = 0;
        for i in vec_count {
            result_count += i / 3;
        }
        if result_count >= rep {
            return result_count - rep;
        }
        return 0;
    }

    pub fn compu_type_count(password: &String) -> i32 {
        let mut lower_case = 0;
        let mut upper_case = 0;
        let mut digit = 0;
        let mut total = 0;
        for i in password.as_bytes() {
            if *i >= 48 && *i <= 57 {
                if digit == 0 {
                    digit = 1;
                    total += 1;
                    if total == 3 {
                        return 3;
                    }
                    continue;
                }
            }
            if *i >= 65 && *i <= 90 {
                if upper_case == 0 {
                    upper_case = 1;
                    total += 1;
                    if total == 3 {
                        return 3;
                    }
                    continue;
                }
            }
            if *i >= 97 && *i <= 122 {
                if lower_case == 0 {
                    lower_case = 1;
                    total += 1;
                    if total == 3 {
                        return 3;
                    }
                    continue;
                }
            }
        }
        return total;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0420_example_1() {
        let password = "a".to_string();
        let result = 5;

        assert_eq!(Solution::strong_password_checker(password), result);
    }

    #[test]
    fn test_0420_example_2() {
        let password = "aA1".to_string();
        let result = 3;

        assert_eq!(Solution::strong_password_checker(password), result);
    }

    #[test]
    fn test_0420_example_3() {
        let password = "1337C0d3".to_string();
        let result = 0;

        assert_eq!(Solution::strong_password_checker(password), result);
    }
}

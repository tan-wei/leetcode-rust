/**
 * [12] Integer to Roman
 *
 * Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
 *
 * Symbol       Value
 * I             1
 * V             5
 * X             10
 * L             50
 * C             100
 * D             500
 * M             1000
 * For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
 * Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
 *
 * 	I can be placed before V (5) and X (10) to make 4 and 9.
 * 	X can be placed before L (50) and C (100) to make 40 and 90.
 * 	C can be placed before D (500) and M (1000) to make 400 and 900.
 *
 * Given an integer, convert it to a roman numeral.
 *  
 * Example 1:
 *
 * Input: num = 3
 * Output: "III"
 *
 * Example 2:
 *
 * Input: num = 4
 * Output: "IV"
 *
 * Example 3:
 *
 * Input: num = 9
 * Output: "IX"
 *
 * Example 4:
 *
 * Input: num = 58
 * Output: "LVIII"
 * Explanation: L = 50, V = 5, III = 3.
 *
 * Example 5:
 *
 * Input: num = 1994
 * Output: "MCMXCIV"
 * Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 3999
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-to-roman/
// discuss: https://leetcode.com/problems/integer-to-roman/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut result = String::new();
        let mut num = num;
        while num > 0 {
            if num >= 1000 {
                result.push_str("M");
                num -= 1000;
            } else if num >= 900 {
                result.push_str("CM");
                num -= 900;
            } else if num >= 500 {
                result.push_str("D");
                num -= 500;
            } else if num >= 400 {
                result.push_str("CD");
                num -= 400;
            } else if num >= 100 {
                result.push_str("C");
                num -= 100;
            } else if num >= 90 {
                result.push_str("XC");
                num -= 90;
            } else if num >= 50 {
                result.push_str("L");
                num -= 50;
            } else if num >= 40 {
                result.push_str("XL");
                num -= 40;
            } else if num >= 10 {
                result.push_str("X");
                num -= 10;
            } else if num >= 9 {
                result.push_str("IX");
                num -= 9;
            } else if num >= 5 {
                result.push_str("V");
                num -= 5;
            } else if num >= 4 {
                result.push_str("IV");
                num -= 4;
            } else if num >= 1 {
                result.push_str("I");
                num -= 1;
            } else {
            }
        }
        result
    }

    pub fn int_to_roman_v2(num: i32) -> String {
        let m = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let s = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let (mut num, mut buf) = (num, vec![]);
        for i in 0..13 {
            let mut j = num / m[i];
            num %= m[i];
            while j > 0 {
                buf.push(s[i]);
                j -= 1;
            }
        }
        buf.into_iter().collect()
    }

    pub fn int_to_roman_v3(num: i32) -> String {
        let m = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let s = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut remainder = num;
        m.iter()
            .zip(s.iter())
            .fold(String::new(), |mut acc, (divisor, roman)| {
                let quotient = remainder / divisor;
                if quotient > 0 {
                    remainder %= divisor * quotient;
                    acc.push_str(&roman.repeat(quotient as usize));
                }
                acc
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0012_example_1() {
        assert_eq!(Solution::int_to_roman(3), String::from("III"));
        assert_eq!(Solution::int_to_roman_v2(3), String::from("III"));
        assert_eq!(Solution::int_to_roman_v3(3), String::from("III"));
    }

    #[test]
    fn test_0012_example_2() {
        assert_eq!(Solution::int_to_roman(4), String::from("IV"));
        assert_eq!(Solution::int_to_roman_v2(4), String::from("IV"));
        assert_eq!(Solution::int_to_roman_v3(4), String::from("IV"));
    }

    #[test]
    fn test_0012_example_3() {
        assert_eq!(Solution::int_to_roman(9), String::from("IX"));
        assert_eq!(Solution::int_to_roman_v2(9), String::from("IX"));
        assert_eq!(Solution::int_to_roman_v3(9), String::from("IX"));
    }

    #[test]
    fn test_0012_example_4() {
        assert_eq!(Solution::int_to_roman(58), String::from("LVIII"));
        assert_eq!(Solution::int_to_roman_v2(58), String::from("LVIII"));
        assert_eq!(Solution::int_to_roman_v3(58), String::from("LVIII"));
    }

    #[test]
    fn test_0012_example_5() {
        assert_eq!(Solution::int_to_roman(1994), String::from("MCMXCIV"));
        assert_eq!(Solution::int_to_roman_v2(1994), String::from("MCMXCIV"));
        assert_eq!(Solution::int_to_roman_v3(1994), String::from("MCMXCIV"));
    }
}

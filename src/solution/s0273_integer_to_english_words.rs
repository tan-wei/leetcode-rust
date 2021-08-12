/**
 * [273] Integer to English Words
 *
 * Convert a non-negative integer num to its English words representation.
 *  
 * Example 1:
 * Input: num = 123
 * Output: "One Hundred Twenty Three"
 * Example 2:
 * Input: num = 12345
 * Output: "Twelve Thousand Three Hundred Forty Five"
 * Example 3:
 * Input: num = 1234567
 * Output: "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
 * Example 4:
 * Input: num = 1234567891
 * Output: "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
 *  
 * Constraints:
 *
 * 	0 <= num <= 2^31 - 1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/integer-to-english-words/
// discuss: https://leetcode.com/problems/integer-to-english-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/integer-to-english-words/discuss/397108/Rust-Solution-(Just-pattern-matching-and-recursion!)
    pub fn number_to_words(num: i32) -> String {
        match num {
            0 => "Zero".into(),
            1 => "One".into(),
            2 => "Two".into(),
            3 => "Three".into(),
            4 => "Four".into(),
            5 => "Five".into(),
            6 => "Six".into(),
            7 => "Seven".into(),
            8 => "Eight".into(),
            9 => "Nine".into(),
            10 => "Ten".into(),
            11 => "Eleven".into(),
            12 => "Twelve".into(),
            13 => "Thirteen".into(),
            14 => "Fourteen".into(),
            15 => "Fifteen".into(),
            16 => "Sixteen".into(),
            17 => "Seventeen".into(),
            18 => "Eighteen".into(),
            19 => "Nineteen".into(),
            20 => "Twenty".into(),
            30 => "Thirty".into(),
            40 => "Forty".into(),
            50 => "Fifty".into(),
            60 => "Sixty".into(),
            70 => "Seventy".into(),
            80 => "Eighty".into(),
            90 => "Ninety".into(),
            100 => "One Hundred".into(),
            x @ 20..=99 => {
                format!(
                    "{} {}",
                    Self::number_to_words((x / 10) * 10),
                    Self::number_to_words(x % 10)
                )
            }
            x @ 100..=999 => Self::magnitude("Hundred", x, 100),
            x @ 1000..=999999 => Self::magnitude("Thousand", x, 1000),
            x @ 1000000..=999999999 => Self::magnitude("Million", x, 1000000),
            x @ 1000000000..=std::i32::MAX => Self::magnitude("Billion", x, 1000000000),
            _ => unimplemented!(),
        }
    }

    fn magnitude(name: &str, number: i32, magnitude: i32) -> String {
        let remainder = number % magnitude;
        if remainder == 0 {
            format!("{} {}", Self::number_to_words(number / magnitude), name)
        } else {
            format!(
                "{} {} {}",
                Self::number_to_words(number / magnitude),
                name,
                Self::number_to_words(remainder)
            )
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0273_example_1() {
        let num = 123;
        let result = "One Hundred Twenty Three".to_string();

        assert_eq!(Solution::number_to_words(num), result);
    }

    #[test]
    fn test_0273_example_2() {
        let num = 12345;
        let result = "Twelve Thousand Three Hundred Forty Five".to_string();

        assert_eq!(Solution::number_to_words(num), result);
    }

    #[test]
    fn test_0273_example_3() {
        let num = 1234567;
        let result =
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string();

        assert_eq!(Solution::number_to_words(num), result);
    }

    #[test]
    fn test_0273_example_4() {
        let num = 1234567891;
        let result = "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One".to_string();

        assert_eq!(Solution::number_to_words(num), result);
    }
}

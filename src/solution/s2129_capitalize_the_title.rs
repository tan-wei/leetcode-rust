/**
 * [2129] Capitalize the Title
 *
 * You are given a string title consisting of one or more words separated by a single space, where each word consists of English letters. Capitalize the string by changing the capitalization of each word such that:
 *
 * 	If the length of the word is 1 or 2 letters, change all letters to lowercase.
 * 	Otherwise, change the first letter to uppercase and the remaining letters to lowercase.
 *
 * Return the capitalized title.
 *  
 * Example 1:
 *
 * Input: title = "capiTalIze tHe titLe"
 * Output: "Capitalize The Title"
 * Explanation:
 * Since all the words have a length of at least 3, the first letter of each word is uppercase, and the remaining letters are lowercase.
 *
 * Example 2:
 *
 * Input: title = "First leTTeR of EACH Word"
 * Output: "First Letter of Each Word"
 * Explanation:
 * The word "of" has length 2, so it is all lowercase.
 * The remaining words have a length of at least 3, so the first letter of each remaining word is uppercase, and the remaining letters are lowercase.
 *
 * Example 3:
 *
 * Input: title = "i lOve leetcode"
 * Output: "i Love Leetcode"
 * Explanation:
 * The word "i" has length 1, so it is lowercase.
 * The remaining words have a length of at least 3, so the first letter of each remaining word is uppercase, and the remaining letters are lowercase.
 *
 *  
 * Constraints:
 *
 * 	1 <= title.length <= 100
 * 	title consists of words separated by a single space without any leading or trailing spaces.
 * 	Each word consists of uppercase and lowercase English letters and is non-empty.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/capitalize-the-title/
// discuss: https://leetcode.com/problems/capitalize-the-title/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn capitalize_title(title: String) -> String {
        title
            .split(" ")
            .map(|s| {
                s.chars()
                    .zip(0..)
                    .map(|(c, i)| match i == 0 && s.len() > 2 {
                        true => c.to_ascii_uppercase(),
                        false => c.to_ascii_lowercase(),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2129_example_1() {
        let title = "capiTalIze tHe titLe".to_string();
        let result = "Capitalize The Title".to_string();

        assert_eq!(Solution::capitalize_title(title), result);
    }

    #[test]
    fn test_2129_example_2() {
        let title = "First leTTeR of EACH Word".to_string();
        let result = "First Letter of Each Word".to_string();

        assert_eq!(Solution::capitalize_title(title), result);
    }

    #[test]
    fn test_2129_example_3() {
        let title = "i lOve leetcode".to_string();
        let result = "i Love Leetcode".to_string();

        assert_eq!(Solution::capitalize_title(title), result);
    }
}

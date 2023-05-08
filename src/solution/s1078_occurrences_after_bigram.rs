/**
 * [1078] Occurrences After Bigram
 *
 * Given two strings first and second, consider occurrences in some text of the form "first second third", where second comes immediately after first, and third comes immediately after second.
 * Return an array of all the words third for each occurrence of "first second third".
 *  
 * Example 1:
 * Input: text = "alice is a good girl she is a good student", first = "a", second = "good"
 * Output: ["girl","student"]
 * Example 2:
 * Input: text = "we will we will rock you", first = "we", second = "will"
 * Output: ["we","rock"]
 *  
 * Constraints:
 *
 * 	1 <= text.length <= 1000
 * 	text consists of lowercase English letters and spaces.
 * 	All the words in text a separated by a single space.
 * 	1 <= first.length, second.length <= 10
 * 	first and second consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/occurrences-after-bigram/
// discuss: https://leetcode.com/problems/occurrences-after-bigram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        text.split_whitespace()
            .scan(("", ""), |(q, r), word| {
                let p = *q;
                *q = r;
                *r = word;
                Some(if p == &first && q == &second {
                    Some(word.to_string())
                } else {
                    None
                })
            })
            .filter_map(|w| w)
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1078_example_1() {
        let text = "alice is a good girl she is a good student".to_string();
        let first = "a".to_string();
        let second = "good".to_string();
        let result = vec_string!["girl", "student"];

        assert_eq!(Solution::find_ocurrences(text, first, second), result);
    }

    #[test]
    fn test_1078_example_2() {
        let text = "we will we will rock you".to_string();
        let first = "we".to_string();
        let second = "will".to_string();
        let result = vec_string!["we", "rock"];

        assert_eq!(Solution::find_ocurrences(text, first, second), result);
    }
}

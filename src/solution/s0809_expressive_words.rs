/**
 * [0809] Expressive Words
 *
 * Sometimes people repeat letters to represent extra feeling. For example:
 *
 * 	"hello" -> "heeellooo"
 * 	"hi" -> "hiiii"
 *
 * In these strings like "heeellooo", we have groups of adjacent letters that are all the same: "h", "eee", "ll", "ooo".
 * You are given a string s and an array of query strings words. A query word is stretchy if it can be made to be equal to s by any number of applications of the following extension operation: choose a group consisting of characters c, and add some number of characters c to the group so that the size of the group is three or more.
 *
 * 	For example, starting with "hello", we could do an extension on the group "o" to get "hellooo", but we cannot get "helloo" since the group "oo" has a size less than three. Also, we could do another extension like "ll" -> "lllll" to get "helllllooo". If s = "helllllooo", then the query word "hello" would be stretchy because of these two extension operations: query = "hello" -> "hellooo" -> "helllllooo" = s.
 *
 * Return the number of query strings that are stretchy.
 *  
 * Example 1:
 *
 * Input: s = "heeellooo", words = ["hello", "hi", "helo"]
 * Output: 1
 * Explanation:
 * We can extend "e" and "o" in the word "hello" to get "heeellooo".
 * We can't extend "helo" to get "heeellooo" because the group "ll" is not size 3 or more.
 *
 * Example 2:
 *
 * Input: s = "zzzzzyyyyy", words = ["zzyy","zy","zyy"]
 * Output: 3
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length, words.length <= 100
 * 	1 <= words[i].length <= 100
 * 	s and words[i] consist of lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/expressive-words/
// discuss: https://leetcode.com/problems/expressive-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/expressive-words/discuss/1577531/C%2B%2B-and-RUST-two-start-pointers-for-two-strings
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let mut result = 0;
        let target_length = s.len();
        for word in words {
            if word.len() > target_length {
                continue;
            }
            let mut s_bytes = s.as_bytes().iter();
            let mut word_bytes = word.as_bytes().iter();
            let mut current1 = s_bytes.next();
            let mut current2 = word_bytes.next();
            let matched = loop {
                match (current1, current2) {
                    (None, Some(_)) | (Some(_), None) => break false,
                    (None, None) => break true,
                    (Some(_), Some(_)) => {
                        if current1 != current2 {
                            break false;
                        }
                        let mut count1 = 0;
                        let mut count2 = 0;
                        loop {
                            let next = s_bytes.next();
                            if current1 == next {
                                count1 += 1
                            } else {
                                current1 = next;
                                break;
                            }
                        }
                        loop {
                            let next = word_bytes.next();
                            if current2 == next {
                                count2 += 1
                            } else {
                                current2 = next;
                                break;
                            }
                        }
                        if count2 > count1 || (count1 > count2 && count1 < 2) {
                            break false;
                        }
                    }
                }
            };

            if matched {
                result += 1
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0809_example_1() {
        let s = "heeellooo".to_string();
        let words = vec_string!["hello", "hi", "helo"];
        let result = 1;

        assert_eq!(Solution::expressive_words(s, words), result);
    }

    #[test]
    fn test_0809_example_2() {
        let s = "zzzzzyyyyy".to_string();
        let words = vec_string!["zzyy", "zy", "zyy"];
        let result = 3;

        assert_eq!(Solution::expressive_words(s, words), result);
    }
}

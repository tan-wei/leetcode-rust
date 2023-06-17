/**
 * [1160] Find Words That Can Be Formed by Characters
 *
 * You are given an array of strings words and a string chars.
 * A string is good if it can be formed by characters from chars (each character can only be used once).
 * Return the sum of lengths of all good strings in words.
 *  
 * Example 1:
 *
 * Input: words = ["cat","bt","hat","tree"], chars = "atach"
 * Output: 6
 * Explanation: The strings that can be formed are "cat" and "hat" so the answer is 3 + 3 = 6.
 *
 * Example 2:
 *
 * Input: words = ["hello","world","leetcode"], chars = "welldonehoneyr"
 * Output: 10
 * Explanation: The strings that can be formed are "hello" and "world" so the answer is 5 + 5 = 10.
 *
 *  
 * Constraints:
 *
 * 	1 <= words.length <= 1000
 * 	1 <= words[i].length, chars.length <= 100
 * 	words[i] and chars consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
// discuss: https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut m: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        let mut count: i32 = 0;
        for i in chars.chars() {
            *m.entry(i).or_insert(0) += 1;
        }
        for i in 0..words.len() {
            let mut temp: std::collections::HashMap<char, i32> = m.clone();
            for (j, val) in words[i].chars().enumerate() {
                if temp.contains_key(&val) {
                    if temp.get(&val).unwrap() > &0 {
                        *temp.entry(val).or_insert(0) -= 1;
                    } else {
                        break;
                    }
                    if j == words[i].len() - 1 {
                        count += words[i].len() as i32;
                    }
                } else {
                    break;
                }
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1160_example_1() {
        let words = vec_string!["cat", "bt", "hat", "tree"];
        let chars = "atach".to_string();
        let result = 6;

        assert_eq!(Solution::count_characters(words, chars), result);
    }

    #[test]
    fn test_1160_example_2() {
        let words = vec_string!["hello", "world", "leetcode"];
        let chars = "welldonehoneyr".to_string();
        let result = 10;

        assert_eq!(Solution::count_characters(words, chars), result);
    }
}

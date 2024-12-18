/**
 * [1807] Evaluate the Bracket Pairs of a String
 *
 * You are given a string s that contains some bracket pairs, with each pair containing a non-empty key.
 *
 * 	For example, in the string "(name)is(age)yearsold", there are two bracket pairs that contain the keys "name" and "age".
 *
 * You know the values of a wide range of keys. This is represented by a 2D string array knowledge where each knowledge[i] = [keyi, valuei] indicates that key keyi has a value of valuei.
 * You are tasked to evaluate all of the bracket pairs. When you evaluate a bracket pair that contains some key keyi, you will:
 *
 * 	Replace keyi and the bracket pair with the key's corresponding valuei.
 * 	If you do not know the value of the key, you will replace keyi and the bracket pair with a question mark "?" (without the quotation marks).
 *
 * Each key will appear at most once in your knowledge. There will not be any nested brackets in s.
 * Return the resulting string after evaluating all of the bracket pairs.
 *  
 * Example 1:
 *
 * Input: s = "(name)is(age)yearsold", knowledge = [["name","bob"],["age","two"]]
 * Output: "bobistwoyearsold"
 * Explanation:
 * The key "name" has a value of "bob", so replace "(name)" with "bob".
 * The key "age" has a value of "two", so replace "(age)" with "two".
 *
 * Example 2:
 *
 * Input: s = "hi(name)", knowledge = [["a","b"]]
 * Output: "hi?"
 * Explanation: As you do not know the value of the key "name", replace "(name)" with "?".
 *
 * Example 3:
 *
 * Input: s = "(a)(a)(a)aaa", knowledge = [["a","yes"]]
 * Output: "yesyesyesaaa"
 * Explanation: The same key can appear multiple times.
 * The key "a" has a value of "yes", so replace all occurrences of "(a)" with "yes".
 * Notice that the "a"s not in a bracket pair are not evaluated.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	0 <= knowledge.length <= 10^5
 * 	knowledge[i].length == 2
 * 	1 <= keyi.length, valuei.length <= 10
 * 	s consists of lowercase English letters and round brackets '(' and ')'.
 * 	Every open bracket '(' in s will have a corresponding close bracket ')'.
 * 	The key in each bracket pair of s will be non-empty.
 * 	There will not be any nested bracket pairs in s.
 * 	keyi and valuei consist of lowercase English letters.
 * 	Each keyi in knowledge is unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/evaluate-the-bracket-pairs-of-a-string/
// discuss: https://leetcode.com/problems/evaluate-the-bracket-pairs-of-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let knowledge: std::collections::HashMap<String, String> = knowledge
            .into_iter()
            .map(|mut a| (a.swap_remove(0), a.swap_remove(0)))
            .collect();

        let mut result = String::new();

        let mut iter = s.split(&['(', ')']);

        while let Some(word) = iter.next() {
            result.push_str(word);

            if let Some(word) = iter.next() {
                // get value from hashmap, and if it does not exist, append "?"
                result.push_str(&knowledge.get(word).unwrap_or(&String::from("?")));
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
    fn test_1807_example_1() {
        let s = "(name)is(age)yearsold".to_string();
        let knowledge = vec![vec_string!["name", "bob"], vec_string!["age", "two"]];

        let result = "bobistwoyearsold".to_string();

        assert_eq!(Solution::evaluate(s, knowledge), result);
    }

    #[test]
    fn test_1807_example_2() {
        let s = "hi(name)".to_string();
        let knowledge = vec![vec_string!["a", "b"]];

        let result = "hi?".to_string();

        assert_eq!(Solution::evaluate(s, knowledge), result);
    }

    fn test_1807_example_3() {
        let s = "(a)(a)(a)aaa".to_string();
        let knowledge = vec![vec_string!["a", "yes"]];

        let result = "yesyesyesaaa".to_string();

        assert_eq!(Solution::evaluate(s, knowledge), result);
    }
}

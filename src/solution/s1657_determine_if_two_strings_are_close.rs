/**
 * [1657] Determine if Two Strings Are Close
 *
 * Two strings are considered close if you can attain one from the other using the following operations:
 *
 * 	Operation 1: Swap any two existing characters.
 *
 * 		For example, a<u>b</u>cd<u>e</u> -> a<u>e</u>cd<u>b</u>
 *
 *
 * 	Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
 *
 * 		For example, <u>aa</u>c<u>abb</u> -> <u>bb</u>c<u>baa</u> (all a's turn into b's, and all b's turn into a's)
 *
 *
 *
 * You can use the operations on either string as many times as necessary.
 * Given two strings, word1 and word2, return true if word1 and word2 are close, and false otherwise.
 *  
 * Example 1:
 *
 * Input: word1 = "abc", word2 = "bca"
 * Output: true
 * Explanation: You can attain word2 from word1 in 2 operations.
 * Apply Operation 1: "a<u>bc</u>" -> "a<u>cb</u>"
 * Apply Operation 1: "<u>a</u>c<u>b</u>" -> "<u>b</u>c<u>a</u>"
 *
 * Example 2:
 *
 * Input: word1 = "a", word2 = "aa"
 * Output: false
 * Explanation: It is impossible to attain word2 from word1, or vice versa, in any number of operations.
 *
 * Example 3:
 *
 * Input: word1 = "cabbba", word2 = "abbccc"
 * Output: true
 * Explanation: You can attain word2 from word1 in 3 operations.
 * Apply Operation 1: "ca<u>b</u>bb<u>a</u>" -> "ca<u>a</u>bb<u>b</u>"
 * Apply Operation 2: "<u>c</u>aa<u>bbb</u>" -> "<u>b</u>aa<u>ccc</u>"
 * Apply Operation 2: "<u>baa</u>ccc" -> "<u>abb</u>ccc"
 *
 *  
 * Constraints:
 *
 * 	1 <= word1.length, word2.length <= 10^5
 * 	word1 and word2 contain only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/determine-if-two-strings-are-close/
// discuss: https://leetcode.com/problems/determine-if-two-strings-are-close/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let (mut word1_v, mut word1_c) =
            word1
                .chars()
                .fold((vec![0; 128], vec![0; 128]), |mut acc, c| {
                    acc.0[c as usize] += 1;
                    acc.1[c as usize] = 1;
                    acc
                });

        word1_v.sort_unstable();

        let (mut word2_v, mut word2_c) =
            word2
                .chars()
                .fold((vec![0; 128], vec![0; 128]), |mut acc, c| {
                    acc.0[c as usize] += 1;
                    acc.1[c as usize] = 1;
                    acc
                });

        word2_v.sort_unstable();

        !word1_v.iter().zip(word2_v).any(|(c1, c2)| *c1 != c2)
            && !word1_c.iter().zip(word2_c).any(|(c1, c2)| *c1 != c2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1657_example_1() {
        let word1 = "abc".to_string();
        let word2 = "bca".to_string();

        let result = true;

        assert_eq!(Solution::close_strings(word1, word2), result);
    }

    fn test_1657_example_2() {
        let word1 = "a".to_string();
        let word2 = "aa".to_string();

        let result = false;

        assert_eq!(Solution::close_strings(word1, word2), result);
    }

    fn test_1657_example_3() {
        let word1 = "cabbba".to_string();
        let word2 = "abbccc".to_string();

        let result = true;

        assert_eq!(Solution::close_strings(word1, word2), result);
    }
}

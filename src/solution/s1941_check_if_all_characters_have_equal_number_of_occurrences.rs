/**
 * [1941] Check if All Characters Have Equal Number of Occurrences
 *
 * Given a string s, return true if s is a good string, or false otherwise.
 * A string s is good if all the characters that appear in s have the same number of occurrences (i.e., the same frequency).
 *  
 * Example 1:
 *
 * Input: s = "abacbc"
 * Output: true
 * Explanation: The characters that appear in s are 'a', 'b', and 'c'. All characters occur 2 times in s.
 *
 * Example 2:
 *
 * Input: s = "aaabb"
 * Output: false
 * Explanation: The characters that appear in s are 'a' and 'b'.
 * 'a' occurs 3 times while 'b' occurs 2 times, which is not the same number of times.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/
// discuss: https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let hm: std::collections::HashMap<char, u32> =
            s.chars()
                .fold(std::collections::HashMap::new(), |mut hm, c| {
                    *hm.entry(c).or_insert(0) += 1;
                    hm
                });

        let expected: &u32 = hm.get(&s.chars().next().unwrap()).unwrap();

        hm.iter().all(|(_, count)| count == expected)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1941_example_1() {
        let s = "abacbc".to_string();

        let result = true;

        assert_eq!(Solution::are_occurrences_equal(s), result);
    }

    #[test]
    fn test_1941_example_2() {
        let s = "aaabb".to_string();

        let result = false;

        assert_eq!(Solution::are_occurrences_equal(s), result);
    }
}

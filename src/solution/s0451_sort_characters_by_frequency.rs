/**
 * [0451] Sort Characters By Frequency
 *
 * Given a string s, sort it in decreasing order based on the frequency of the characters. The frequency of a character is the number of times it appears in the string.
 * Return the sorted string. If there are multiple answers, return any of them.
 *  
 * Example 1:
 *
 * Input: s = "tree"
 * Output: "eert"
 * Explanation: 'e' appears twice while 'r' and 't' both appear once.
 * So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.
 *
 * Example 2:
 *
 * Input: s = "cccaaa"
 * Output: "aaaccc"
 * Explanation: Both 'c' and 'a' appear three times, so both "cccaaa" and "aaaccc" are valid answers.
 * Note that "cacaca" is incorrect, as the same characters must be together.
 *
 * Example 3:
 *
 * Input: s = "Aabb"
 * Output: "bbAa"
 * Explanation: "bbaA" is also a valid answer, but "Aabb" is incorrect.
 * Note that 'A' and 'a' are treated as two different characters.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 5 * 10^5
 * 	s consists of uppercase and lowercase English letters and digits.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-characters-by-frequency/
// discuss: https://leetcode.com/problems/sort-characters-by-frequency/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut mp = std::collections::HashMap::new();
        s.chars().for_each(|b| *mp.entry(b).or_insert(0) += 1);
        let mut v = mp.into_iter().map(|(c, n)| (c, n)).collect::<Vec<_>>();
        v.sort_unstable_by_key(|pair| -pair.1);
        v.into_iter()
            .flat_map(|(c, n)| vec![c; n as usize])
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0451_example_1() {
        let s = "tree".to_string();
        let result = "eert".to_string();

        assert_eq!(Solution::frequency_sort(s), result);
    }

    #[test]
    #[ignore]
    fn test_0451_example_2() {
        let s = "cccaaa".to_string();
        let result = "aaaccc".to_string();

        assert_eq!(Solution::frequency_sort(s), result);
    }

    #[test]
    #[ignore]
    fn test_0451_example_3() {
        let s = "Aabb".to_string();
        let result = "bbAa".to_string();

        assert_eq!(Solution::frequency_sort(s), result);
    }
}

/**
 * [1593] Split a String Into the Max Number of Unique Substrings
 *
 * Given a string s<var>,</var> return the maximum number of unique substrings that the given string can be split into.
 * You can split string s into any list of non-empty substrings, where the concatenation of the substrings forms the original string. However, you must split the substrings such that all of them are unique.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * Example 1:
 *
 * Input: s = "ababccc"
 * Output: 5
 * Explanation: One way to split maximally is ['a', 'b', 'ab', 'c', 'cc']. Splitting like ['a', 'b', 'a', 'b', 'c', 'cc'] is not valid as you have 'a' and 'b' multiple times.
 *
 * Example 2:
 *
 * Input: s = "aba"
 * Output: 2
 * Explanation: One way to split maximally is ['a', 'ba'].
 *
 * Example 3:
 *
 * Input: s = "aa"
 * Output: 1
 * Explanation: It is impossible to split the string any further.
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= s.length <= 16
 *
 *
 * 	s contains only lower case English letters.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/
// discuss: https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/solutions/2124953/rust-recursion-backtracking-100-100-80ms/
    pub fn max_unique_split(s: String) -> i32 {
        let mut combos = std::collections::HashSet::new();
        let mut result = 0;
        let chars = s.chars().map(|c| c.to_string()).collect::<Vec<String>>();
        Self::dfs_helper(&mut combos, &chars, &mut result);

        result as i32
    }

    fn dfs_helper(
        combos: &mut std::collections::HashSet<String>,
        chars: &[String],
        result: &mut usize,
    ) {
        if !chars.is_empty() {
            for i in 0..chars.len() {
                let mut substr =
                    chars[0..i + 1]
                        .iter()
                        .fold(String::with_capacity(i), |mut new_str, s| {
                            new_str.push_str(s);
                            new_str
                        });
                if !combos.contains(&substr) {
                    combos.insert(substr.clone());
                    Self::dfs_helper(combos, &chars[i + 1..], result);
                    combos.remove(&substr);
                }
            }
        } else {
            if combos.len() > *result {
                *result = combos.len();
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1593_example_1() {
        let s = "ababccc".to_string();

        let result = 5;

        assert_eq!(Solution::max_unique_split(s), result);
    }

    #[test]
    fn test_1593_example_2() {
        let s = "aba".to_string();

        let result = 2;

        assert_eq!(Solution::max_unique_split(s), result);
    }

    #[test]
    fn test_1593_example_3() {
        let s = "aa".to_string();

        let result = 1;

        assert_eq!(Solution::max_unique_split(s), result);
    }
}

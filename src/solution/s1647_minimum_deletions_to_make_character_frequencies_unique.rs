/**
 * [1647] Minimum Deletions to Make Character Frequencies Unique
 *
 * A string s is called good if there are no two different characters in s that have the same frequency.
 * Given a string s, return the minimum number of characters you need to delete to make s good.
 * The frequency of a character in a string is the number of times it appears in the string. For example, in the string "aab", the frequency of 'a' is 2, while the frequency of 'b' is 1.
 *  
 * Example 1:
 *
 * Input: s = "aab"
 * Output: 0
 * Explanation: s is already good.
 *
 * Example 2:
 *
 * Input: s = "aaabbbcc"
 * Output: 2
 * Explanation: You can delete two 'b's resulting in the good string "aaabcc".
 * Another way it to delete one 'b' and one 'c' resulting in the good string "aaabbc".
 * Example 3:
 *
 * Input: s = "ceabaacb"
 * Output: 2
 * Explanation: You can delete both 'c's resulting in the good string "eabaab".
 * Note that we only care about characters that are still in the string at the end (i.e. frequency of 0 is ignored).
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/
// discuss: https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut nums = std::collections::HashSet::new();
        let mut freq = vec![0; 26];
        s.chars()
            .map(|c| (c as u8 - 'a' as u8) as usize)
            .for_each(|i| freq[i] += 1);

        freq.iter_mut().fold(0, |mut result, f| {
            while !nums.insert(*f) && *f > 0 {
                *f -= 1;
                result += 1;
            }
            result
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1647_example_1() {
        let s = "aab".to_string();

        let result = 0;

        assert_eq!(Solution::min_deletions(s), result);
    }

    #[test]
    fn test_1647_example_2() {
        let s = "aaabbbcc".to_string();

        let result = 2;

        assert_eq!(Solution::min_deletions(s), result);
    }

    #[test]
    fn test_1647_example_3() {
        let s = "ceabaacb".to_string();

        let result = 2;

        assert_eq!(Solution::min_deletions(s), result);
    }
}

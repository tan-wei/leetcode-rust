/**
 * [1525] Number of Good Ways to Split a String
 *
 * You are given a string s.
 * A split is called good if you can split s into two non-empty strings sleft and sright where their concatenation is equal to s (i.e., sleft + sright = s) and the number of distinct letters in sleft and sright is the same.
 * Return the number of good splits you can make in s.
 *  
 * Example 1:
 *
 * Input: s = "aacaba"
 * Output: 2
 * Explanation: There are 5 ways to split "aacaba" and 2 of them are good.
 * ("a", "acaba") Left string and right string contains 1 and 3 different letters respectively.
 * ("aa", "caba") Left string and right string contains 1 and 3 different letters respectively.
 * ("aac", "aba") Left string and right string contains 2 and 2 different letters respectively (good split).
 * ("aaca", "ba") Left string and right string contains 2 and 2 different letters respectively (good split).
 * ("aacab", "a") Left string and right string contains 3 and 1 different letters respectively.
 *
 * Example 2:
 *
 * Input: s = "abcd"
 * Output: 1
 * Explanation: Split the string as follows ("ab", "cd").
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-good-ways-to-split-a-string/
// discuss: https://leetcode.com/problems/number-of-good-ways-to-split-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();

        let mut front = vec![0; n];
        let mut back = vec![0; n];
        let mut count = 0;
        let mut memo = vec![false; 26];

        for i in 0..n {
            let c = (s[i] as u8 - 'a' as u8) as usize;
            if !memo[c] {
                count += 1;
                memo[c] = true;
            }
            front[i] = count;
        }

        let mut count = 0;
        let mut memo = vec![false; 26];
        for i in (0..n).rev() {
            let c = (s[i] as u8 - 'a' as u8) as usize;
            if !memo[c] {
                count += 1;
                memo[c] = true;
            }
            back[i] = count;
        }

        let mut result = 0;
        for i in 0..n - 1 {
            if front[i] == back[i + 1] {
                result += 1;
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
    fn test_1525_example_1() {
        let s = "aacaba".to_string();

        let result = 2;

        assert_eq!(Solution::num_splits(s), result);
    }

    #[test]
    fn test_1525_example_2() {
        let s = "aacaba".to_string();

        let result = 2;

        assert_eq!(Solution::num_splits(s), result);
    }
}

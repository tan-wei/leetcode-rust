/**
 * [0387] First Unique Character in a String
 *
 * Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.
 *  
 * Example 1:
 * Input: s = "leetcode"
 * Output: 0
 * Example 2:
 * Input: s = "loveleetcode"
 * Output: 2
 * Example 3:
 * Input: s = "aabb"
 * Output: -1
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/first-unique-character-in-a-string/
// discuss: https://leetcode.com/problems/first-unique-character-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counter = [0; 26];
        s.bytes()
            .map(|b| (b - b'a') as usize)
            .for_each(|b| counter[b] += 1);
        s.bytes()
            .map(|b| (b - b'a') as usize)
            .enumerate()
            .find(|(_, b)| counter[*b] == 1)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0387_example_1() {
        let s = "leetcode".to_string();
        let result = 0;

        assert_eq!(Solution::first_uniq_char(s), result);
    }

    #[test]
    fn test_0387_example_2() {
        let s = "loveleetcode".to_string();
        let result = 2;

        assert_eq!(Solution::first_uniq_char(s), result);
    }

    #[test]
    fn test_0387_example_3() {
        let s = "aabb".to_string();
        let result = -1;

        assert_eq!(Solution::first_uniq_char(s), result);
    }
}

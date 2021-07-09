/**
 * [214] Shortest Palindrome
 *
 * You are given a string s. You can convert s to a palindrome by adding characters in front of it.
 * Return the shortest palindrome you can find by performing this transformation.
 *  
 * Example 1:
 * Input: s = "aacecaaa"
 * Output: "aaacecaaa"
 * Example 2:
 * Input: s = "abcd"
 * Output: "dcbabcd"
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of lowercase English letters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-palindrome/
// discuss: https://leetcode.com/problems/shortest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let len = s.len();
        if len == 0 {
            return "".to_string();
        }
        let len = s.len();
        let origin = s;
        let rev: String = origin.chars().rev().collect();
        let target = (0..len)
            .find(|&i| {
                let origin_part = &origin[0..len - i];
                let rev_part = &rev[i..len];
                rev_part == origin_part
            })
            .expect("it won't happen");
        let mut result = rev;
        result.push_str(&origin[len - target..len]);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0214_example_1() {
        let s = "aacecaaa".to_string();
        let result = "aaacecaaa".to_string();

        assert_eq!(Solution::shortest_palindrome(s), result);
    }

    #[test]
    fn test_0214_example_2() {
        let s = "abcd".to_string();
        let result = "dcbabcd".to_string();

        assert_eq!(Solution::shortest_palindrome(s), result);
    }
}

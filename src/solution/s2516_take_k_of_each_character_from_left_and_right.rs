/**
 * [2516] Take K of Each Character From Left and Right
 *
 * You are given a string s consisting of the characters 'a', 'b', and 'c' and a non-negative integer k. Each minute, you may take either the leftmost character of s, or the rightmost character of s.
 * Return the minimum number of minutes needed for you to take at least k of each character, or return -1 if it is not possible to take k of each character.
 *  
 * Example 1:
 *
 * Input: s = "aabaaaacaabc", k = 2
 * Output: 8
 * Explanation:
 * Take three characters from the left of s. You now have two 'a' characters, and one 'b' character.
 * Take five characters from the right of s. You now have four 'a' characters, two 'b' characters, and two 'c' characters.
 * A total of 3 + 5 = 8 minutes is needed.
 * It can be proven that 8 is the minimum number of minutes needed.
 *
 * Example 2:
 *
 * Input: s = "a", k = 1
 * Output: -1
 * Explanation: It is not possible to take one 'b' or 'c' so return -1.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only the letters 'a', 'b', and 'c'.
 * 	0 <= k <= s.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/
// discuss: https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut m = s.iter().fold(vec![0; 3], |mut v, &c| {
            v[c as usize - 97] += 1;
            v
        });
        if m[0] < k || m[1] < k || m[2] < k {
            return -1;
        }
        (0..s.len())
            .fold((i32::MAX, 0), |(res, mut l), r| {
                m[s[r] as usize - 97] -= 1;
                while m[0] < k || m[1] < k || m[2] < k {
                    m[s[l] as usize - 97] += 1;
                    l += 1;
                }
                (res.min((s.len() - r + l - 1) as i32), l)
            })
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2516_example_1() {
        let s = "aabaaaacaabc".to_string();
        let k = 2;

        let result = 8;

        assert_eq!(Solution::take_characters(s, k), result);
    }

    #[test]
    fn test_2516_example_2() {
        let s = "a".to_string();
        let k = 1;

        let result = -1;

        assert_eq!(Solution::take_characters(s, k), result);
    }
}

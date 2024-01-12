/**
 * [1456] Maximum Number of Vowels in a Substring of Given Length
 *
 * Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.
 * Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.
 *  
 * Example 1:
 *
 * Input: s = "abciiidef", k = 3
 * Output: 3
 * Explanation: The substring "iii" contains 3 vowel letters.
 *
 * Example 2:
 *
 * Input: s = "aeiou", k = 2
 * Output: 2
 * Explanation: Any substring of length 2 contains 2 vowels.
 *
 * Example 3:
 *
 * Input: s = "leetcode", k = 3
 * Output: 2
 * Explanation: "lee", "eet" and "ode" contain 2 vowels.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 * 	1 <= k <= s.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/
// discuss: https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let count = s.iter().take(k).filter(|&x| Self::is_vowel(*x)).count();

        s.iter()
            .skip(k)
            .zip(s.iter())
            .fold((count, count), |a, x| {
                let (max, count) = a;
                let (cur, left) = x;
                match (Self::is_vowel(*left), Self::is_vowel(*cur)) {
                    (false, true) => (max.max(count + 1), count + 1),
                    (true, false) => (max, count - 1),
                    _ => (max, count),
                }
            })
            .0 as i32
    }

    fn is_vowel(ch: u8) -> bool {
        match ch {
            b'a' | b'e' | b'i' | b'o' | b'u' => true,
            _ => false,
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1456_example_1() {
        let s = "abciiidef".to_string();
        let k = 3;

        let result = 3;

        assert_eq!(Solution::max_vowels(s, k), result);
    }

    #[test]
    fn test_1456_example_2() {
        let s = "aeiou".to_string();
        let k = 2;

        let result = 2;

        assert_eq!(Solution::max_vowels(s, k), result);
    }

    #[test]
    fn test_1456_example_3() {
        let s = "leetcode".to_string();
        let k = 3;

        let result = 2;

        assert_eq!(Solution::max_vowels(s, k), result);
    }
}

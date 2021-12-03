/**
 * [0438] Find All Anagrams in a String
 *
 * Given two strings s and p, return an array of all the start indices of p's anagrams in s. You may return the answer in any order.
 * An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
 *  
 * Example 1:
 *
 * Input: s = "cbaebabacd", p = "abc"
 * Output: [0,6]
 * Explanation:
 * The substring with start index = 0 is "cba", which is an anagram of "abc".
 * The substring with start index = 6 is "bac", which is an anagram of "abc".
 *
 * Example 2:
 *
 * Input: s = "abab", p = "ab"
 * Output: [0,1,2]
 * Explanation:
 * The substring with start index = 0 is "ab", which is an anagram of "ab".
 * The substring with start index = 1 is "ba", which is an anagram of "ab".
 * The substring with start index = 2 is "ab", which is an anagram of "ab".
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length, p.length <= 3 * 10^4
 * 	s and p consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-anagrams-in-a-string/
// discuss: https://leetcode.com/problems/find-all-anagrams-in-a-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/find-all-anagrams-in-a-string/discuss/748941/O(N)-or-C%2B%2B-%40-~12-32ms-or-Golang-%40-~0-8ms-or-Rust-%40-0ms
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut reslut = Vec::new();

        if s.len() < p.len() {
            return reslut;
        }

        let usizeof = |bc: u8| (bc - 97) as usize;
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut sw = [0u8; 26];
        let mut pw = [0u8; 26];

        // initialize windows
        for i in 0..p.len() {
            pw[usizeof(p[i])] += 1;
            sw[usizeof(s[i])] += 1;
        }

        for i in p.len() - 1..s.len() {
            if sw == pw {
                // valid anagram
                reslut.push((i + 1 - p.len()) as i32);
            }
            // update windows
            if i + 1 < s.len() {
                sw[usizeof(s[i + 1 - p.len()])] -= 1;
                sw[usizeof(s[i + 1])] += 1;
            }
        }
        reslut
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0438_example_1() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let result = vec![0, 6];

        assert_eq!(Solution::find_anagrams(s, p), result);
    }

    #[test]
    fn test_0438_example_2() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        let result = vec![0, 1, 2];

        assert_eq!(Solution::find_anagrams(s, p), result);
    }
}

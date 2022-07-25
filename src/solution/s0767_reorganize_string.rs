/**
 * [0767] Reorganize String
 *
 * Given a string s, rearrange the characters of s so that any two adjacent characters are not the same.
 * Return any possible rearrangement of s or return "" if not possible.
 *  
 * Example 1:
 * Input: s = "aab"
 * Output: "aba"
 * Example 2:
 * Input: s = "aaab"
 * Output: ""
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reorganize-string/
// discuss: https://leetcode.com/problems/reorganize-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut result = String::with_capacity(s.len());

        let mut characters = [0; 26];
        for ch in s.as_bytes().iter().copied() {
            characters[(ch - b'a') as usize] += 1;
        }

        let mut heap = std::collections::BinaryHeap::with_capacity(26);
        for (idx, count) in characters.iter().copied().enumerate() {
            if count > 0 {
                heap.push((count, (idx as u8 + b'a') as char));
            }
        }

        let mut last = None;
        while let Some((mut count, ch)) = heap.pop() {
            result.push(ch);
            count -= 1;

            if let Some(last) = last.take() {
                heap.push(last);
            }

            if count > 0 {
                last = Some((count, ch));
            }
        }

        if last.is_some() {
            result.truncate(0);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0767_example_1() {
        let s = "aab".to_string();
        let result = "aba".to_string();

        assert_eq!(Solution::reorganize_string(s), result);
    }

    #[test]
    fn test_0767_example_2() {
        let s = "aaab".to_string();
        let result = "".to_string();

        assert_eq!(Solution::reorganize_string(s), result);
    }
}

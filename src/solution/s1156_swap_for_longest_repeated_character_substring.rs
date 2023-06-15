/**
 * [1156] Swap For Longest Repeated Character Substring
 *
 * You are given a string text. You can swap two of the characters in the text.
 * Return the length of the longest substring with repeated characters.
 *  
 * Example 1:
 *
 * Input: text = "ababa"
 * Output: 3
 * Explanation: We can swap the first 'b' with the last 'a', or the last 'b' with the first 'a'. Then, the longest repeated character substring is "aaa" with length 3.
 *
 * Example 2:
 *
 * Input: text = "aaabaaa"
 * Output: 6
 * Explanation: Swap 'b' with the last 'a' (or the first 'a'), and we get longest repeated character substring "aaaaaa" with length 6.
 *
 * Example 3:
 *
 * Input: text = "aaaaa"
 * Output: 5
 * Explanation: No need to swap, longest repeated character substring is "aaaaa" with length is 5.
 *
 *  
 * Constraints:
 *
 * 	1 <= text.length <= 2 * 10^4
 * 	text consist of lowercase English characters only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/swap-for-longest-repeated-character-substring/
// discuss: https://leetcode.com/problems/swap-for-longest-repeated-character-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let (mut cache, mut freq, mut left, text_arr) =
            (Vec::new(), vec![0; 26], 0, text.as_bytes());
        for i in 0..=text.len() {
            if i < text.len() {
                freq[(text_arr[i] - b'a') as usize] += 1;
            }
            if i == text.len() || i < text.len() && text_arr[i] != text_arr[left] {
                cache.push((text_arr[left], (i - left) as i32));
                left = i;
            }
        }
        let mut result = 0;
        for i in 0..cache.len() {
            let mut len = cache[i].1;
            if i + 2 < cache.len() && cache[i + 2].0 == cache[i].0 && cache[i + 1].1 == 1 {
                len += cache[i + 2].1;
            }
            result = result.max(
                len + (if len < freq[(cache[i].0 - b'a') as usize] {
                    1
                } else {
                    0
                }),
            );
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1156_example_1() {
        let text = "ababa".to_string();
        let result = 3;

        assert_eq!(Solution::max_rep_opt1(text), result);
    }

    #[test]
    fn test_1156_example_2() {
        let text = "aaabaaa".to_string();
        let result = 6;

        assert_eq!(Solution::max_rep_opt1(text), result);
    }

    #[test]
    fn test_1156_example_3() {
        let text = "aaaaa".to_string();
        let result = 5;

        assert_eq!(Solution::max_rep_opt1(text), result);
    }
}

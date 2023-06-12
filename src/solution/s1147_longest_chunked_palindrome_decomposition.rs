/**
 * [1147] Longest Chunked Palindrome Decomposition
 *
 * You are given a string text. You should split it to k substrings (subtext1, subtext2, ..., subtextk) such that:
 *
 * 	subtexti is a non-empty string.
 * 	The concatenation of all the substrings is equal to text (i.e., subtext1 + subtext2 + ... + subtextk == text).
 * 	subtexti == subtextk - i + 1 for all valid values of i (i.e., 1 <= i <= k).
 *
 * Return the largest possible value of k.
 *  
 * Example 1:
 *
 * Input: text = "ghiabcdefhelloadamhelloabcdefghi"
 * Output: 7
 * Explanation: We can split the string on "(ghi)(abcdef)(hello)(adam)(hello)(abcdef)(ghi)".
 *
 * Example 2:
 *
 * Input: text = "merchant"
 * Output: 1
 * Explanation: We can split the string on "(merchant)".
 *
 * Example 3:
 *
 * Input: text = "antaprezatepzapreanta"
 * Output: 11
 * Explanation: We can split the string on "(a)(nt)(a)(pre)(za)(tep)(za)(pre)(a)(nt)(a)".
 *
 *  
 * Constraints:
 *
 * 	1 <= text.length <= 1000
 * 	text consists only of lowercase English characters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-chunked-palindrome-decomposition/
// discuss: https://leetcode.com/problems/longest-chunked-palindrome-decomposition/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/longest-chunked-palindrome-decomposition/solutions/2811418/fastest-and-also-rather-straight-forward-rust-solution-by-matching-substrings-o-n/
    pub fn longest_decomposition(text: String) -> i32 {
        let mut singlecentre = true;
        let mut token_count = 0;
        let mut prev_token_end = 0;
        let len = text.len();
        let maxi = (len) / 2 + 1;

        for i in 1..maxi {
            if (i <= prev_token_end) {
                continue;
            }
            if text[prev_token_end..i] == text[(len - i)..(len - prev_token_end)] {
                token_count += 1;
                if i >= maxi - 1 && len % 2 != 1 {
                    singlecentre = false;
                    if prev_token_end > i - 1 {
                        token_count -= 1;
                    }
                }
                prev_token_end = i;
            }
        }

        token_count * 2 + if singlecentre { 1 } else { 0 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1147_example_1() {
        let text = "ghiabcdefhelloadamhelloabcdefghi".to_string();
        let result = 7;

        assert_eq!(Solution::longest_decomposition(text), result);
    }

    #[test]
    fn test_1147_example_2() {
        let text = "merchant".to_string();
        let result = 1;

        assert_eq!(Solution::longest_decomposition(text), result);
    }

    #[test]
    fn test_1147_example_3() {
        let text = "antaprezatepzapreanta".to_string();
        let result = 11;

        assert_eq!(Solution::longest_decomposition(text), result);
    }
}

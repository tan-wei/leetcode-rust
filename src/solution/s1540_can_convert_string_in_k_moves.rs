/**
 * [1540] Can Convert String in K Moves
 *
 * Given two strings s and t, your goal is to convert s into t in k moves or less.
 * During the i^th (<font face="monospace">1 <= i <= k) </font>move you can:
 *
 * 	Choose any index j (1-indexed) from s, such that 1 <= j <= s.length and j has not been chosen in any previous move, and shift the character at that index i times.
 * 	Do nothing.
 *
 * Shifting a character means replacing it by the next letter in the alphabet (wrapping around so that 'z' becomes 'a'). Shifting a character by i means applying the shift operations i times.
 * Remember that any index j can be picked at most once.
 * Return true if it's possible to convert s into t in no more than k moves, otherwise return false.
 *  
 * Example 1:
 *
 * Input: s = "input", t = "ouput", k = 9
 * Output: true
 * Explanation: In the 6th move, we shift 'i' 6 times to get 'o'. And in the 7th move we shift 'n' to get 'u'.
 *
 * Example 2:
 *
 * Input: s = "abc", t = "bcd", k = 10
 * Output: false
 * Explanation: We need to shift each character in s one time to convert it into t. We can shift 'a' to 'b' during the 1st move. However, there is no way to shift the other characters in the remaining moves to obtain t from s.
 *
 * Example 3:
 *
 * Input: s = "aab", t = "bbb", k = 27
 * Output: true
 * Explanation: In the 1st move, we shift the first 'a' 1 time to get 'b'. In the 27th move, we shift the second 'a' 27 times to get 'b'.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length, t.length <= 10^5
 * 	0 <= k <= 10^9
 * 	s, t contain only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/can-convert-string-in-k-moves/
// discuss: https://leetcode.com/problems/can-convert-string-in-k-moves/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let diffs = s
            .bytes()
            .zip(t.bytes())
            .map(|(x, y)| (y as i32 - x as i32 + 26) % 26);
        let diffs = diffs.into_iter().filter(|&x| x != 0);
        let mut map = std::collections::HashMap::new();
        for diff in diffs {
            let multiplier = *map.entry(diff).or_insert(0);
            if diff + (26 * multiplier) > k {
                return false;
            }
            *map.entry(diff).or_insert(0) += 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1540_example_1() {
        let s = "input".to_string();
        let t = "ouput".to_string();
        let k = 9;

        let result = true;

        assert_eq!(Solution::can_convert_string(s, t, k), result);
    }

    #[test]
    fn test_1540_example_2() {
        let s = "abc".to_string();
        let t = "bcd".to_string();
        let k = 10;

        let result = false;

        assert_eq!(Solution::can_convert_string(s, t, k), result);
    }

    #[test]
    fn test_1540_example_3() {
        let s = "aab".to_string();
        let t = "bbb".to_string();
        let k = 27;

        let result = true;

        assert_eq!(Solution::can_convert_string(s, t, k), result);
    }
}

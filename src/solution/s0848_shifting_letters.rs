/**
 * [0848] Shifting Letters
 *
 * You are given a string s of lowercase English letters and an integer array shifts of the same length.
 * Call the shift() of a letter, the next letter in the alphabet, (wrapping around so that 'z' becomes 'a').
 *
 * 	For example, shift('a') = 'b', shift('t') = 'u', and shift('z') = 'a'.
 *
 * Now for each shifts[i] = x, we want to shift the first i + 1 letters of s, x times.
 * Return the final string after all such shifts to s are applied.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abc", shifts = [3,5,9]
 * Output: "rpl"
 * Explanation: We start with "abc".
 * After shifting the first 1 letters of s by 3, we have "dbc".
 * After shifting the first 2 letters of s by 5, we have "igc".
 * After shifting the first 3 letters of s by 9, we have "rpl", the answer.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "aaa", shifts = [1,2,3]
 * Output: "gfd"
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 * 	shifts.length == s.length
 * 	0 <= shifts[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shifting-letters/
// discuss: https://leetcode.com/problems/shifting-letters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/shifting-letters/solutions/1451915/rust-solution/
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        s.bytes()
            .zip(&shifts)
            .scan(
                shifts.iter().fold(0, |acc, &x| (acc + x) % 26),
                |state, (u, &shift)| {
                    let s = *state as u8;
                    *state = (*state - shift).rem_euclid(26);
                    Some((b'a' + (u - b'a' + s) % 26) as char)
                },
            )
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0848_example_1() {
        let s = "abc".to_string();
        let shifts = vec![3, 5, 9];
        let result = "rpl".to_string();

        assert_eq!(Solution::shifting_letters(s, shifts), result);
    }

    #[test]
    fn test_0848_example_2() {
        let s = "aaa".to_string();
        let shifts = vec![1, 2, 3];
        let result = "gfd".to_string();

        assert_eq!(Solution::shifting_letters(s, shifts), result);
    }
}

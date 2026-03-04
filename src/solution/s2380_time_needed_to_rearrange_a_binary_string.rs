/**
 * [2380] Time Needed to Rearrange a Binary String
 *
 * You are given a binary string s. In one second, all occurrences of "01" are simultaneously replaced with "10". This process repeats until no occurrences of "01" exist.
 * Return the number of seconds needed to complete this process.
 *  
 * Example 1:
 *
 * Input: s = "0110101"
 * Output: 4
 * Explanation:
 * After one second, s becomes "1011010".
 * After another second, s becomes "1101100".
 * After the third second, s becomes "1110100".
 * After the fourth second, s becomes "1111000".
 * No occurrence of "01" exists any longer, and the process needed 4 seconds to complete,
 * so we return 4.
 *
 * Example 2:
 *
 * Input: s = "11100"
 * Output: 0
 * Explanation:
 * No occurrence of "01" exists in s, and the processes needed 0 seconds to complete,
 * so we return 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 1000
 * 	s[i] is either '0' or '1'.
 *
 *  
 * Follow up:
 * Can you solve this problem in O(n) time complexity?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/time-needed-to-rearrange-a-binary-string/
// discuss: https://leetcode.com/problems/time-needed-to-rearrange-a-binary-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/time-needed-to-rearrange-a-binary-string/solutions/2554095/rust-0-ms-functional-style-one-liner-wit-9yhx/
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        s.chars()
            .skip_while(|&c| c == '1')
            .fold((0, 0), |(swaps, zeros), c| match c {
                '1' => ((swaps + 1).max(zeros), zeros),
                '0' => (swaps, zeros + 1),
                _ => unreachable!("Wrong symbol {}", c),
            })
            .0 as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2380_example_1() {
        let s = "0110101".to_string();

        let result = 4;

        assert_eq!(Solution::seconds_to_remove_occurrences(s), result);
    }

    #[test]
    fn test_2380_example_2() {
        let s = "11100".to_string();

        let result = 0;

        assert_eq!(Solution::seconds_to_remove_occurrences(s), result);
    }
}

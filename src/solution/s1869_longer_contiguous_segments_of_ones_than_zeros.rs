/**
 * [1869] Longer Contiguous Segments of Ones than Zeros
 *
 * Given a binary string s, return true if the longest contiguous segment of 1's is strictly longer than the longest contiguous segment of 0's in s, or return false otherwise.
 *
 * 	For example, in s = "<u>11</u>01<u>000</u>10" the longest continuous segment of 1s has length 2, and the longest continuous segment of 0s has length 3.
 *
 * Note that if there are no 0's, then the longest continuous segment of 0's is considered to have a length 0. The same applies if there is no 1's.
 *  
 * Example 1:
 *
 * Input: s = "1101"
 * Output: true
 * Explanation:
 * The longest contiguous segment of 1s has length 2: "<u>11</u>01"
 * The longest contiguous segment of 0s has length 1: "11<u>0</u>1"
 * The segment of 1s is longer, so return true.
 *
 * Example 2:
 *
 * Input: s = "111000"
 * Output: false
 * Explanation:
 * The longest contiguous segment of 1s has length 3: "<u>111</u>000"
 * The longest contiguous segment of 0s has length 3: "111<u>000</u>"
 * The segment of 1s is not longer, so return false.
 *
 * Example 3:
 *
 * Input: s = "110100010"
 * Output: false
 * Explanation:
 * The longest contiguous segment of 1s has length 2: "<u>11</u>0100010"
 * The longest contiguous segment of 0s has length 3: "1101<u>000</u>10"
 * The segment of 1s is not longer, so return false.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longer-contiguous-segments-of-ones-than-zeros/
// discuss: https://leetcode.com/problems/longer-contiguous-segments-of-ones-than-zeros/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let chs = s.chars().collect::<Vec<char>>();
        let mut longest_one: u8 = 0;
        let mut ones_count: u8 = 0;
        let mut longest_zero: u8 = 0;
        let mut zeros_count: u8 = 0;

        for ch in chs.into_iter() {
            if ch == '0' {
                ones_count = 0;
                zeros_count += 1;
                longest_zero = std::cmp::max(longest_zero, zeros_count);
            } else {
                zeros_count = 0;
                ones_count += 1;
                longest_one = std::cmp::max(longest_one, ones_count);
            }
        }

        longest_one > longest_zero
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1869_example_1() {
        let s = "1101".to_string();

        let result = true;

        assert_eq!(Solution::check_zero_ones(s), result);
    }

    #[test]
    fn test_1869_example_2() {
        let s = "111000".to_string();

        let result = false;

        assert_eq!(Solution::check_zero_ones(s), result);
    }

    #[test]
    fn test_1869_example_3() {
        let s = "110100010".to_string();

        let result = false;

        assert_eq!(Solution::check_zero_ones(s), result);
    }
}

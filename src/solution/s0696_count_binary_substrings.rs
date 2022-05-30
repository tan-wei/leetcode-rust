/**
 * [0696] Count Binary Substrings
 *
 * Give a binary string s, return the number of non-empty substrings that have the same number of 0's and 1's, and all the 0's and all the 1's in these substrings are grouped consecutively.
 * Substrings that occur multiple times are counted the number of times they occur.
 *  
 * Example 1:
 *
 * Input: s = "00110011"
 * Output: 6
 * Explanation: There are 6 substrings that have equal number of consecutive 1's and 0's: "0011", "01", "1100", "10", "0011", and "01".
 * Notice that some of these substrings repeat and are counted the number of times they occur.
 * Also, "00110011" is not a valid substring because all the 0's (and 1's) are not grouped together.
 *
 * Example 2:
 *
 * Input: s = "10101"
 * Output: 4
 * Explanation: There are 4 substrings: "10", "01", "10", "01" that have equal number of consecutive 1's and 0's.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either '0' or '1'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-binary-substrings/
// discuss: https://leetcode.com/problems/count-binary-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .fold(vec![1], |mut v, w| {
                if w[0] == w[1] {
                    *v.last_mut().unwrap() += 1;
                } else {
                    v.push(1);
                }
                v
            })
            .windows(2)
            .map(|w| w[0].min(w[1]))
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0696_example_1() {
        let s = "00110011".to_string();
        let result = 6;

        assert_eq!(Solution::count_binary_substrings(s), result);
    }

    #[test]
    fn test_0696_example_2() {
        let s = "10101".to_string();
        let result = 4;

        assert_eq!(Solution::count_binary_substrings(s), result);
    }
}

/**
 * [0467] Unique Substrings in Wraparound String
 *
 * We define the string s to be the infinite wraparound string of "abcdefghijklmnopqrstuvwxyz", so s will look like this:
 *
 * 	"...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".
 *
 * Given a string p, return the number of unique non-empty substrings of p are present in s.
 *  
 * Example 1:
 *
 * Input: p = "a"
 * Output: 1
 * Explanation: Only the substring "a" of p is in s.
 *
 * Example 2:
 *
 * Input: p = "cac"
 * Output: 2
 * Explanation: There are two substrings ("a", "c") of p in s.
 *
 * Example 3:
 *
 * Input: p = "zab"
 * Output: 6
 * Explanation: There are six substrings ("z", "a", "b", "za", "ab", and "zab") of p in s.
 *
 *  
 * Constraints:
 *
 * 	1 <= p.length <= 10^5
 * 	p consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-substrings-in-wraparound-string/
// discuss: https://leetcode.com/problems/unique-substrings-in-wraparound-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let n = p.len();
        let v = p.as_bytes();
        let mut dp = vec![0; 26];
        let mut curr = 0;
        for i in 0..n {
            if i > 0 && (v[i] == v[i - 1] + 1 || v[i] == b'a' && v[i - 1] == b'z') {
                curr += 1;
            } else {
                curr = 1;
            }
            let pos = (v[i] - b'a') as usize;
            if dp[pos] < curr {
                dp[pos] = curr
            }
        }
        dp.iter().fold(0, |acc, &x| acc + x)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0467_example_1() {
        let p = "a".to_string();
        let result = 1;

        assert_eq!(Solution::find_substring_in_wrapround_string(p), result);
    }

    #[test]
    fn test_0467_example_2() {
        let p = "cac".to_string();
        let result = 2;

        assert_eq!(Solution::find_substring_in_wrapround_string(p), result);
    }

    #[test]
    fn test_0467_example_3() {
        let p = "zab".to_string();
        let result = 6;

        assert_eq!(Solution::find_substring_in_wrapround_string(p), result);
    }
}

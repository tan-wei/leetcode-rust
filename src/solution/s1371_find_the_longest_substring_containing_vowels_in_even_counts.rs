/**
 * [1371] Find the Longest Substring Containing Vowels in Even Counts
 *
 * Given the string s, return the size of the longest substring containing each vowel an even number of times. That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.
 *  
 * Example 1:
 *
 * Input: s = "eleetminicoworoep"
 * Output: 13
 * Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.
 *
 * Example 2:
 *
 * Input: s = "leetcodeisgreat"
 * Output: 5
 * Explanation: The longest substring is "leetc" which contains two e's.
 *
 * Example 3:
 *
 * Input: s = "bcbcbc"
 * Output: 6
 * Explanation: In this case, the given string "bcbcbc" is the longest because all vowels: a, e, i, o and u appear zero times.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 5 x 10^5
 * 	s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/
// discuss: https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let limit = 1 << 5;
        let mut dp = vec![vec![]; limit];

        let dict = ['a', 'i', 'u', 'e', 'o'];
        let mut now = 0;
        for i in 0..n {
            dp[now].push(i);
            for j in 0..5 {
                if s[i] == dict[j] {
                    let v = 1 << j;
                    now ^= v;
                    break;
                }
            }
        }
        dp[now].push(n);

        let mut result = 0;
        let mut i = 0;
        for arr in dp {
            if arr.len() >= 2 {
                result = result.max(arr[arr.len() - 1] - arr[0]);
            }
            i += 1;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1371_example_1() {
        let s = "eleetminicoworoep".to_string();

        let result = 13;

        assert_eq!(Solution::find_the_longest_substring(s), result);
    }

    #[test]
    fn test_1371_example_2() {
        let s = "bcbcbc".to_string();

        let result = 6;

        assert_eq!(Solution::find_the_longest_substring(s), result);
    }
}

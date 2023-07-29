/**
 * [1239] Maximum Length of a Concatenated String with Unique Characters
 *
 * You are given an array of strings arr. A string s is formed by the concatenation of a subsequence of arr that has unique characters.
 * Return the maximum possible length of s.
 * A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 *  
 * Example 1:
 *
 * Input: arr = ["un","iq","ue"]
 * Output: 4
 * Explanation: All the valid concatenations are:
 * - ""
 * - "un"
 * - "iq"
 * - "ue"
 * - "uniq" ("un" + "iq")
 * - "ique" ("iq" + "ue")
 * Maximum length is 4.
 *
 * Example 2:
 *
 * Input: arr = ["cha","r","act","ers"]
 * Output: 6
 * Explanation: Possible longest valid concatenations are "chaers" ("cha" + "ers") and "acters" ("act" + "ers").
 *
 * Example 3:
 *
 * Input: arr = ["abcdefghijklmnopqrstuvwxyz"]
 * Output: 26
 * Explanation: The only string in arr has all 26 characters.
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 16
 * 	1 <= arr[i].length <= 26
 * 	arr[i] contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/
// discuss: https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-length-of-a-concatenated-string-with-unique-characters/solutions/1478613/rust-solution/
    pub fn max_length(arr: Vec<String>) -> i32 {
        let arr = arr
            .iter()
            .filter_map(|s| {
                let u = s.bytes().map(|u| 1 << (u - b'a')).sum::<u32>();
                if u.count_ones() == s.len() as u32 {
                    Some(u)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        (0..1 << arr.len())
            .filter_map(|i| {
                (0..arr.len())
                    .filter(|&j| i & 1 << j != 0)
                    .try_fold(0, |acc, j| {
                        if acc & arr[j] == 0 {
                            Some(acc | arr[j])
                        } else {
                            None
                        }
                    })
                    .map(|u| u.count_ones())
            })
            .max()
            .unwrap() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1239_example_1() {
        let arr = vec_string!["un", "iq", "ue"];
        let result = 4;

        assert_eq!(Solution::max_length(arr), result);
    }

    #[test]
    fn test_1239_example_2() {
        let arr = vec_string!["cha", "r", "act", "ers"];
        let result = 6;

        assert_eq!(Solution::max_length(arr), result);
    }

    #[test]
    fn test_1239_example_3() {
        let arr = vec_string!["abcdefghijklmnopqrstuvwxyz"];
        let result = 26;

        assert_eq!(Solution::max_length(arr), result);
    }
}

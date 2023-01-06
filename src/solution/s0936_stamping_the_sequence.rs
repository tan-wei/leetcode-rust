/**
 * [0936] Stamping The Sequence
 *
 * You are given two strings stamp and target. Initially, there is a string s of length target.length with all s[i] == '?'.
 * In one turn, you can place stamp over s and replace every letter in the s with the corresponding letter from stamp.
 *
 * 	For example, if stamp = "abc" and target = "abcba", then s is "?????" initially. In one turn you can:
 *
 * 		place stamp at index 0 of s to obtain "abc??",
 * 		place stamp at index 1 of s to obtain "?abc?", or
 * 		place stamp at index 2 of s to obtain "??abc".
 *
 * 	Note that stamp must be fully contained in the boundaries of s in order to stamp (i.e., you cannot place stamp at index 3 of s).
 *
 * We want to convert s to target using at most 10 * target.length turns.
 * Return an array of the index of the left-most letter being stamped at each turn. If we cannot obtain target from s within 10 * target.length turns, return an empty array.
 *  
 * Example 1:
 *
 * Input: stamp = "abc", target = "ababc"
 * Output: [0,2]
 * Explanation: Initially s = "?????".
 * - Place stamp at index 0 to get "abc??".
 * - Place stamp at index 2 to get "ababc".
 * [1,0,2] would also be accepted as an answer, as well as some other answers.
 *
 * Example 2:
 *
 * Input: stamp = "abca", target = "aabcaca"
 * Output: [3,0,1]
 * Explanation: Initially s = "???????".
 * - Place stamp at index 3 to get "???abca".
 * - Place stamp at index 0 to get "abcabca".
 * - Place stamp at index 1 to get "aabcaca".
 *
 *  
 * Constraints:
 *
 * 	1 <= stamp.length <= target.length <= 1000
 * 	stamp and target consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stamping-the-sequence/
// discuss: https://leetcode.com/problems/stamping-the-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/stamping-the-sequence/solutions/2457790/rust-easy-to-understand-solution-beats-100/
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let mut target = target;
        let mut result = vec![];

        while target != "?".repeat(target.len()) {
            let mut flag = false;
            let mut i = 0;
            while i + stamp.len() <= target.len() {
                if Solution::partial_match(&stamp, &target[i..i + stamp.len()]) {
                    flag = true;
                    result.push(i as i32);
                    target.replace_range(i..i + stamp.len(), &"?".repeat(stamp.len()));
                }
                i += 1;
            }

            if !flag {
                return vec![];
            }
        }
        result.into_iter().rev().collect()
    }

    fn partial_match(stamp: &str, k: &str) -> bool {
        if k == &"?".repeat(k.len()) {
            return false;
        }
        for (a, b) in stamp.chars().zip(k.chars()) {
            if a != b && b != '?' {
                return false;
            }
        }
        return true;
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0936_example_1() {
        let stamp = "abc".to_string();
        let target = "ababc".to_string();
        let result = vec![0, 2];

        assert_eq!(Solution::moves_to_stamp(stamp, target), result);
    }

    #[test]
    #[ignore]
    fn test_0936_example_2() {
        let stamp = "abca".to_string();
        let target = "aabcaca".to_string();
        let result = vec![3, 0, 1];

        assert_eq!(Solution::moves_to_stamp(stamp, target), result);
    }
}

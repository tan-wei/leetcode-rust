/**
 * [0691] Stickers to Spell Word
 *
 * We are given n different types of stickers. Each sticker has a lowercase English word on it.
 * You would like to spell out the given string target by cutting individual letters from your collection of stickers and rearranging them. You can use each sticker more than once if you want, and you have infinite quantities of each sticker.
 * Return the minimum number of stickers that you need to spell out target. If the task is impossible, return -1.
 * Note: In all test cases, all words were chosen randomly from the 1000 most common US English words, and target was chosen as a concatenation of two random words.
 *  
 * Example 1:
 *
 * Input: stickers = ["with","example","science"], target = "thehat"
 * Output: 3
 * Explanation:
 * We can use 2 "with" stickers, and 1 "example" sticker.
 * After cutting and rearrange the letters of those stickers, we can form the target "thehat".
 * Also, this is the minimum number of stickers necessary to form the target string.
 *
 * Example 2:
 *
 * Input: stickers = ["notice","possible"], target = "basicbasic"
 * Output: -1
 * Explanation:
 * We cannot form the target "basicbasic" from cutting letters from the given stickers.
 *
 *  
 * Constraints:
 *
 * 	n == stickers.length
 * 	1 <= n <= 50
 * 	1 <= stickers[i].length <= 10
 * 	1 <= target.length <= 15
 * 	stickers[i] and target consist of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/stickers-to-spell-word/
// discuss: https://leetcode.com/problems/stickers-to-spell-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/stickers-to-spell-word/discuss/495587/Rust-124ms-2.1MB-100
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let n2 = 1 << n;
        let mut dp = vec![u32::max_value() as u32; n2];
        dp[0] = 0;
        for i in 0..n2 {
            if dp[i] == u32::max_value() {
                continue;
            }
            for s in &stickers {
                let mut now = i;
                for c in s.chars() {
                    for r in 0..n {
                        if target[r..].chars().next().unwrap() == c && !((now >> r) & 1 != 0) {
                            now |= 1 << r;
                            break;
                        }
                    }
                }
                dp[now] = std::cmp::min(dp[now], dp[i] + 1);
            }
        }
        dp[(n2 - 1) as usize] as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0691_example_1() {
        let stickers = vec_string!["with", "example", "science"];
        let target = "thehat".to_string();
        let result = 3;

        assert_eq!(Solution::min_stickers(stickers, target), result);
    }

    #[test]
    fn test_0691_example_2() {
        let stickers = vec_string!["notice", "possible"];
        let target = "basicbasic".to_string();
        let result = -1;

        assert_eq!(Solution::min_stickers(stickers, target), result);
    }
}

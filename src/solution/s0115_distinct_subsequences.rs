/**
 * [115] Distinct Subsequences
 *
 * Given two strings s and t, return the number of distinct subsequences of s which equals t.
 * A string's subsequence is a new string formed from the original string by deleting some (can be none) of the characters without disturbing the remaining characters' relative positions. (i.e., "ACE" is a subsequence of "ABCDE" while "AEC" is not).
 * It is guaranteed the answer fits on a 32-bit signed integer.
 *  
 * Example 1:
 *
 * Input: s = "rabbbit", t = "rabbit"
 * Output: 3
 * Explanation:
 * As shown below, there are 3 ways you can generate "rabbit" from S.
 * <u>rabb</u>b<u>it</u>
 * <u>ra</u>b<u>bbit</u>
 * <u>rab</u>b<u>bit</u>
 *
 * Example 2:
 *
 * Input: s = "babgbag", t = "bag"
 * Output: 5
 * Explanation:
 * As shown below, there are 5 ways you can generate "bag" from S.
 * <u>ba</u>b<u>g</u>bag
 * <u>ba</u>bgba<u>g</u>
 * <u>b</u>abgb<u>ag</u>
 * ba<u>b</u>gb<u>ag</u>
 * babg<u>bag</u>
 *  
 * Constraints:
 *
 * 	1 <= s.length, t.length <= 1000
 * 	s and t consist of English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-subsequences/
// discuss: https://leetcode.com/problems/distinct-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/distinct-subsequences/discuss/1041833/Rust-naive-recursion-with-memorization
    pub fn num_distinct(s: String, t: String) -> i32 {
        fn helper<'a>(
            s: &'a [u8],
            t: &'a [u8],
            memo: &mut std::collections::HashMap<(&'a [u8], &'a [u8]), i32>,
        ) -> i32 {
            if let Some(&i) = memo.get(&(s, t)) {
                return i;
            }
            if t.is_empty() {
                memo.insert((s, t), 1);
                return 1;
            }
            if s.is_empty() {
                memo.insert((s, t), 0);
                return 0;
            }
            let ans = if s[0] != t[0] {
                helper(&s[1..], t, memo)
            } else if s.len() <= 1 {
                helper(&s[1..], &t[1..], memo)
            } else {
                helper(&s[1..], &t[1..], memo) + helper(&s[1..], &t[0..], memo)
            };
            memo.insert((s, t), ans);
            ans
        }

        let ss = s.as_bytes();
        let ts = t.as_bytes();
        let mut memo = std::collections::HashMap::new();
        helper(&ss, &ts, &mut memo)
    }

    // Credit: https://leetcode.com/problems/distinct-subsequences/discuss/236467/Rust-Bottom-Up-DP-with-0-ms-and-O(N)-space
    pub fn num_distinct_v2(s: String, t: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut cache = vec![0; s.len()];
        for (i, ch) in t.chars().into_iter().enumerate() {
            let mut acc = 0;
            // first char initialization
            if i == 0 {
                for i in 0..s.len() {
                    if ch == s[i] {
                        cache[i] = 1
                    }
                }
                continue;
            }
            for i in 0..s.len() {
                let new_acc = acc + cache[i];
                cache[i] = if s[i] == ch { acc } else { 0 };
                acc = new_acc;
            }
        }
        cache.into_iter().fold(0, |acc, x| acc + x)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0115_example_1() {
        let s = "rabbbit".to_string();
        let t = "rabbit".to_string();
        let result = 3;

        assert_eq!(Solution::num_distinct(s, t), result);

        let s = "rabbbit".to_string();
        let t = "rabbit".to_string();
        let result = 3;

        assert_eq!(Solution::num_distinct_v2(s, t), result);
    }

    #[test]
    fn test_0115_example_2() {
        let s = "babgbag".to_string();
        let t = "bag".to_string();
        let result = 5;

        assert_eq!(Solution::num_distinct(s, t), result);

        let s = "babgbag".to_string();
        let t = "bag".to_string();
        let result = 5;

        assert_eq!(Solution::num_distinct_v2(s, t), result);
    }
}

/**
 * [76] Minimum Window Substring
 *
 * Given two strings s and t, return the minimum window in s which will contain all the characters in t. If there is no such window in s that covers all characters in t, return the empty string "".
 * Note that If there is such a window, it is guaranteed that there will always be only one unique minimum window in s.
 *  
 * Example 1:
 * Input: s = "ADOBECODEBANC", t = "ABC"
 * Output: "BANC"
 * Example 2:
 * Input: s = "a", t = "a"
 * Output: "a"
 *  
 * Constraints:
 *
 * 	1 <= s.length, t.length <= 10^5
 * 	s and t consist of English letters.
 *
 *  
 * Follow up: Could you find an algorithm that runs in O(n) time?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-window-substring/
// discuss: https://leetcode.com/problems/minimum-window-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-window-substring/discuss/549313/Rust-solution-for-reference
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() {
            return String::new();
        }

        let target = t.as_bytes().into_iter().fold(HashMap::new(), |mut a, e| {
            a.entry(e).and_modify(|v| *v += 1).or_insert(1);
            a
        });

        let mut qualify = t
            .as_bytes()
            .into_iter()
            .map(|&e| e)
            .collect::<HashSet<u8>>();

        let mut rec = HashMap::new();
        let s = s.as_bytes();
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut ret: &[u8] = &[];

        for (i, c) in s.into_iter().enumerate() {
            if !target.contains_key(c) {
                continue;
            }

            q.push_back(i);

            if *rec.entry(c).and_modify(|e| *e += 1).or_insert(1) >= *target.get(c).unwrap() {
                qualify.remove(c);
            }

            while !q.is_empty() {
                let bef_len = q.len();
                let qc = &s[*q.front().unwrap()];

                rec.entry(qc).and_modify(|e| {
                    if *e > *target.get(qc).unwrap() {
                        q.pop_front();
                        *e -= 1;
                    }
                });

                if bef_len == q.len() {
                    break;
                }
            }

            if qualify.is_empty() {
                if ret.len() == 0 || q.back().unwrap() - q.front().unwrap() + 1 < ret.len() {
                    ret = &s[*q.front().unwrap()..=*q.back().unwrap()];
                }
            }
        }
        String::from_utf8(ret.to_owned()).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0076_example_1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let result = "BANC".to_string();

        assert_eq!(Solution::min_window(s, t), result);
    }

    #[test]
    fn test_0076_example_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let result = "a".to_string();

        assert_eq!(Solution::min_window(s, t), result);
    }
}

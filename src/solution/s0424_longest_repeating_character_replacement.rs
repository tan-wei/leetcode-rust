/**
 * [0424] Longest Repeating Character Replacement
 *
 * You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.
 * Return the length of the longest substring containing the same letter you can get after performing the above operations.
 *  
 * Example 1:
 *
 * Input: s = "ABAB", k = 2
 * Output: 4
 * Explanation: Replace the two 'A's with two 'B's or vice versa.
 *
 * Example 2:
 *
 * Input: s = "AABABBA", k = 1
 * Output: 4
 * Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
 * The substring "BBBB" has the longest repeating letters, which is 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only uppercase English letters.
 * 	0 <= k <= s.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-repeating-character-replacement/
// discuss: https://leetcode.com/problems/longest-repeating-character-replacement/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/longest-repeating-character-replacement/discuss/925108/Rust-or-4ms-or-sliding-window-or-O(n)-runtime
struct Helper {
    max: Option<u8>,
    max_total: i32,
    total: i32,
    map: std::collections::HashMap<u8, i32>,
}

impl Helper {
    fn new() -> Self {
        Self {
            max: None,
            max_total: 0,
            total: 0,
            map: std::collections::HashMap::new(),
        }
    }
    fn inc(&mut self, key: u8) {
        let value = self.map.entry(key).or_default();
        *value += 1;
        self.total += 1;
        match self.max {
            None => self.find_max(),
            Some(m) if key != m => {
                if *value < self.max_total {
                    return;
                }
                self.find_max();
            }
            _ => self.max_total += 1,
        }
    }
    fn find_max(&mut self) {
        if self.max_total > self.total / 2 {
            return;
        }

        if let Some((max, max_total)) = self.map.iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(cur) => {
                if cur.1 < x.1 {
                    return Some(x);
                } else {
                    return Some(cur);
                }
            }
        }) {
            self.max = Some(*max);
            self.max_total = *max_total;
        }
    }
    fn dec(&mut self, key: u8) {
        self.map.entry(key).and_modify(|e| *e -= 1);
        self.total -= 1;
        match self.max {
            None => self.find_max(),
            Some(m) if m == key => {
                self.max_total -= 1;
                self.find_max();
            }
            _ => {}
        }
    }
    fn validate(&self, k: i32) -> bool {
        self.total - self.max_total <= k
    }
}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        match s.len() {
            0 | 1 => s.len() as i32,
            s_len => {
                let (mut start, mut end, s_bytes) = (0, 1, s.as_bytes());
                let mut max = 0;
                let mut helper = Helper::new();
                helper.inc(s_bytes[start]);
                loop {
                    helper.inc(s_bytes[end]);
                    while !helper.validate(k) {
                        helper.dec(s_bytes[start]);
                        start += 1;
                    }
                    if max < helper.total {
                        max = helper.total;
                    }
                    end += 1;
                    if end == s_len {
                        break max;
                    }
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0424_example_1() {
        let s = "ABAB".to_string();
        let k = 2;
        let result = 4;

        assert_eq!(Solution::character_replacement(s, k), result);
    }

    #[test]
    fn test_0424_example_2() {
        let s = "AABABBA".to_string();
        let k = 1;
        let result = 4;

        assert_eq!(Solution::character_replacement(s, k), result);
    }
}

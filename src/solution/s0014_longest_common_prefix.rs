/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 * If there is no common prefix, return an empty string "".
 *  
 * Example 1:
 *
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 * Example 2:
 *
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *  
 * Constraints:
 *
 * 	0 <= strs.length <= 200
 * 	0 <= strs[i].length <= 200
 * 	strs[i] consists of only lower-case English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-common-prefix/
// discuss: https://leetcode.com/problems/longest-common-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".into();
        }

        let min_len = strs.iter().map(|s| s.len()).min().unwrap();

        for i in 0..min_len {
            let ch = strs[0].chars().nth(i).unwrap();
            for s in &strs {
                if s.chars().nth(i).unwrap() != ch {
                    return s[0..i].into();
                }
            }
        }
        return strs[0][..min_len].into();
    }

    /// Credit: https://leetcode.com/problems/longest-common-prefix/discuss/224514/Iterators-based-Rust
    pub fn longest_common_prefix_v2(strs: Vec<String>) -> String {
        let mut strs = strs.iter();

        if let Some(head) = strs.next().cloned() {
            strs.fold(head, |head, tail| {
                head.chars()
                    .zip(tail.chars())
                    .take_while(|(l, r)| l == r)
                    .map(|(chr, _)| chr)
                    .collect()
            })
        } else {
            "".into()
        }
    }

    /// Credit: https://leetcode.com/problems/longest-common-prefix/discuss/763488/Rust%3A-0ms-solution
    pub fn longest_common_prefix_v3(strs: Vec<String>) -> String {
        match strs.is_empty() {
            true => "".to_string(),
            false => strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
                acc.chars()
                    .zip(x.chars())
                    .take_while(|(x, y)| x == y)
                    .map(|(x, _)| x)
                    .collect()
            }),
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0014_example_1() {
        let strs: Vec<String> = ["flower", "flow", "flight"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(
            Solution::longest_common_prefix(strs.clone()),
            "fl".to_string()
        );

        assert_eq!(
            Solution::longest_common_prefix_v2(strs.clone()),
            "fl".to_string()
        );

        assert_eq!(
            Solution::longest_common_prefix_v3(strs.clone()),
            "fl".to_string()
        );
    }

    #[test]
    fn test_0014_example_2() {
        let strs: Vec<String> = ["dog", "racecar", "car"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(
            Solution::longest_common_prefix(strs.clone()),
            "".to_string()
        );

        assert_eq!(
            Solution::longest_common_prefix_v2(strs.clone()),
            "".to_string()
        );

        assert_eq!(
            Solution::longest_common_prefix_v3(strs.clone()),
            "".to_string()
        );
    }
}

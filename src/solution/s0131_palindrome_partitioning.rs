/**
 * [131] Palindrome Partitioning
 *
 * Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.
 * A palindrome string is a string that reads the same backward as forward.
 *  
 * Example 1:
 * Input: s = "aab"
 * Output: [["a","a","b"],["aa","b"]]
 * Example 2:
 * Input: s = "a"
 * Output: [["a"]]
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 16
 * 	s contains only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning/
// discuss: https://leetcode.com/problems/palindrome-partitioning/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/palindrome-partitioning/discuss/932976/rust-DFS-simple-solution
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let s: Vec<char> = s.chars().collect();
        Self::helper(&s, &mut vec![], &mut res, 0);
        res
    }

    fn helper(s: &Vec<char>, cans: &mut Vec<String>, res: &mut Vec<Vec<String>>, idx: usize) {
        if idx == s.len() {
            res.push(cans.clone());
        }

        for i in idx..s.len() {
            if !Self::is_palindrome(s, idx, i) {
                continue;
            }
            let can: String = s[idx..i + 1].into_iter().collect();
            cans.push(can);
            Self::helper(s, cans, res, i + 1);
            cans.pop();
        }
    }

    fn is_palindrome(s: &Vec<char>, start: usize, end: usize) -> bool {
        let (mut start, mut end) = (start, end);
        while start < end {
            if s[start] != s[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0131_example_1() {
        let s = "aab".to_string();
        let result = vec![vec_string!["a", "a", "b"], vec_string!["aa", "b"]];

        assert_eq!(Solution::partition(s), result);
    }

    #[test]
    fn test_0131_example_2() {
        let s = "a".to_string();
        let result = vec![vec_string!["a"]];

        assert_eq!(Solution::partition(s), result);
    }
}

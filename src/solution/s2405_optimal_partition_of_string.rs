/**
 * [2405] Optimal Partition of String
 *
 * Given a string s, partition the string into one or more substrings such that the characters in each substring are unique. That is, no letter appears in a single substring more than once.
 * Return the minimum number of substrings in such a partition.
 * Note that each character should belong to exactly one substring in a partition.
 *  
 * Example 1:
 *
 * Input: s = "abacaba"
 * Output: 4
 * Explanation:
 * Two possible partitions are ("a","ba","cab","a") and ("ab","a","ca","ba").
 * It can be shown that 4 is the minimum number of substrings needed.
 *
 * Example 2:
 *
 * Input: s = "ssssss"
 * Output: 6
 * Explanation:
 * The only valid partition is ("s","s","s","s","s","s").
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s consists of only English lowercase letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/optimal-partition-of-string/
// discuss: https://leetcode.com/problems/optimal-partition-of-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2405_example_1() {
        let s = "abacaba".to_string();

        let result = 4;

        assert_eq!(Solution::partition_string(s), result);
    }

    #[test]
    #[ignore]
    fn test_2405_example_2() {
        let s = "ssssss".to_string();

        let result = 6;

        assert_eq!(Solution::partition_string(s), result);
    }
}

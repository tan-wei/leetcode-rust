/**
 * [1278] Palindrome Partitioning III
 *
 * You are given a string s containing lowercase letters and an integer k. You need to :
 *
 * 	First, change some characters of s to other lowercase English letters.
 * 	Then divide s into k non-empty disjoint substrings such that each substring is a palindrome.
 *
 * Return the minimal number of characters that you need to change to divide the string.
 *  
 * Example 1:
 *
 * Input: s = "abc", k = 2
 * Output: 1
 * Explanation: You can split the string into "ab" and "c", and change 1 character in "ab" to make it palindrome.
 *
 * Example 2:
 *
 * Input: s = "aabbc", k = 3
 * Output: 0
 * Explanation: You can split the string into "aa", "bb" and "c", all of them are palindrome.
 * Example 3:
 *
 * Input: s = "leetcode", k = 8
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= s.length <= 100.
 * 	s only contains lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning-iii/
// discuss: https://leetcode.com/problems/palindrome-partitioning-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/palindrome-partitioning-iii/solutions/2931463/rust-simple-solution-recursion-memorization/
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let c: Vec<char> = s.chars().collect();

        let mut memo: std::collections::HashMap<(i32, usize), i32> =
            std::collections::HashMap::new();
        Self::dfs_helper(k, c.len(), &c, &mut memo)
    }

    fn dfs_helper(
        k: i32,
        l: usize,
        c: &Vec<char>,
        memo: &mut std::collections::HashMap<(i32, usize), i32>,
    ) -> i32 {
        if k as usize >= l {
            return 0;
        }

        if k == 0 && l > 0 {
            return l as i32;
        }

        if let Some(r) = memo.get(&(k, l)) {
            return *r;
        }

        let mut result = l as i32;

        for ll in ((k - 1) as usize..l) {
            let mut ri = l - 1;
            let mut li = ll;
            let mut lc = 0; //steps to replace the right part to palindrome
            while li < ri {
                if c[li] != c[ri] {
                    lc += 1;
                }
                li += 1;
                ri -= 1;
            }
            result = result.min(Self::dfs_helper(k - 1, ll, c, memo) + lc);
        }

        memo.insert((k, l), result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1278_example_1() {
        let s = "abc".to_string();
        let k = 2;
        let result = 1;

        assert_eq!(Solution::palindrome_partition(s, k), result);
    }

    #[test]
    fn test_1278_example_2() {
        let s = "aabbc".to_string();
        let k = 3;
        let result = 0;

        assert_eq!(Solution::palindrome_partition(s, k), result);
    }

    #[test]
    fn test_1278_example_3() {
        let s = "leetcode".to_string();
        let k = 8;
        let result = 0;

        assert_eq!(Solution::palindrome_partition(s, k), result);
    }
}

/**
 * [2399] Check Distances Between Same Letters
 *
 * You are given a 0-indexed string s consisting of only lowercase English letters, where each letter in s appears exactly twice. You are also given a 0-indexed integer array distance of length 26.
 * Each letter in the alphabet is numbered from 0 to 25 (i.e. 'a' -> 0, 'b' -> 1, 'c' -> 2, ... , 'z' -> 25).
 * In a well-spaced string, the number of letters between the two occurrences of the i^th letter is distance[i]. If the i^th letter does not appear in s, then distance[i] can be ignored.
 * Return true if s is a well-spaced string, otherwise return false.
 *  
 * Example 1:
 *
 * Input: s = "abaccb", distance = [1,3,0,5,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
 * Output: true
 * Explanation:
 * - 'a' appears at indices 0 and 2 so it satisfies distance[0] = 1.
 * - 'b' appears at indices 1 and 5 so it satisfies distance[1] = 3.
 * - 'c' appears at indices 3 and 4 so it satisfies distance[2] = 0.
 * Note that distance[3] = 5, but since 'd' does not appear in s, it can be ignored.
 * Return true because s is a well-spaced string.
 *
 * Example 2:
 *
 * Input: s = "aa", distance = [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
 * Output: false
 * Explanation:
 * - 'a' appears at indices 0 and 1 so there are zero letters between them.
 * Because distance[0] = 1, s is not a well-spaced string.
 *
 *  
 * Constraints:
 *
 * 	2 <= s.length <= 52
 * 	s consists only of lowercase English letters.
 * 	Each letter appears in s exactly twice.
 * 	distance.length == 26
 * 	0 <= distance[i] <= 50
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-distances-between-same-letters/
// discuss: https://leetcode.com/problems/check-distances-between-same-letters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        s.bytes()
            .map(|b| distance[(b - b'a') as usize])
            .enumerate()
            .all(|(i, b)| {
                if i + (b as usize) + 1 < s.len()
                    && s.as_bytes()[i] == s.as_bytes()[i + (b as usize) + 1]
                {
                    true
                } else if i >= (b as usize) + 1
                    && s.as_bytes()[i] == s.as_bytes()[i - (b as usize) - 1]
                {
                    true
                } else {
                    false
                }
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2399_example_1() {
        let s = "abaccb".to_string();
        let distance = vec![
            1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let result = true;

        assert_eq!(Solution::check_distances(s, distance), result);
    }

    #[test]
    fn test_2399_example_2() {
        let s = "aa".to_string();
        let distance = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let result = false;

        assert_eq!(Solution::check_distances(s, distance), result);
    }
}

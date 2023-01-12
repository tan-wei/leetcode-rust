/**
 * [0942] DI String Match
 *
 * A permutation perm of n + 1 integers of all the integers in the range [0, n] can be represented as a string s of length n where:
 *
 * 	s[i] == 'I' if perm[i] < perm[i + 1], and
 * 	s[i] == 'D' if perm[i] > perm[i + 1].
 *
 * Given a string s, reconstruct the permutation perm and return it. If there are multiple valid permutations perm, return any of them.
 *  
 * Example 1:
 * Input: s = "IDID"
 * Output: [0,4,1,3,2]
 * Example 2:
 * Input: s = "III"
 * Output: [0,1,2,3]
 * Example 3:
 * Input: s = "DDI"
 * Output: [3,2,0,1]
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	s[i] is either 'I' or 'D'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/di-string-match/
// discuss: https://leetcode.com/problems/di-string-match/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut min = 0;
        let mut max = 0;
        let mut result = vec![0];

        for c in s.chars() {
            match c {
                'I' => {
                    max += 1;
                    result.push(max);
                }
                'D' => {
                    min -= 1;
                    result.push(min);
                }
                _ => panic!(),
            }
        }

        result.iter().map(|x| x - min).collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0942_example_1() {
        let s = "IDID".to_string();
        let result = vec![0, 4, 1, 3, 2];

        assert_eq!(Solution::di_string_match(s), result);
    }

    #[test]
    #[ignore]
    fn test_0942_example_2() {
        let s = "III".to_string();
        let result = vec![0, 1, 2, 3];

        assert_eq!(Solution::di_string_match(s), result);
    }

    #[test]
    #[ignore]
    fn test_0942_example_3() {
        let s = "DDI".to_string();
        let result = vec![3, 2, 0, 1];

        assert_eq!(Solution::di_string_match(s), result);
    }
}

/**
 * [1781] Sum of Beauty of All Substrings
 *
 * The beauty of a string is the difference in frequencies between the most frequent and least frequent characters.
 *
 * 	For example, the beauty of "abaacc" is 3 - 1 = 2.
 *
 * Given a string s, return the sum of beauty of all of its substrings.
 *  
 * Example 1:
 *
 * Input: s = "aabcb"
 * Output: 5
 * Explanation: The substrings with non-zero beauty are ["aab","aabc","aabcb","abcb","bcb"], each with beauty equal to 1.
 * Example 2:
 *
 * Input: s = "aabcbaa"
 * Output: 17
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <=^ 500
 * 	s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-beauty-of-all-substrings/
// discuss: https://leetcode.com/problems/sum-of-beauty-of-all-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut memo = vec![vec![0; 26]; n + 1];

        for i in 0..n {
            memo[i + 1] = memo[i].clone();
            memo[i + 1][s[i] as usize - 'a' as usize] += 1;
        }

        let mut result = 0;
        for i in 0..n {
            for j in i..n {
                let mut max = 0;
                let mut min = n + 10;

                for k in 0..26 {
                    let v = memo[j + 1][k] - memo[i][k];
                    max = max.max(v);
                    if v != 0 {
                        min = min.min(v);
                    }
                }
                result += max - min;
            }
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1781_example_1() {
        let s = "aabcb".to_string();

        let result = 5;

        assert_eq!(Solution::beauty_sum(s), result);
    }

    #[test]
    fn test_1781_example_2() {
        let s = "aabcbaa".to_string();

        let result = 17;

        assert_eq!(Solution::beauty_sum(s), result);
    }
}

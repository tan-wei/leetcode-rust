/**
 * [0821] Shortest Distance to a Character
 *
 * Given a string s and a character c that occurs in s, return an array of integers answer where answer.length == s.length and answer[i] is the distance from index i to the closest occurrence of character c in s.
 * The distance between two indices i and j is abs(i - j), where abs is the absolute value function.
 *  
 * Example 1:
 *
 * Input: s = "loveleetcode", c = "e"
 * Output: [3,2,1,0,1,0,0,1,2,2,1,0]
 * Explanation: The character 'e' appears at indices 3, 5, 6, and 11 (0-indexed).
 * The closest occurrence of 'e' for index 0 is at index 3, so the distance is abs(0 - 3) = 3.
 * The closest occurrence of 'e' for index 1 is at index 3, so the distance is abs(1 - 3) = 2.
 * For index 4, there is a tie between the 'e' at index 3 and the 'e' at index 5, but the distance is still the same: abs(4 - 3) == abs(4 - 5) = 1.
 * The closest occurrence of 'e' for index 8 is at index 6, so the distance is abs(8 - 6) = 2.
 *
 * Example 2:
 *
 * Input: s = "aaab", c = "b"
 * Output: [3,2,1,0]
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^4
 * 	s[i] and c are lowercase English letters.
 * 	It is guaranteed that c occurs at least once in s.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-distance-to-a-character/
// discuss: https://leetcode.com/problems/shortest-distance-to-a-character/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        (0..s.len())
            .map(|i| {
                let left = s[0..i + 1]
                    .chars()
                    .rev()
                    .position(|k| k == c)
                    .unwrap_or(usize::MAX);
                let right = s[i..].find(c).unwrap_or(usize::MAX);
                (if left < right { left } else { right }) as i32
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0821_example_1() {
        let s = "loveleetcode".to_string();
        let c = 'e';
        let result = vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0];

        assert_eq!(Solution::shortest_to_char(s, c), result);
    }

    #[test]
    fn test_0821_example_2() {
        let s = "aaab".to_string();
        let c = 'b';
        let result = vec![3, 2, 1, 0];

        assert_eq!(Solution::shortest_to_char(s, c), result);
    }
}

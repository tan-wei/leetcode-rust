/**
 * [1717] Maximum Score From Removing Substrings
 *
 * You are given a string s and two integers x and y. You can perform two types of operations any number of times.
 *
 * 	Remove substring "ab" and gain x points.
 *
 * 		For example, when removing "ab" from "c<u>ab</u>xbae" it becomes "cxbae".
 *
 *
 * 	Remove substring "ba" and gain y points.
 *
 * 		For example, when removing "ba" from "cabx<u>ba</u>e" it becomes "cabxe".
 *
 *
 *
 * Return the maximum points you can gain after applying the above operations on s.
 *  
 * Example 1:
 *
 * Input: s = "cdbcbbaaabab", x = 4, y = 5
 * Output: 19
 * Explanation:
 * - Remove the "ba" underlined in "cdbcbbaaa<u>ba</u>b". Now, s = "cdbcbbaaab" and 5 points are added to the score.
 * - Remove the "ab" underlined in "cdbcbbaa<u>ab</u>". Now, s = "cdbcbbaa" and 4 points are added to the score.
 * - Remove the "ba" underlined in "cdbcb<u>ba</u>a". Now, s = "cdbcba" and 5 points are added to the score.
 * - Remove the "ba" underlined in "cdbc<u>ba</u>". Now, s = "cdbc" and 5 points are added to the score.
 * Total score = 5 + 4 + 5 + 5 = 19.
 * Example 2:
 *
 * Input: s = "aabbaaxybbaabb", x = 5, y = 4
 * Output: 20
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 10^5
 * 	1 <= x, y <= 10^4
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-score-from-removing-substrings/
// discuss: https://leetcode.com/problems/maximum-score-from-removing-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-score-from-removing-substrings/solutions/5465387/rust-100-greedy-const-generics/
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        if x > y {
            Self::gain::<b'a', b'b'>(x, y, s.as_ref())
        } else {
            Self::gain::<b'b', b'a'>(y, x, s.as_ref())
        }
    }

    fn gain<const A: u8, const B: u8>(x: i32, y: i32, s: &str) -> i32 {
        let (mut cb, mut ca, mut result) = (0, 0, 0);
        for c in s.bytes() {
            if c == B && ca > 0 {
                ca -= 1;
                result += x;
            } else if c == B {
                cb += 1;
            } else if c == A {
                ca += 1;
            } else {
                result += y * ca.min(cb);
                (ca, cb) = (0, 0);
            }
        }
        result + y * ca.min(cb)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1717_example_1() {
        let s = "cdbcbbaaabab".to_string();
        let x = 4;
        let y = 5;

        let result = 19;

        assert_eq!(Solution::maximum_gain(s, x, y), result);
    }

    #[test]
    fn test_1717_example_2() {
        let s = "aabbaaxybbaabb".to_string();
        let x = 5;
        let y = 4;

        let result = 20;

        assert_eq!(Solution::maximum_gain(s, x, y), result);
    }
}

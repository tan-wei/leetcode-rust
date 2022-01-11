/**
 * [0481] Magical String
 *
 * A magical string s consists of only '1' and '2' and obeys the following rules:
 *
 * 	The string s is magical because concatenating the number of contiguous occurrences of characters '1' and '2' generates the string s itself.
 *
 * The first few elements of s is s = "1221121221221121122&hellip;&hellip;". If we group the consecutive 1's and 2's in s, it will be "1 22 11 2 1 22 1 22 11 2 11 22 ......" and the occurrences of 1's or 2's in each group are "1 2 2 1 1 2 1 2 2 1 2 2 ......". You can see that the occurrence sequence is s itself.
 * Given an integer n, return the number of 1's in the first n number in the magical string s.
 *  
 * Example 1:
 *
 * Input: n = 6
 * Output: 3
 * Explanation: The first 6 elements of magical string s is "122112" and it contains three 1's, so return 3.
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/magical-string/
// discuss: https://leetcode.com/problems/magical-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        let mut s = vec!['1', '2', '2'];
        let mut i = 2;
        while s.len() < (n as usize) {
            let append = if *s.last().unwrap() == '1' { '2' } else { '1' };

            match s[i] {
                '1' => s.push(append),
                '2' => {
                    s.push(append);
                    s.push(append);
                }
                _ => panic!(),
            };

            i += 1;
        }
        s.into_iter().take(n as usize).filter(|x| *x == '1').count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0481_example_1() {
        let n = 6;
        let result = 3;

        assert_eq!(Solution::magical_string(n), result);
    }

    #[test]
    fn test_0481_example_2() {
        let n = 1;
        let result = 1;

        assert_eq!(Solution::magical_string(n), result);
    }
}

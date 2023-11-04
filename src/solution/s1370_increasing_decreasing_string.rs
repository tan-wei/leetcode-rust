/**
 * [1370] Increasing Decreasing String
 *
 * You are given a string s. Reorder the string using the following algorithm:
 * <ol>
 * 	Pick the smallest character from s and append it to the result.
 * 	Pick the smallest character from s which is greater than the last appended character to the result and append it.
 * 	Repeat step 2 until you cannot pick more characters.
 * 	Pick the largest character from s and append it to the result.
 * 	Pick the largest character from s which is smaller than the last appended character to the result and append it.
 * 	Repeat step 5 until you cannot pick more characters.
 * 	Repeat the steps from 1 to 6 until you pick all characters from s.
 * </ol>
 * In each step, If the smallest or the largest character appears more than once you can choose any occurrence and append it to the result.
 * Return the result string after sorting s with this algorithm.
 *  
 * Example 1:
 *
 * Input: s = "aaaabbbbcccc"
 * Output: "abccbaabccba"
 * Explanation: After steps 1, 2 and 3 of the first iteration, result = "abc"
 * After steps 4, 5 and 6 of the first iteration, result = "abccba"
 * First iteration is done. Now s = "aabbcc" and we go back to step 1
 * After steps 1, 2 and 3 of the second iteration, result = "abccbaabc"
 * After steps 4, 5 and 6 of the second iteration, result = "abccbaabccba"
 *
 * Example 2:
 *
 * Input: s = "rat"
 * Output: "art"
 * Explanation: The word "rat" becomes "art" after re-ordering it with the mentioned algorithm.
 *
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 500
 * 	s consists of only lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/increasing-decreasing-string/
// discuss: https://leetcode.com/problems/increasing-decreasing-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut result = String::with_capacity(s.len());
        let mut counter = [0; 26];
        let (mut incr, mut ind) = (1, 0);

        s.as_bytes()
            .iter()
            .for_each(|b| counter[(*b - b'a') as usize] += 1);

        while result.len() < s.len() {
            if counter[ind as usize] > 0 {
                result.push((b'a' + ind as u8) as char);
                counter[ind as usize] -= 1;
            };

            ind += incr;
            if ind < 0 || ind > 25 {
                ind = if ind < 0 { 0 } else { 25 }; // ind.clamp(0, 25) doesn't work in this old version of Rust
                incr *= -1;
            };
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1370_example_1() {
        let s = "aaaabbbbcccc".to_string();

        let result = "abccbaabccba".to_string();

        assert_eq!(Solution::sort_string(s), result);
    }

    #[test]
    fn test_1370_example_2() {
        let s = "rat".to_string();

        let result = "art".to_string();

        assert_eq!(Solution::sort_string(s), result);
    }
}

/**
 * [1945] Sum of Digits of String After Convert
 *
 * You are given a string s consisting of lowercase English letters, and an integer k. Your task is to convert the string into an integer by a special process, and then transform it by summing its digits repeatedly k times. More specifically, perform the following steps:
 * <ol>
 * 	Convert s into an integer by replacing each letter with its position in the alphabet (i.e. replace 'a' with 1, 'b' with 2, ..., 'z' with 26).
 * 	Transform the integer by replacing it with the sum of its digits.
 * 	Repeat the transform operation (step 2) k times in total.
 * </ol>
 * For example, if s = "zbax" and k = 2, then the resulting integer would be 8 by the following operations:
 * <ol>
 * 	Convert: "zbax" ➝ "(26)(2)(1)(24)" ➝ "262124" ➝ 262124
 * 	Transform #1: 262124 ➝ 2 + 6 + 2 + 1 + 2 + 4 ➝ 17
 * 	Transform #2: 17 ➝ 1 + 7 ➝ 8
 * </ol>
 * Return the resulting integer after performing the operations described above.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">s = "iiii", k = 1</span>
 * Output: <span class="example-io">36</span>
 * Explanation:
 * The operations are as follows:<br />
 * - Convert: "iiii" ➝ "(9)(9)(9)(9)" ➝ "9999" ➝ 9999<br />
 * - Transform #1: 9999 ➝ 9 + 9 + 9 + 9 ➝ 36<br />
 * Thus the resulting integer is 36.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">s = "leetcode", k = 2</span>
 * Output: <span class="example-io">6</span>
 * Explanation:
 * The operations are as follows:<br />
 * - Convert: "leetcode" ➝ "(12)(5)(5)(20)(3)(15)(4)(5)" ➝ "12552031545" ➝ 12552031545<br />
 * - Transform #1: 12552031545 ➝ 1 + 2 + 5 + 5 + 2 + 0 + 3 + 1 + 5 + 4 + 5 ➝ 33<br />
 * - Transform #2: 33 ➝ 3 + 3 ➝ 6<br />
 * Thus the resulting integer is 6.
 * </div>
 * Example 3:
 * <div class="example-block">
 * Input: <span class="example-io">s = "zbax", k = 2</span>
 * Output: <span class="example-io">8</span>
 * </div>
 *  
 * Constraints:
 *
 * 	1 <= s.length <= 100
 * 	1 <= k <= 10
 * 	s consists of lowercase English letters.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-digits-of-string-after-convert/
// discuss: https://leetcode.com/problems/sum-of-digits-of-string-after-convert/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let mut s = s
            .chars()
            .map(|c| (c as u8 - b'a' + 1).to_string())
            .collect::<Vec<String>>()
            .join("");

        for _ in 0..k {
            s = s
                .chars()
                .map(|c| c as i32 - '0' as i32)
                .sum::<i32>()
                .to_string();
        }

        s.parse::<i32>().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1945_example_1() {
        let s = "iiii".to_string();
        let k = 1;

        let result = 36;

        assert_eq!(Solution::get_lucky(s, k), result);
    }

    #[test]
    fn test_1945_example_2() {
        let s = "leetcode".to_string();
        let k = 2;

        let result = 6;

        assert_eq!(Solution::get_lucky(s, k), result);
    }

    #[test]
    fn test_1945_example_3() {
        let s = "zbax".to_string();
        let k = 2;

        let result = 8;

        assert_eq!(Solution::get_lucky(s, k), result);
    }
}

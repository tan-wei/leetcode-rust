/**
 * [2024] Maximize the Confusion of an Exam
 *
 * A teacher is writing a test with n true/false questions, with 'T' denoting true and 'F' denoting false. He wants to confuse the students by maximizing the number of consecutive questions with the same answer (multiple trues or multiple falses in a row).
 * You are given a string answerKey, where answerKey[i] is the original answer to the i^th question. In addition, you are given an integer k, the maximum number of times you may perform the following operation:
 *
 * 	Change the answer key for any question to 'T' or 'F' (i.e., set answerKey[i] to 'T' or 'F').
 *
 * Return the maximum number of consecutive 'T's or 'F's in the answer key after performing the operation at most k times.
 *  
 * Example 1:
 *
 * Input: answerKey = "TTFF", k = 2
 * Output: 4
 * Explanation: We can replace both the 'F's with 'T's to make answerKey = "<u>TTTT</u>".
 * There are four consecutive 'T's.
 *
 * Example 2:
 *
 * Input: answerKey = "TFFT", k = 1
 * Output: 3
 * Explanation: We can replace the first 'T' with an 'F' to make answerKey = "<u>FFF</u>T".
 * Alternatively, we can replace the second 'T' with an 'F' to make answerKey = "T<u>FFF</u>".
 * In both cases, there are three consecutive 'F's.
 *
 * Example 3:
 *
 * Input: answerKey = "TTFTTFTT", k = 1
 * Output: 5
 * Explanation: We can replace the first 'F' to make answerKey = "<u>TTTTT</u>FTT"
 * Alternatively, we can replace the second 'F' to make answerKey = "TTF<u>TTTTT</u>".
 * In both cases, there are five consecutive 'T's.
 *
 *  
 * Constraints:
 *
 * 	n == answerKey.length
 * 	1 <= n <= 5 * 10^4
 * 	answerKey[i] is either 'T' or 'F'
 * 	1 <= k <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-the-confusion-of-an-exam/
// discuss: https://leetcode.com/problems/maximize-the-confusion-of-an-exam/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximize-the-confusion-of-an-exam/solutions/3729580/optimal-rust-solution/
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let (mut trues, mut falses) = (0i32, 0i32);
        let answer_key = answer_key.into_bytes();

        let (mut left, mut right) = (0, 0);
        let mut result = 0;

        while right < answer_key.len() {
            if answer_key[right] == b'T' {
                trues += 1
            } else {
                falses += 1
            }

            while left < right && i32::min(falses, trues) > k {
                if answer_key[left] == b'T' {
                    trues -= 1
                } else {
                    falses -= 1
                }
                left += 1;
            }

            result = usize::max(result, right - left + 1);
            right += 1;
        }

        result as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2024_example_1() {
        let answer_key = "TTFF".to_string();
        let k = 2;

        let result = 4;

        assert_eq!(Solution::max_consecutive_answers(answer_key, k), result);
    }

    #[test]
    fn test_2024_example_2() {
        let answer_key = "TFFT".to_string();
        let k = 1;

        let result = 3;

        assert_eq!(Solution::max_consecutive_answers(answer_key, k), result);
    }

    #[test]
    fn test_2024_example_3() {
        let answer_key = "TTFTTFTT".to_string();
        let k = 1;

        let result = 5;

        assert_eq!(Solution::max_consecutive_answers(answer_key, k), result);
    }
}

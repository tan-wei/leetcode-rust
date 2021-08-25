/**
 * [299] Bulls and Cows
 *
 * You are playing the <a href="https://en.wikipedia.org/wiki/Bulls_and_Cows" target="_blank">Bulls and Cows</a> game with your friend.
 * You write down a secret number and ask your friend to guess what the number is. When your friend makes a guess, you provide a hint with the following info:
 *
 * 	The number of "bulls", which are digits in the guess that are in the correct position.
 * 	The number of "cows", which are digits in the guess that are in your secret number but are located in the wrong position. Specifically, the non-bull digits in the guess that could be rearranged such that they become bulls.
 *
 * Given the secret number secret and your friend's guess guess, return the hint for your friend's guess.
 * The hint should be formatted as "xAyB", where x is the number of bulls and y is the number of cows. Note that both secret and guess may contain duplicate digits.
 *  
 * Example 1:
 *
 * Input: secret = "1807", guess = "7810"
 * Output: "1A3B"
 * Explanation: Bulls are connected with a '|' and cows are underlined:
 * "1807"
 *   |
 * "<u>7</u>8<u>10</u>"
 * Example 2:
 *
 * Input: secret = "1123", guess = "0111"
 * Output: "1A1B"
 * Explanation: Bulls are connected with a '|' and cows are underlined:
 * "1123"        "1123"
 *   |      or     |
 * "01<u>1</u>1"        "011<u>1</u>"
 * Note that only one of the two unmatched 1s is counted as a cow since the non-bull digits can only be rearranged to allow one 1 to be a bull.
 *
 * Example 3:
 *
 * Input: secret = "1", guess = "0"
 * Output: "0A0B"
 *
 * Example 4:
 *
 * Input: secret = "1", guess = "1"
 * Output: "1A0B"
 *
 *  
 * Constraints:
 *
 * 	1 <= secret.length, guess.length <= 1000
 * 	secret.length == guess.length
 * 	secret and guess consist of digits only.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/bulls-and-cows/
// discuss: https://leetcode.com/problems/bulls-and-cows/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut a = 0;
        let mut b = 0;
        let mut nums = [0; 10];
        for (ch1, ch2) in secret.chars().zip(guess.chars()) {
            if ch1 == ch2 {
                a += 1;
            } else {
                let i = ch1 as usize - '0' as usize;
                let j = ch2 as usize - '0' as usize;
                if nums[i] < 0 {
                    b += 1;
                }
                if nums[j] > 0 {
                    b += 1;
                }
                nums[i] += 1;
                nums[j] -= 1;
            }
        }
        format!("{}A{}B", a, b)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0299_example_1() {
        let secret = "1807".to_string();
        let guess = "7810".to_string();
        let result = "1A3B".to_string();

        assert_eq!(Solution::get_hint(secret, guess), result);
    }

    #[test]
    fn test_0299_example_2() {
        let secret = "1123".to_string();
        let guess = "0111".to_string();
        let result = "1A1B".to_string();

        assert_eq!(Solution::get_hint(secret, guess), result);
    }

    #[test]
    fn test_0299_example_3() {
        let secret = "1".to_string();
        let guess = "0".to_string();
        let result = "0A0B".to_string();

        assert_eq!(Solution::get_hint(secret, guess), result);
    }

    #[test]
    fn test_0299_example_4() {
        let secret = "1".to_string();
        let guess = "1".to_string();
        let result = "1A0B".to_string();

        assert_eq!(Solution::get_hint(secret, guess), result);
    }
}

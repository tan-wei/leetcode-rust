/**
 * [1736] Latest Time by Replacing Hidden Digits
 *
 * You are given a string time in the form of  hh:mm, where some of the digits in the string are hidden (represented by ?).
 * The valid times are those inclusively between 00:00 and 23:59.
 * Return the latest valid time you can get from time by replacing the hidden digits.
 *  
 * Example 1:
 *
 * Input: time = "2?:?0"
 * Output: "23:50"
 * Explanation: The latest hour beginning with the digit '2' is 23 and the latest minute ending with the digit '0' is 50.
 *
 * Example 2:
 *
 * Input: time = "0?:3?"
 * Output: "09:39"
 *
 * Example 3:
 *
 * Input: time = "1?:22"
 * Output: "19:22"
 *
 *  
 * Constraints:
 *
 * 	time is in the format hh:mm.
 * 	It is guaranteed that you can produce a valid time from the given string.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/
// discuss: https://leetcode.com/problems/latest-time-by-replacing-hidden-digits/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut time = time.chars().collect::<Vec<_>>();

        if time[0] == '?' {
            time[0] = if time[1] <= '3' || time[1] == '?' {
                '2'
            } else {
                '1'
            };
        }

        if time[1] == '?' {
            time[1] = if time[0] == '2' { '3' } else { '9' };
        }

        if time[3] == '?' {
            time[3] = '5';
        }
        if time[4] == '?' {
            time[4] = '9';
        }

        time.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1736_example_1() {
        let time = "2?:?0".to_string();

        let result = "23:50".to_string();

        assert_eq!(Solution::maximum_time(time), result);
    }

    #[test]
    fn test_1736_example_2() {
        let time = "0?:3?".to_string();

        let result = "09:39".to_string();

        assert_eq!(Solution::maximum_time(time), result);
    }

    #[test]
    fn test_1736_example_3() {
        let time = "1?:22".to_string();

        let result = "19:22".to_string();

        assert_eq!(Solution::maximum_time(time), result);
    }
}

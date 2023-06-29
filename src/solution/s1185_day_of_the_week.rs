/**
 * [1185] Day of the Week
 *
 * Given a date, return the corresponding day of the week for that date.
 * The input is given as three integers representing the day, month and year respectively.
 * Return the answer as one of the following values {"Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"}.
 *  
 * Example 1:
 *
 * Input: day = 31, month = 8, year = 2019
 * Output: "Saturday"
 *
 * Example 2:
 *
 * Input: day = 18, month = 7, year = 1999
 * Output: "Sunday"
 *
 * Example 3:
 *
 * Input: day = 15, month = 8, year = 1993
 * Output: "Sunday"
 *
 *  
 * Constraints:
 *
 * 	The given dates are valid dates between the years 1971 and 2100.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/day-of-the-week/
// discuss: https://leetcode.com/problems/day-of-the-week/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let weekdays = [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday",
        ];
        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let is_leap_year = |x: i32| -> bool { x % 4 == 0 && (x % 100 != 0 || x % 400 == 0) };

        let num_days = (1971..year)
            .map(|y| 365 + is_leap_year(y) as i32)
            .sum::<i32>()
            + (0..month - 1)
                .map(|m| months[m as usize] + (m == 1 && is_leap_year(year)) as i32)
                .sum::<i32>()
            + day;

        // 01/01/1971 - Friday
        let res_day = (4 + (num_days - 1)) % 7;
        weekdays[res_day as usize].to_string()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1185_example_1() {
        let day = 31;
        let month = 8;
        let year = 2019;
        let result = "Saturday".to_string();

        assert_eq!(Solution::day_of_the_week(day, month, year), result);
    }

    #[test]
    fn test_1185_example_2() {
        let day = 18;
        let month = 7;
        let year = 1999;
        let result = "Sunday".to_string();

        assert_eq!(Solution::day_of_the_week(day, month, year), result);
    }

    #[test]
    fn test_1185_example_3() {
        let day = 15;
        let month = 8;
        let year = 1993;
        let result = "Sunday".to_string();

        assert_eq!(Solution::day_of_the_week(day, month, year), result);
    }
}

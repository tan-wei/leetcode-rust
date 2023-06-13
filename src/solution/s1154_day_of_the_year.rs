/**
 * [1154] Day of the Year
 *
 * Given a string date representing a <a href="https://en.wikipedia.org/wiki/Gregorian_calendar" target="_blank">Gregorian calendar</a> date formatted as YYYY-MM-DD, return the day number of the year.
 *  
 * Example 1:
 *
 * Input: date = "2019-01-09"
 * Output: 9
 * Explanation: Given date is the 9th day of the year in 2019.
 *
 * Example 2:
 *
 * Input: date = "2019-02-10"
 * Output: 41
 *
 *  
 * Constraints:
 *
 * 	date.length == 10
 * 	date[4] == date[7] == '-', and all other date[i]'s are digits
 * 	date represents a calendar date between Jan 1^st, 1900 and Dec 31^th, 2019.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/day-of-the-year/
// discuss: https://leetcode.com/problems/day-of-the-year/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let date_parts = date
            .split('-')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let (year, month, day) = (date_parts[0], date_parts[1], date_parts[2]);
        let leap_year_day = month > 2 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let days_in_months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        days_in_months[..month as usize - 1].iter().sum::<i32>() + day + leap_year_day as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1154_example_1() {
        let date = "2019-01-09".to_string();
        let result = 9;

        assert_eq!(Solution::day_of_year(date), result);
    }

    #[test]
    fn test_1154_example_2() {
        let date = "2019-02-10".to_string();
        let result = 41;

        assert_eq!(Solution::day_of_year(date), result);
    }
}

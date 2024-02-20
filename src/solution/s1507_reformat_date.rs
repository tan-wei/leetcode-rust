/**
 * [1507] Reformat Date
 *
 * Given a date string in the form Day Month Year, where:
 *
 * 	Day is in the set {"1st", "2nd", "3rd", "4th", ..., "30th", "31st"}.
 * 	Month is in the set {"Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"}.
 * 	Year is in the range [1900, 2100].
 *
 * Convert the date string to the format YYYY-MM-DD, where:
 *
 * 	YYYY denotes the 4 digit year.
 * 	MM denotes the 2 digit month.
 * 	DD denotes the 2 digit day.
 *
 *  
 * Example 1:
 *
 * Input: date = "20th Oct 2052"
 * Output: "2052-10-20"
 *
 * Example 2:
 *
 * Input: date = "6th Jun 1933"
 * Output: "1933-06-06"
 *
 * Example 3:
 *
 * Input: date = "26th May 1960"
 * Output: "1960-05-26"
 *
 *  
 * Constraints:
 *
 * 	The given dates are guaranteed to be valid, so no error handling is necessary.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reformat-date/
// discuss: https://leetcode.com/problems/reformat-date/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let months: [&str; 12] = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        let split: Vec<&str> = date.split(" ").collect();

        let month: usize = months.iter().position(|&s| s == split[1]).unwrap() + 1;

        let day: &i32 = &split[0][..split[0].len() - 2].parse().unwrap();

        format!("{}-{:02}-{:02}", split[2], month, day)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1507_example_1() {
        let date = "20th Oct 2052".to_string();

        let result = "2052-10-20".to_string();

        assert_eq!(Solution::reformat_date(date), result);
    }

    #[test]
    fn test_1507_example_2() {
        let date = "6th Jun 1933".to_string();

        let result = "1933-06-06".to_string();

        assert_eq!(Solution::reformat_date(date), result);
    }

    #[test]
    fn test_1507_example_3() {
        let date = "26th May 1960".to_string();

        let result = "1960-05-26".to_string();

        assert_eq!(Solution::reformat_date(date), result);
    }
}

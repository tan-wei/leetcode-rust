/**
 * [1854] Maximum Population Year
 *
 * You are given a 2D integer array logs where each logs[i] = [birthi, deathi] indicates the birth and death years of the i^th person.
 * The population of some year x is the number of people alive during that year. The i^th person is counted in year x's population if x is in the inclusive range [birthi, deathi - 1]. Note that the person is not counted in the year that they die.
 * Return the earliest year with the maximum population.
 *  
 * Example 1:
 *
 * Input: logs = [[1993,1999],[2000,2010]]
 * Output: 1993
 * Explanation: The maximum population is 1, and 1993 is the earliest year with this population.
 *
 * Example 2:
 *
 * Input: logs = [[1950,1961],[1960,1971],[1970,1981]]
 * Output: 1960
 * Explanation:
 * The maximum population is 2, and it had happened in years 1960 and 1970.
 * The earlier year between them is 1960.
 *  
 * Constraints:
 *
 * 	1 <= logs.length <= 100
 * 	1950 <= birthi < deathi <= 2050
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-population-year/
// discuss: https://leetcode.com/problems/maximum-population-year/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut most_populated_year = i32::MIN;
        let mut most_population = i32::MIN;

        let count = |y: i32| logs.iter().filter(|l| y >= l[0] && y < l[1]).count() as i32;

        for year in (1950..2050).rev() {
            let population = count(year);
            if most_population <= population {
                most_population = population;
                most_populated_year = year;
            }
        }

        most_populated_year
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1854_example_1() {
        let logs = vec![vec![1993, 1999], vec![2000, 2010]];

        let result = 1993;

        assert_eq!(Solution::maximum_population(logs), result);
    }

    #[test]
    fn test_1854_example_2() {
        let logs = vec![vec![1950, 1961], vec![1960, 1971], vec![1970, 1981]];

        let result = 1960;

        assert_eq!(Solution::maximum_population(logs), result);
    }
}

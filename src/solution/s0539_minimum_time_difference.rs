/**
 * [0539] Minimum Time Difference
 *
 * Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.
 *  
 * Example 1:
 * Input: timePoints = ["23:59","00:00"]
 * Output: 1
 * Example 2:
 * Input: timePoints = ["00:00","23:59","00:00"]
 * Output: 0
 *  
 * Constraints:
 *
 * 	2 <= timePoints.length <= 2 * 10^4
 * 	timePoints[i] is in the format "HH:MM".
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-difference/
// discuss: https://leetcode.com/problems/minimum-time-difference/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points = time_points
            .into_iter()
            .flat_map(|t| {
                let hour_minute = t
                    .split(":")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                vec![
                    std::time::Duration::new(hour_minute[0] * 3600 + hour_minute[1] * 60, 0),
                    std::time::Duration::new(
                        hour_minute[0] * 3600 + hour_minute[1] * 60 + 86400,
                        0,
                    ),
                ]
            })
            .collect::<Vec<std::time::Duration>>();

        time_points.sort_unstable();

        (1..time_points.len())
            .map(|i| (time_points[i] - time_points[i - 1]).as_secs() / 60)
            .min()
            .unwrap() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0539_example_1() {
        let time_points = vec_string!["23:59", "00:00"];
        let result = 1;

        assert_eq!(Solution::find_min_difference(time_points), result)
    }

    #[test]
    fn test_0539_example_2() {
        let time_points = vec_string!["00:00", "23:59", "00:00"];
        let result = 0;

        assert_eq!(Solution::find_min_difference(time_points), result)
    }
}

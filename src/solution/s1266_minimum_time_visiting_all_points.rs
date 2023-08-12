/**
 * [1266] Minimum Time Visiting All Points
 *
 * On a 2D plane, there are n points with integer coordinates points[i] = [xi, yi]. Return the minimum time in seconds to visit all the points in the order given by points.
 * You can move according to these rules:
 *
 * 	In 1 second, you can either:
 *
 * 		move vertically by one unit,
 * 		move horizontally by one unit, or
 * 		move diagonally sqrt(2) units (in other words, move one unit vertically then one unit horizontally in 1 second).
 *
 *
 * 	You have to visit the points in the same order as they appear in the array.
 * 	You are allowed to pass through points that appear later in the order, but these do not count as visits.
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/11/14/1626_example_1.PNG" style="width: 500px; height: 428px;" />
 * Input: points = [[1,1],[3,4],[-1,0]]
 * Output: 7
 * Explanation: One optimal path is [1,1] -> [2,2] -> [3,3] -> [3,4] -> [2,3] -> [1,2] -> [0,1] -> [-1,0]   
 * Time from [1,1] to [3,4] = 3 seconds
 * Time from [3,4] to [-1,0] = 4 seconds
 * Total time = 7 seconds
 * Example 2:
 *
 * Input: points = [[3,2],[-2,2]]
 * Output: 5
 *
 *  
 * Constraints:
 *
 * 	points.length == n
 * 	1 <= n <= 100
 * 	points[i].length == 2
 * 	-1000 <= points[i][0], points[i][1] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-time-visiting-all-points/
// discuss: https://leetcode.com/problems/minimum-time-visiting-all-points/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .iter()
            .zip(points[1..].to_vec())
            .map(|(x, y)| i32::abs(x[0] - y[0]).max(i32::abs(x[1] - y[1])))
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1266_example_1() {
        let points = vec![vec![1, 1], vec![3, 4], vec![-1, 0]];
        let result = 7;

        assert_eq!(Solution::min_time_to_visit_all_points(points), result);
    }

    fn test_1266_example_2() {
        let points = vec![vec![3, 2], vec![-2, 2]];
        let result = 5;

        assert_eq!(Solution::min_time_to_visit_all_points(points), result);
    }
}

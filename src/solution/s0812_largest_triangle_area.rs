/**
 * [0812] Largest Triangle Area
 *
 * Given an array of points on the X-Y plane points where points[i] = [xi, yi], return the area of the largest triangle that can be formed by any three different points. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/04/1027.png" style="height: 369px; width: 450px;" />
 * Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
 * Output: 2.00000
 * Explanation: The five points are shown in the above figure. The red triangle is the largest.
 *
 * Example 2:
 *
 * Input: points = [[1,0],[0,0],[0,1]]
 * Output: 0.50000
 *
 *  
 * Constraints:
 *
 * 	3 <= points.length <= 50
 * 	-50 <= xi, yi <= 50
 * 	All the given points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-triangle-area/
// discuss: https://leetcode.com/problems/largest-triangle-area/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut result: f64 = 0.0;
        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points[i + 1..].iter().enumerate() {
                for p3 in points[j + 1..].iter() {
                    if let [x1, y1] = p1[..] {
                        if let [x2, y2] = p2[..] {
                            if let [x3, y3] = p3[..] {
                                result = result.max(
                                    0.5 * ((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs()
                                        as f64,
                                );
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0812_example_1() {
        let points = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
        let result = 2.00000;

        assert_f64_near!(Solution::largest_triangle_area(points), result);
    }

    #[test]
    fn test_0812_example_2() {
        let points = vec![vec![1, 0], vec![0, 0], vec![0, 1]];
        let result = 0.50000;

        assert_f64_near!(Solution::largest_triangle_area(points), result);
    }
}

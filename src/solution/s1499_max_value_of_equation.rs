/**
 * [1499] Max Value of Equation
 *
 * You are given an array points containing the coordinates of points on a 2D plane, sorted by the x-values, where points[i] = [xi, yi] such that xi < xj for all 1 <= i < j <= points.length. You are also given an integer k.
 * Return the maximum value of the equation yi + yj + |xi - xj| where |xi - xj| <= k and 1 <= i < j <= points.length.
 * It is guaranteed that there exists at least one pair of points that satisfy the constraint |xi - xj| <= k.
 *  
 * Example 1:
 *
 * Input: points = [[1,3],[2,0],[5,10],[6,-10]], k = 1
 * Output: 4
 * Explanation: The first two points satisfy the condition |xi - xj| <= 1 and if we calculate the equation we get 3 + 0 + |1 - 2| = 4. Third and fourth points also satisfy the condition and give a value of 10 + -10 + |5 - 6| = 1.
 * No other pairs satisfy the condition, so we return the max of 4 and 1.
 *
 * Example 2:
 *
 * Input: points = [[0,0],[3,0],[9,2]], k = 3
 * Output: 3
 * Explanation: Only the first two points have an absolute difference of 3 or less in the x-values, and give the value of 0 + 0 + |0 - 3| = 3.
 *
 *  
 * Constraints:
 *
 * 	2 <= points.length <= 10^5
 * 	points[i].length == 2
 * 	-10^8 <= xi, yi <= 10^8
 * 	0 <= k <= 2 * 10^8
 * 	xi < xj for all 1 <= i < j <= points.length
 * 	xi form a strictly increasing sequence.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-value-of-equation/
// discuss: https://leetcode.com/problems/max-value-of-equation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/max-value-of-equation/solutions/3160057/just-a-runnable-solution/
    pub fn find_max_value_of_equation(points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut p = std::collections::BinaryHeap::<(i32, i32)>::new();
        let mut result = std::i32::MIN;
        for point in points.iter() {
            while !p.is_empty() && (point[0] - p.peek().unwrap().1) > k {
                p.pop();
            }
            if !p.is_empty() {
                result = result.max(point[0] + point[1] + p.peek().unwrap().0);
            }
            p.push((point[1] - point[0], point[0]));
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1499_example_1() {
        let points = vec![vec![1, 3], vec![2, 0], vec![5, 10], vec![6, -10]];
        let k = 1;

        let result = 4;

        assert_eq!(Solution::find_max_value_of_equation(points, k), result);
    }

    #[test]
    fn test_1499_example_2() {
        let points = vec![vec![0, 0], vec![3, 0], vec![9, 2]];
        let k = 3;

        let result = 3;

        assert_eq!(Solution::find_max_value_of_equation(points, k), result);
    }
}

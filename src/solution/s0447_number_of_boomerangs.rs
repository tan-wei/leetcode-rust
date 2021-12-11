/**
 * [0447] Number of Boomerangs
 *
 * You are given n points in the plane that are all distinct, where points[i] = [xi, yi]. A boomerang is a tuple of points (i, j, k) such that the distance between i and j equals the distance between i and k (the order of the tuple matters).
 * Return the number of boomerangs.
 *  
 * Example 1:
 *
 * Input: points = [[0,0],[1,0],[2,0]]
 * Output: 2
 * Explanation: The two boomerangs are [[1,0],[0,0],[2,0]] and [[1,0],[2,0],[0,0]].
 *
 * Example 2:
 *
 * Input: points = [[1,1],[2,2],[3,3]]
 * Output: 2
 *
 * Example 3:
 *
 * Input: points = [[1,1]]
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	n == points.length
 * 	1 <= n <= 500
 * 	points[i].length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 	All the points are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-boomerangs/
// discuss: https://leetcode.com/problems/number-of-boomerangs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut mp = std::collections::HashMap::new();
        let mut result = 0;

        for i in 0..points.len() {
            for j in 0..points.len() {
                let dx = points[i][0] - points[j][0];
                let dy = points[i][1] - points[j][1];
                *mp.entry(dx * dx + dy * dy).or_insert(0) += 1;
            }

            result += mp.values().map(|v| v * (v - 1)).sum::<i32>();
            mp.clear();
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0447_example_1() {
        let points = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
        let result = 2;

        assert_eq!(Solution::number_of_boomerangs(points), result);
    }

    #[test]
    fn test_0447_example_2() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let result = 2;

        assert_eq!(Solution::number_of_boomerangs(points), result);
    }

    #[test]
    fn test_0447_example_3() {
        let points = vec![vec![1, 1]];
        let result = 0;

        assert_eq!(Solution::number_of_boomerangs(points), result);
    }
}

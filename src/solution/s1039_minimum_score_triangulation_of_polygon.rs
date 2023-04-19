/**
 * [1039] Minimum Score Triangulation of Polygon
 *
 * You have a convex n-sided polygon where each vertex has an integer value. You are given an integer array values where values[i] is the value of the i^th vertex (i.e., clockwise order).
 * You will triangulate the polygon into n - 2 triangles. For each triangle, the value of that triangle is the product of the values of its vertices, and the total score of the triangulation is the sum of these values over all n - 2 triangles in the triangulation.
 * Return the smallest possible total score that you can achieve with some triangulation of the polygon.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/shape1.jpg" style="width: 201px; height: 133px;" />
 * Input: values = [1,2,3]
 * Output: 6
 * Explanation: The polygon is already triangulated, and the score of the only triangle is 6.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/shape2.jpg" style="width: 446px; height: 163px;" />
 * Input: values = [3,7,4,5]
 * Output: 144
 * Explanation: There are two triangulations, with possible scores: 3*7*5 + 4*5*7 = 245, or 3*4*5 + 3*4*7 = 144.
 * The minimum score is 144.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/25/shape3.jpg" style="width: 207px; height: 163px;" />
 * Input: values = [1,3,1,4,1,5]
 * Output: 13
 * Explanation: The minimum score triangulation has score 1*1*3 + 1*1*4 + 1*1*5 + 1*1*1 = 13.
 *
 *  
 * Constraints:
 *
 * 	n == values.length
 * 	3 <= n <= 50
 * 	1 <= values[i] <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
// discuss: https://leetcode.com/problems/minimum-score-triangulation-of-polygon/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-score-triangulation-of-polygon/solutions/534311/rust-recursive-with-memo/
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let mut memo = std::collections::HashMap::new();
        Self::dfs_helper(&values, 0, values.len() - 1, &mut memo)
    }

    fn dfs_helper(
        values: &Vec<i32>,
        i: usize,
        j: usize,
        mut memo: &mut std::collections::HashMap<String, i32>,
    ) -> i32 {
        if j - i < 2 {
            return 0;
        }
        if j - i == 2 {
            return values[i] * values[i + 1] * values[i + 2];
        }

        let key = format!("{},{}", i, j);
        if let Some(&v) = memo.get(&key) {
            return v;
        }

        let mut result = i32::max_value();

        for k in i + 1..j {
            let min_score_i = values[i] * values[j] * values[k]
                + Self::dfs_helper(values, i, k, &mut memo)
                + Self::dfs_helper(values, k, j, &mut memo);

            result = std::cmp::min(result, min_score_i);
        }

        memo.entry(key).or_insert(result);
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1039_example_1() {}
}

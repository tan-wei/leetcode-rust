/**
 * [120] Triangle
 *
 * Given a triangle array, return the minimum path sum from top to bottom.
 * For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.
 *  
 * Example 1:
 *
 * Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
 * Output: 11
 * Explanation: The triangle looks like:
 *    <u>2</u>
 *   <u>3</u> 4
 *  6 <u>5</u> 7
 * 4 <u>1</u> 8 3
 * The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
 *
 * Example 2:
 *
 * Input: triangle = [[-10]]
 * Output: -10
 *
 *  
 * Constraints:
 *
 * 	1 <= triangle.length <= 200
 * 	triangle[0].length == 1
 * 	triangle[i].length == triangle[i - 1].length + 1
 * 	-10^4 <= triangle[i][j] <= 10^4
 *
 *  
 * Follow up: Could you do this using only O(n) extra space, where n is the total number of rows in the triangle?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/triangle/
// discuss: https://leetcode.com/problems/triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        (0..triangle.len() - 1)
            .rev()
            .fold(triangle[triangle.len() - 1].clone(), |mut acc, i| {
                (0..=i).for_each(|j| acc[j] = triangle[i][j] + acc[j].min(acc[j + 1]));
                acc
            })[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0120_example_1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let result = 11;

        assert_eq!(Solution::minimum_total(triangle), result);
    }

    #[test]
    fn test_0120_example_2() {
        let triangle = vec![vec![-10]];
        let result = -10;

        assert_eq!(Solution::minimum_total(triangle), result);
    }
}

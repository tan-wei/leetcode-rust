/**
 * [2392] Build a Matrix With Conditions
 *
 * You are given a positive integer k. You are also given:
 *
 * 	a 2D integer array rowConditions of size n where rowConditions[i] = [abovei, belowi], and
 * 	a 2D integer array colConditions of size m where colConditions[i] = [lefti, righti].
 *
 * The two arrays contain integers from 1 to k.
 * You have to build a k x k matrix that contains each of the numbers from 1 to k exactly once. The remaining cells should have the value 0.
 * The matrix should also satisfy the following conditions:
 *
 * 	The number abovei should appear in a row that is strictly above the row at which the number belowi appears for all i from 0 to n - 1.
 * 	The number lefti should appear in a column that is strictly left of the column at which the number righti appears for all i from 0 to m - 1.
 *
 * Return any matrix that satisfies the conditions. If no answer exists, return an empty matrix.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/07/06/gridosdrawio.png" style="width: 211px; height: 211px;" />
 * Input: k = 3, rowConditions = [[1,2],[3,2]], colConditions = [[2,1],[3,2]]
 * Output: [[3,0,0],[0,0,1],[0,2,0]]
 * Explanation: The diagram above shows a valid example of a matrix that satisfies all the conditions.
 * The row conditions are the following:
 * - Number 1 is in row <u>1</u>, and number 2 is in row <u>2</u>, so 1 is above 2 in the matrix.
 * - Number 3 is in row <u>0</u>, and number 2 is in row <u>2</u>, so 3 is above 2 in the matrix.
 * The column conditions are the following:
 * - Number 2 is in column <u>1</u>, and number 1 is in column <u>2</u>, so 2 is left of 1 in the matrix.
 * - Number 3 is in column <u>0</u>, and number 2 is in column <u>1</u>, so 3 is left of 2 in the matrix.
 * Note that there may be multiple correct answers.
 *
 * Example 2:
 *
 * Input: k = 3, rowConditions = [[1,2],[2,3],[3,1],[2,3]], colConditions = [[2,1]]
 * Output: []
 * Explanation: From the first two conditions, 3 has to be below 1 but the third conditions needs 3 to be above 1 to be satisfied.
 * No matrix can satisfy all the conditions, so we return the empty matrix.
 *
 *  
 * Constraints:
 *
 * 	2 <= k <= 400
 * 	1 <= rowConditions.length, colConditions.length <= 10^4
 * 	rowConditions[i].length == colConditions[i].length == 2
 * 	1 <= abovei, belowi, lefti, righti <= k
 * 	abovei != belowi
 * 	lefti != righti
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/build-a-matrix-with-conditions/
// discuss: https://leetcode.com/problems/build-a-matrix-with-conditions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2392_example_1() {
        let k = 3;
        let row_conditions = vec![vec![1, 2], vec![3, 2]];
        let col_conditions = vec![vec![2, 1], vec![3, 2]];

        let result = vec![vec![3, 0, 0], vec![0, 0, 1], vec![0, 2, 0]];

        assert_eq!(
            Solution::build_matrix(k, row_conditions, col_conditions),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2392_example_2() {
        let k = 3;
        let row_conditions = vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![2, 3]];
        let col_conditions = vec![vec![2, 1]];

        let result: Vec<Vec<i32>> = vec![];

        assert_eq!(
            Solution::build_matrix(k, row_conditions, col_conditions),
            result
        );
    }
}

/**
 * [1735] Count Ways to Make Array With Product
 *
 * You are given a 2D integer array, queries. For each queries[i], where queries[i] = [ni, ki], find the number of different ways you can place positive integers into an array of size ni such that the product of the integers is ki. As the number of ways may be too large, the answer to the i^th query is the number of ways modulo 10^9 + 7.
 * Return an integer array answer where answer.length == queries.length, and answer[i] is the answer to the i^th query.
 *  
 * Example 1:
 *
 * Input: queries = [[2,6],[5,1],[73,660]]
 * Output: [4,1,50734910]
 * Explanation: Each query is independent.
 * [2,6]: There are 4 ways to fill an array of size 2 that multiply to 6: [1,6], [2,3], [3,2], [6,1].
 * [5,1]: There is 1 way to fill an array of size 5 that multiply to 1: [1,1,1,1,1].
 * [73,660]: There are 1050734917 ways to fill an array of size 73 that multiply to 660. 1050734917 modulo 10^9 + 7 = 50734910.
 *
 * Example 2:
 *
 * Input: queries = [[1,1],[2,2],[3,3],[4,4],[5,5]]
 * Output: [1,2,3,10,5]
 *
 *  
 * Constraints:
 *
 * 	1 <= queries.length <= 10^4
 * 	1 <= ni, ki <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-ways-to-make-array-with-product/
// discuss: https://leetcode.com/problems/count-ways-to-make-array-with-product/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    #[ignore]
    fn test_1735_example_1() {
        let queries = vec![vec![2, 6], vec![5, 1], vec![73, 660]];

        let result = vec![4, 1, 50734910];

        assert_eq!(Solution::ways_to_fill_array(queries), result);
    }

    #[test]
    #[ignore]
    fn test_1735_example_2() {
        let queries = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];

        let result = vec![1, 2, 3, 10, 5];

        assert_eq!(Solution::ways_to_fill_array(queries), result);
    }
}

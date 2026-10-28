/**
 * [2672] Number of Adjacent Elements With the Same Color
 *
 * You are given an integer n representing an array colors of length n where all elements are set to 0's meaning uncolored. You are also given a 2D integer array queries where queries[i] = [indexi, colori]. For the i^th query:
 *
 * 	Set colors[indexi] to colori.
 * 	Count the number of adjacent pairs in colors which have the same color (regardless of colori).
 *
 * Return an array answer of the same length as queries where answer[i] is the answer to the i^th query.
 *  
 * Example 1:
 *
 * Input: n = 4, queries = [[0,2],[1,2],[3,1],[1,1],[2,1]]
 * Output: [0,1,1,0,2]
 * Explanation:
 *
 * 	Initially array colors = [0,0,0,0], where 0 denotes uncolored elements of the array.
 * 	After the 1^st query colors = [2,0,0,0]. The count of adjacent pairs with the same color is 0.
 * 	After the 2^nd query colors = [2,2,0,0]. The count of adjacent pairs with the same color is 1.
 * 	After the 3^rd query colors = [2,2,0,1]. The count of adjacent pairs with the same color is 1.
 * 	After the 4^th query colors = [2,1,0,1]. The count of adjacent pairs with the same color is 0.
 * 	After the 5^th query colors = [2,1,1,1]. The count of adjacent pairs with the same color is 2.
 *
 * Example 2:
 *
 * Input: n = 1, queries = [[0,100000]]
 * Output: [0]
 * Explanation:
 * After the 1^st query colors = [100000]. The count of adjacent pairs with the same color is 0.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	1 <= queries.length <= 10^5
 * 	queries[i].length == 2
 * 	0 <= indexi <= n - 1
 * 	1 <=  colori <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-adjacent-elements-with-the-same-color/
// discuss: https://leetcode.com/problems/number-of-adjacent-elements-with-the-same-color/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2672_example_1() {
        let n = 4;
        let queries = vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]];

        let result = vec![0, 1, 1, 0, 2];

        assert_eq!(Solution::color_the_array(n, queries), result);
    }

    #[test]
    #[ignore]
    fn test_2672_example_2() {
        let n = 1;
        let queries = vec![vec![0, 100000]];

        let result = vec![0];

        assert_eq!(Solution::color_the_array(n, queries), result);
    }
}

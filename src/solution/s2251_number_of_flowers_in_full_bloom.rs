/**
 * [2251] Number of Flowers in Full Bloom
 *
 * You are given a 0-indexed 2D integer array flowers, where flowers[i] = [starti, endi] means the i^th flower will be in full bloom from starti to endi (inclusive). You are also given a 0-indexed integer array people of size n, where people[i] is the time that the i^th person will arrive to see the flowers.
 * Return an integer array answer of size n, where answer[i] is the number of flowers that are in full bloom when the i^th person arrives.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/ex1new.jpg" style="width: 550px; height: 216px;" />
 * Input: flowers = [[1,6],[3,7],[9,12],[4,13]], people = [2,3,7,11]
 * Output: [1,2,2,2]
 * Explanation: The figure above shows the times when the flowers are in full bloom and when the people arrive.
 * For each person, we return the number of flowers in full bloom during their arrival.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/ex2new.jpg" style="width: 450px; height: 195px;" />
 * Input: flowers = [[1,10],[3,3]], people = [3,3,2]
 * Output: [2,2,1]
 * Explanation: The figure above shows the times when the flowers are in full bloom and when the people arrive.
 * For each person, we return the number of flowers in full bloom during their arrival.
 *
 *  
 * Constraints:
 *
 * 	1 <= flowers.length <= 5 * 10^4
 * 	flowers[i].length == 2
 * 	1 <= starti <= endi <= 10^9
 * 	1 <= people.length <= 5 * 10^4
 * 	1 <= people[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-flowers-in-full-bloom/
// discuss: https://leetcode.com/problems/number-of-flowers-in-full-bloom/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2251_example_1() {
        let flowers = vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]];
        let people = vec![2, 3, 7, 11];

        let result = vec![1, 2, 2, 2];

        assert_eq!(Solution::full_bloom_flowers(flowers, people), result);
    }

    #[test]
    #[ignore]
    fn test_2251_example_2() {
        let flowers = vec![vec![1, 10], vec![3, 3]];
        let people = vec![3, 3, 2];

        let result = vec![2, 2, 1];

        assert_eq!(Solution::full_bloom_flowers(flowers, people), result);
    }
}

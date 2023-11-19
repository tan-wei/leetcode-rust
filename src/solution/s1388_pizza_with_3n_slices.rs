/**
 * [1388] Pizza With 3n Slices
 *
 * There is a pizza with 3n slices of varying size, you and your friends will take slices of pizza as follows:
 *
 * 	You will pick any pizza slice.
 * 	Your friend Alice will pick the next slice in the anti-clockwise direction of your pick.
 * 	Your friend Bob will pick the next slice in the clockwise direction of your pick.
 * 	Repeat until there are no more slices of pizzas.
 *
 * Given an integer array slices that represent the sizes of the pizza slices in a clockwise direction, return the maximum possible sum of slice sizes that you can pick.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/18/sample_3_1723.png" style="width: 500px; height: 266px;" />
 * Input: slices = [1,2,3,4,5,6]
 * Output: 10
 * Explanation: Pick pizza slice of size 4, Alice and Bob will pick slices with size 3 and 5 respectively. Then Pick slices with size 6, finally Alice and Bob will pick slice of size 2 and 1 respectively. Total = 4 + 6.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/18/sample_4_1723.png" style="width: 500px; height: 299px;" />
 * Input: slices = [8,9,8,6,1,1]
 * Output: 16
 * Explanation: Pick pizza slice of size 8 in each turn. If you pick slice with size 9 your partners will pick slices of size 8.
 *
 *  
 * Constraints:
 *
 * 	3 * n == slices.length
 * 	1 <= slices.length <= 500
 * 	1 <= slices[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pizza-with-3n-slices/
// discuss: https://leetcode.com/problems/pizza-with-3n-slices/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/pizza-with-3n-slices/solutions/3107696/just-a-runnable-solution/
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let m = slices.len();
        let n = m / 3;
        let slices1 = slices[0..m - 1].to_vec();
        let slices2 = slices[1..m].to_vec();
        let val1 = Self::max_sum(&slices1, n);
        let val2 = Self::max_sum(&slices2, n);
        std::cmp::max(val1, val2)
    }

    fn max_sum(slices: &Vec<i32>, n: usize) -> i32 {
        let m = slices.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                if i == 1 {
                    dp[i][j] = slices[0];
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i - 2][j - 1] + slices[i - 1]);
                }
            }
        }
        dp[m][n]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1388_example_1() {
        let slices = vec![1, 2, 3, 4, 5, 6];

        let result = 10;

        assert_eq!(Solution::max_size_slices(slices), result);
    }

    #[test]
    fn test_1388_example_2() {
        let slices = vec![8, 9, 8, 6, 1, 1];

        let result = 16;

        assert_eq!(Solution::max_size_slices(slices), result);
    }
}

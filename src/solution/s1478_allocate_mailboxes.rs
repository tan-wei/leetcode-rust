/**
 * [1478] Allocate Mailboxes
 *
 * Given the array houses where houses[i] is the location of the i^th house along a street and an integer k, allocate k mailboxes in the street.
 * Return the minimum total distance between each house and its nearest mailbox.
 * The test cases are generated so that the answer fits in a 32-bit integer.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/sample_11_1816.png" style="width: 454px; height: 154px;" />
 * Input: houses = [1,4,8,10,20], k = 3
 * Output: 5
 * Explanation: Allocate mailboxes in position 3, 9 and 20.
 * Minimum total distance from each houses to nearest mailboxes is |3-1| + |4-3| + |9-8| + |10-9| + |20-20| = 5
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/sample_2_1816.png" style="width: 433px; height: 154px;" />
 * Input: houses = [2,3,5,12,18], k = 2
 * Output: 9
 * Explanation: Allocate mailboxes in position 3 and 14.
 * Minimum total distance from each houses to nearest mailboxes is |2-3| + |3-3| + |5-3| + |12-14| + |18-14| = 9.
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= houses.length <= 100
 * 	1 <= houses[i] <= 10^4
 * 	All the integers of houses are unique.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/allocate-mailboxes/
// discuss: https://leetcode.com/problems/allocate-mailboxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/allocate-mailboxes/solutions/3159938/just-a-runnable-solution/
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        let mut houses = houses;
        houses.sort();
        let n = houses.len();
        let mut dp = vec![vec![0; n]; n];
        for (i, item) in dp.iter_mut().enumerate().take(n) {
            for (j, item0) in item.iter_mut().enumerate().take(n).skip(i) {
                let mid = (i + j) / 2;
                for k in i..=j {
                    *item0 += (houses[k] - houses[mid]).abs();
                }
            }
        }
        let mut dp2 = vec![vec![0; n]; k as usize];
        for i in 0..n {
            dp2[0][i] = dp[0][i];
        }
        for i in 1..k as usize {
            for j in i..n {
                let mut min = std::i32::MAX;
                for k in i - 1..j {
                    min = min.min(dp2[i - 1][k] + dp[k + 1][j]);
                }
                dp2[i][j] = min;
            }
        }
        dp2[k as usize - 1][n - 1]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1478_example_1() {
        let houses = vec![1, 4, 8, 10, 20];
        let k = 3;

        let result = 5;

        assert_eq!(Solution::min_distance(houses, k), result);
    }

    #[test]
    fn test_1478_example_2() {
        let houses = vec![2, 3, 5, 12, 18];
        let k = 2;

        let result = 9;

        assert_eq!(Solution::min_distance(houses, k), result);
    }
}

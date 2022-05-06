/**
 * [0668] Kth Smallest Number in Multiplication Table
 *
 * Nearly everyone has used the <a href="https://en.wikipedia.org/wiki/Multiplication_table" target="_blank">Multiplication Table</a>. The multiplication table of size m x n is an integer matrix mat where mat[i][j] == i * j (1-indexed).
 * Given three integers m, n, and k, return the k^th smallest element in the m x n multiplication table.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/multtable1-grid.jpg" style="width: 500px; height: 254px;" />
 * Input: m = 3, n = 3, k = 5
 * Output: 3
 * Explanation: The 5^th smallest number is 3.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/02/multtable2-grid.jpg" style="width: 493px; height: 293px;" />
 * Input: m = 2, n = 3, k = 6
 * Output: 6
 * Explanation: The 6^th smallest number is 6.
 *
 *  
 * Constraints:
 *
 * 	1 <= m, n <= 3 * 10^4
 * 	1 <= k <= m * n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/
// discuss: https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (m, n) = if m > n { (n, m) } else { (m, n) };

        let (mut left, mut right) = (1, n * m);
        while left < right {
            let mid = (left + right) / 2;
            let mut rank = 0;
            for i in 1..m + 1 {
                rank += n.min(mid / i);
            }

            if rank < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0668_example_1() {
        let m = 3;
        let n = 3;
        let k = 5;
        let result = 3;

        assert_eq!(Solution::find_kth_number(m, n, k), result);
    }

    #[test]
    fn test_0668_example_2() {
        let m = 2;
        let n = 3;
        let k = 6;
        let result = 6;

        assert_eq!(Solution::find_kth_number(m, n, k), result);
    }
}

/**
 * [1551] Minimum Operations to Make Array Equal
 *
 * You have an array arr of length n where arr[i] = (2 * i) + 1 for all valid values of i (i.e., 0 <= i < n).
 * In one operation, you can select two indices x and y where 0 <= x, y < n and subtract 1 from arr[x] and add 1 to arr[y] (i.e., perform arr[x] -=1 and arr[y] += 1). The goal is to make all the elements of the array equal. It is guaranteed that all the elements of the array can be made equal using some operations.
 * Given an integer n, the length of the array, return the minimum number of operations needed to make all the elements of arr equal.
 *  
 * Example 1:
 *
 * Input: n = 3
 * Output: 2
 * Explanation: arr = [1, 3, 5]
 * First operation choose x = 2 and y = 0, this leads arr to be [2, 3, 4]
 * In the second operation choose x = 2 and y = 0 again, thus arr = [3, 3, 3].
 *
 * Example 2:
 *
 * Input: n = 6
 * Output: 9
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-operations-to-make-array-equal/
// discuss: https://leetcode.com/problems/minimum-operations-to-make-array-equal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        n * n / 4
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1551_example_1() {
        let n = 3;

        let result = 2;

        assert_eq!(Solution::min_operations(n), result);
    }

    #[test]
    fn test_1551_example_2() {
        let n = 6;

        let result = 9;

        assert_eq!(Solution::min_operations(n), result);
    }
}

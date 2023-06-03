/**
 * [1131] Maximum of Absolute Value Expression
 *
 * Given two arrays of integers with equal lengths, return the maximum value of:
 *
 * |arr1[i] - arr1[j]| + |arr2[i] - arr2[j]| + |i - j|
 *
 * where the maximum is taken over all 0 <= i, j < arr1.length.
 *  
 * Example 1:
 *
 * Input: arr1 = [1,2,3,4], arr2 = [-1,4,5,6]
 * Output: 13
 *
 * Example 2:
 *
 * Input: arr1 = [1,-2,-5,0,10], arr2 = [0,-2,-1,-7,-4]
 * Output: 20
 *
 *  
 * Constraints:
 *
 * 	2 <= arr1.length == arr2.length <= 40000
 * 	-10^6 <= arr1[i], arr2[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-of-absolute-value-expression/
// discuss: https://leetcode.com/problems/maximum-of-absolute-value-expression/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut result = 0;
        let n = arr1.len();

        for p in vec![1, -1] {
            for q in vec![1, -1] {
                let mut closest = p * arr1[0] + q * arr2[0] + 0;
                for i in 1..n {
                    let cur = p * arr1[i] + q * arr2[i] + i as i32;
                    result = std::cmp::max(result, cur - closest);
                    closest = std::cmp::min(closest, cur);
                }
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1131_example_1() {
        let arr1 = vec![1, 2, 3, 4];
        let arr2 = vec![-1, 4, 5, 6];
        let result = 13;

        assert_eq!(Solution::max_abs_val_expr(arr1, arr2), result);
    }

    #[test]
    fn test_1131_example_2() {
        let arr1 = vec![1, -2, -5, 0, 10];
        let arr2 = vec![0, -2, -1, -7, -4];
        let result = 20;

        assert_eq!(Solution::max_abs_val_expr(arr1, arr2), result);
    }
}

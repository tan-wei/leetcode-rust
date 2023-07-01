/**
 * [1187] Make Array Strictly Increasing
 *
 * Given two integer arrays arr1 and arr2, return the minimum number of operations (possibly zero) needed to make arr1 strictly increasing.
 * In one operation, you can choose two indices 0 <= i < arr1.length and 0 <= j < arr2.length and do the assignment arr1[i] = arr2[j].
 * If there is no way to make arr1 strictly increasing, return -1.
 *  
 * Example 1:
 *
 * Input: arr1 = [1,5,3,6,7], arr2 = [1,3,2,4]
 * Output: 1
 * Explanation: Replace 5 with 2, then arr1 = [1, 2, 3, 6, 7].
 *
 * Example 2:
 *
 * Input: arr1 = [1,5,3,6,7], arr2 = [4,3,1]
 * Output: 2
 * Explanation: Replace 5 with 3 and then replace 3 with 4. arr1 = [1, 3, 4, 6, 7].
 *
 * Example 3:
 *
 * Input: arr1 = [1,5,3,6,7], arr2 = [1,6,3,3]
 * Output: -1
 * Explanation: You can't make arr1 strictly increasing.
 *  
 * Constraints:
 *
 * 	1 <= arr1.length, arr2.length <= 2000
 * 	0 <= arr1[i], arr2[i] <= 10^9
 *
 *  
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-array-strictly-increasing/
// discuss: https://leetcode.com/problems/make-array-strictly-increasing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/make-array-strictly-increasing/solutions/3647105/rust-dp-solution/
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;
        arr2.sort();
        arr2.dedup();

        let inf = 1 << 30;
        let mut arr1 = arr1;
        arr1.push(inf);
        arr1.insert(0, -inf);

        let mut dp = vec![inf; arr1.len()];
        dp[0] = 0;

        for i in 1..arr1.len() {
            if arr1[i - 1] < arr1[i] {
                dp[i] = dp[i - 1];
            }

            let j = match arr2.binary_search(&arr1[i]) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };

            for k in 1..=j.min(i - 1) {
                if arr1[i - k - 1] < arr2[j - k] {
                    dp[i] = dp[i].min(dp[i - k - 1] + k as i32);
                }
            }
        }

        if dp[arr1.len() - 1] >= inf {
            -1
        } else {
            dp[arr1.len() - 1]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1187_example_1() {
        let arr1 = vec![1, 5, 3, 6, 7];
        let arr2 = vec![1, 3, 2, 4];
        let result = 1;

        assert_eq!(Solution::make_array_increasing(arr1, arr2), result);
    }

    #[test]
    fn test_1187_example_2() {
        let arr1 = vec![1, 5, 3, 6, 7];
        let arr2 = vec![4, 3, 1];
        let result = 2;

        assert_eq!(Solution::make_array_increasing(arr1, arr2), result);
    }

    #[test]
    fn test_1187_example_3() {
        let arr1 = vec![1, 5, 3, 6, 7];
        let arr2 = vec![1, 6, 3, 3];
        let result = -1;

        assert_eq!(Solution::make_array_increasing(arr1, arr2), result);
    }
}

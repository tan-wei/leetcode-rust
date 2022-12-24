/**
 * [0923] 3Sum With Multiplicity
 *
 * Given an integer array arr, and an integer target, return the number of tuples i, j, k such that i < j < k and arr[i] + arr[j] + arr[k] == target.
 * As the answer can be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: arr = [1,1,2,2,3,3,4,4,5,5], target = 8
 * Output: 20
 * Explanation:
 * Enumerating by the values (arr[i], arr[j], arr[k]):
 * (1, 2, 5) occurs 8 times;
 * (1, 3, 4) occurs 8 times;
 * (2, 2, 4) occurs 2 times;
 * (2, 3, 3) occurs 2 times.
 *
 * Example 2:
 *
 * Input: arr = [1,1,2,2,2,2], target = 5
 * Output: 12
 * Explanation:
 * arr[i] = 1, arr[j] = arr[k] = 2 occurs 12 times:
 * We choose one 1 from [1,1] in 2 ways,
 * and two 2s from [2,2,2,2] in 6 ways.
 *
 * Example 3:
 *
 * Input: arr = [2,1,3], target = 6
 * Output: 1
 * Explanation: (1, 2, 3) occured one time in the array so we return 1.
 *
 *  
 * Constraints:
 *
 * 	3 <= arr.length <= 3000
 * 	0 <= arr[i] <= 100
 * 	0 <= target <= 300
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/3sum-with-multiplicity/
// discuss: https://leetcode.com/problems/3sum-with-multiplicity/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let mut result = 0;
        for i in 1..n {
            let v1 = arr[i];
            let mut memo = vec![0; target as usize + 1];
            for j in 0..i {
                let v2 = arr[j];
                let tot = v1 + v2;
                if target < tot {
                    continue;
                }
                memo[tot as usize] += 1;
            }
            for j in i + 1..n {
                let v3 = arr[j];
                if target < v3 {
                    continue;
                }
                let need = target - v3;
                result += memo[need as usize];
                result %= MOD;
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
    fn test_0923_example_1() {
        let arr = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5];
        let target = 8;
        let result = 20;

        assert_eq!(Solution::three_sum_multi(arr, target), result);
    }

    #[test]
    fn test_0923_example_2() {
        let arr = vec![1, 1, 2, 2, 2, 2];
        let target = 5;
        let result = 12;

        assert_eq!(Solution::three_sum_multi(arr, target), result);
    }

    #[test]
    fn test_0923_example_3() {
        let arr = vec![2, 1, 3];
        let target = 6;
        let result = 1;

        assert_eq!(Solution::three_sum_multi(arr, target), result);
    }
}

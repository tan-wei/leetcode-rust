/**
 * [1300] Sum of Mutated Array Closest to Target
 *
 * Given an integer array arr and a target value target, return the integer value such that when we change all the integers larger than value in the given array to be equal to value, the sum of the array gets as close as possible (in absolute difference) to target.
 * In case of a tie, return the minimum such integer.
 * Notice that the answer is not neccesarilly a number from arr.
 *
 * Example 1:
 *
 * Input: arr = [4,9,3], target = 10
 * Output: 3
 * Explanation: When using 3 arr converts to [3, 3, 3] which sums 9 and that's the optimal answer.
 *
 * Example 2:
 *
 * Input: arr = [2,3,5], target = 10
 * Output: 5
 *
 * Example 3:
 *
 * Input: arr = [60864,25176,27249,21296,20204], target = 56803
 * Output: 11361
 *
 *
 * Constraints:
 *
 * 	1 <= arr.length <= 10^4
 * 	1 <= arr[i], target <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-mutated-array-closest-to-target/
// discuss: https://leetcode.com/problems/sum-of-mutated-array-closest-to-target/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sum-of-mutated-array-closest-to-target/solutions/3029793/rust-solution-using-ternary-search/
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = 100010;
        while left + 2 < right {
            let lr1 = (left * 2 + right) / 3;
            let lr2 = (left + right * 2) / 3;

            let lr1v = Self::helper(&arr, target, lr1);
            let lr2v = Self::helper(&arr, target, lr2);

            if lr1v > lr2v {
                left = lr1;
            } else {
                right = lr2;
            }
        }

        let mut min = i32::max_value();
        let mut result = 0;
        for mid in (left..=right).rev() {
            let v = Self::helper(&arr, target, mid);
            if v <= min {
                min = v;
                result = mid;
            }
        }

        result
    }

    fn helper(arr: &Vec<i32>, target: i32, mid: i32) -> i32 {
        let mut temp = 0;
        for &v in arr {
            temp += v.min(mid);
        }
        (target - temp).abs()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1300_example_1() {
        let arr = vec![4, 9, 3];
        let target = 10;
        let result = 3;

        assert_eq!(Solution::find_best_value(arr, target), result);
    }

    #[test]
    fn test_1300_example_2() {
        let arr = vec![2, 3, 5];
        let target = 10;
        let result = 5;

        assert_eq!(Solution::find_best_value(arr, target), result);
    }

    #[test]
    fn test_1300_example_3() {
        let arr = vec![60864, 25176, 27249, 21296, 20204];
        let target = 56803;
        let result = 11361;

        assert_eq!(Solution::find_best_value(arr, target), result);
    }
}

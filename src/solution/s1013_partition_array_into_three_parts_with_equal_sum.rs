/**
 * [1013] Partition Array Into Three Parts With Equal Sum
 *
 * Given an array of integers arr, return true if we can partition the array into three non-empty parts with equal sums.
 * Formally, we can partition the array if we can find indexes i + 1 < j with (arr[0] + arr[1] + ... + arr[i] == arr[i + 1] + arr[i + 2] + ... + arr[j - 1] == arr[j] + arr[j + 1] + ... + arr[arr.length - 1])
 *  
 * Example 1:
 *
 * Input: arr = [0,2,1,-6,6,-7,9,1,2,0,1]
 * Output: true
 * Explanation: 0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1
 *
 * Example 2:
 *
 * Input: arr = [0,2,1,-6,6,7,9,-1,2,0,1]
 * Output: false
 *
 * Example 3:
 *
 * Input: arr = [3,3,6,5,-2,2,5,1,-9,4]
 * Output: true
 * Explanation: 3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
 *
 *  
 * Constraints:
 *
 * 	3 <= arr.length <= 5 * 10^4
 * 	-10^4 <= arr[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/
// discuss: https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum = arr.iter().sum::<i32>();
        if sum % 3 != 0 {
            return false;
        }

        let sum_part = sum / 3;
        let (mut n, mut acc) = (0, 0);

        for &val in &arr {
            acc += val;
            if acc == sum_part {
                n += 1;
                acc = 0;
            }
        }

        n >= 3
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1013_example_1() {
        let arr = vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1];
        let result = true;

        assert_eq!(Solution::can_three_parts_equal_sum(arr), result);
    }

    #[test]
    fn test_1013_example_2() {
        let arr = vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1];
        let result = false;

        assert_eq!(Solution::can_three_parts_equal_sum(arr), result);
    }

    #[test]
    fn test_1013_example_3() {
        let arr = vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4];
        let result = true;

        assert_eq!(Solution::can_three_parts_equal_sum(arr), result);
    }
}

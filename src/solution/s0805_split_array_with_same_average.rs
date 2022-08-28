/**
 * [0805] Split Array With Same Average
 *
 * You are given an integer array nums.
 * You should move each element of nums into one of the two arrays A and B such that A and B are non-empty, and average(A) == average(B).
 * Return true if it is possible to achieve that and false otherwise.
 * Note that for an array arr, average(arr) is the sum of all the elements of arr over the length of arr.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4,5,6,7,8]
 * Output: true
 * Explanation: We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
 *
 * Example 2:
 *
 * Input: nums = [3,1]
 * Output: false
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 30
 * 	0 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-array-with-same-average/
// discuss: https://leetcode.com/problems/split-array-with-same-average/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://rustgym.com/leetcode/805
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        for k in 1..=n / 2 {
            if sum * k as i32 % n as i32 == 0 {
                let target = sum * k as i32 / n as i32;
                if Self::dp_helper(target, k, n, &mut std::collections::HashMap::new(), &nums) {
                    return true;
                }
            }
        }
        false
    }

    fn dp_helper(
        target: i32,
        k: usize,
        n: usize,
        memo: &mut std::collections::HashMap<(i32, usize, usize), bool>,
        nums: &[i32],
    ) -> bool {
        if let Some(&res) = memo.get(&(target, k, n)) {
            res
        } else {
            let res = if k == 0 {
                target == 0
            } else {
                if n < k {
                    false
                } else {
                    let next = target - nums[n - 1];
                    Self::dp_helper(target, k, n - 1, memo, nums)
                        || next >= 0 && Self::dp_helper(next, k - 1, n - 1, memo, nums)
                }
            };
            memo.insert((target, k, n), res);
            res
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0805_example_1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let result = true;

        assert_eq!(Solution::split_array_same_average(nums), result);
    }

    #[test]
    fn test_0805_example_2() {
        let nums = vec![3, 1];
        let result = false;

        assert_eq!(Solution::split_array_same_average(nums), result);
    }
}

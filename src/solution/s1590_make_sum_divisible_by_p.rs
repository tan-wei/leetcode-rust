/**
 * [1590] Make Sum Divisible by P
 *
 * Given an array of positive integers nums, remove the smallest subarray (possibly empty) such that the sum of the remaining elements is divisible by p. It is not allowed to remove the whole array.
 * Return the length of the smallest subarray that you need to remove, or -1 if it's impossible.
 * A subarray is defined as a contiguous block of elements in the array.
 *  
 * Example 1:
 *
 * Input: nums = [3,1,4,2], p = 6
 * Output: 1
 * Explanation: The sum of the elements in nums is 10, which is not divisible by 6. We can remove the subarray [4], and the sum of the remaining elements is 6, which is divisible by 6.
 *
 * Example 2:
 *
 * Input: nums = [6,3,5,2], p = 9
 * Output: 2
 * Explanation: We cannot remove a single element to get a sum divisible by 9. The best way is to remove the subarray [5,2], leaving us with [6,3] with sum 9.
 *
 * Example 3:
 *
 * Input: nums = [1,2,3], p = 3
 * Output: 0
 * Explanation: Here the sum is 6. which is already divisible by 3. Thus we do not need to remove anything.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 * 	1 <= p <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-sum-divisible-by-p/
// discuss: https://leetcode.com/problems/make-sum-divisible-by-p/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let p = p as i64;
        let sum = nums.iter().sum::<i64>();
        let rem = sum % p;
        if rem == 0 {
            return 0;
        }
        let mut result = nums.len() as i64;
        let mut m = std::collections::HashMap::new();
        let mut sum = 0;

        for (i, &num) in nums.iter().enumerate() {
            let i = i as i64;
            sum += num;
            let k = sum % p;
            if k == rem {
                result = result.min(i + 1);
            }
            if let Some(&j) = m.get(&(k - rem)) {
                result = result.min(i - j);
            }
            if let Some(&j) = m.get(&(p + k - rem)) {
                result = result.min(i - j);
            }
            m.insert(k, i);
        }

        if result >= nums.len() as i64 {
            return -1;
        }

        result as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1590_example_1() {
        let nums = vec![3, 1, 4, 2];
        let p = 6;

        let result = 1;

        assert_eq!(Solution::min_subarray(nums, p), result);
    }

    #[test]
    fn test_1590_example_2() {
        let nums = vec![6, 3, 5, 2];
        let p = 9;

        let result = 2;

        assert_eq!(Solution::min_subarray(nums, p), result);
    }

    #[test]
    fn test_1590_example_3() {
        let nums = vec![1, 2, 3];
        let p = 3;

        let result = 0;

        assert_eq!(Solution::min_subarray(nums, p), result);
    }
}

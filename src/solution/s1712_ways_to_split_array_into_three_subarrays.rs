/**
 * [1712] Ways to Split Array Into Three Subarrays
 *
 * A split of an integer array is good if:
 *
 * 	The array is split into three non-empty contiguous subarrays - named left, mid, right respectively from left to right.
 * 	The sum of the elements in left is less than or equal to the sum of the elements in mid, and the sum of the elements in mid is less than or equal to the sum of the elements in right.
 *
 * Given nums, an array of non-negative integers, return the number of good ways to split nums. As the number may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: nums = [1,1,1]
 * Output: 1
 * Explanation: The only good way to split nums is [1] [1] [1].
 * Example 2:
 *
 * Input: nums = [1,2,2,2,5,0]
 * Output: 3
 * Explanation: There are three good ways of splitting nums:
 * [1] [2] [2,2,5,0]
 * [1] [2,2] [2,5,0]
 * [1,2] [2,2] [5,0]
 *
 * Example 3:
 *
 * Input: nums = [3,2,1]
 * Output: 0
 * Explanation: There is no good way to split nums.
 *  
 * Constraints:
 *
 * 	3 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ways-to-split-array-into-three-subarrays/
// discuss: https://leetcode.com/problems/ways-to-split-array-into-three-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
const MOD: usize = 1_000_000_007;

impl Solution {
    // Credit: https://leetcode.com/problems/ways-to-split-array-into-three-subarrays/solutions/2808581/rust-solution-using-binary-search/
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut memo = vec![0; n + 1];
        for i in 0..n {
            memo[i + 1] = nums[i] as isize + memo[i];
        }

        for i in 2..n {
            let lv = memo[i - 1];
            if memo[n - 1] - lv < lv {
                continue;
            }
            let mut left = i;
            let mut right = n + 1;
            while left < right {
                let mid = (left + right) / 2;
                let cv = memo[mid] - lv;
                if lv <= cv {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            if left == n {
                continue;
            }
            let min_ci = left;
            let min_cv = memo[min_ci] - lv;
            if min_cv < lv {
                continue;
            }

            let max_rv = memo[n] - memo[min_ci];
            if max_rv < min_cv {
                continue;
            }

            let mut left = min_ci;
            let mut right = n;
            while left + 1 < right {
                let mid = (left + right) / 2;
                let rv = memo[n] - memo[mid];
                let cv = memo[mid] - lv;
                if cv <= rv {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            let max_ci = left;
            result += max_ci - min_ci + 1;
            result %= MOD;
        }

        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1712_example_1() {
        let nums = vec![1, 1, 1];

        let result = 1;

        assert_eq!(Solution::ways_to_split(nums), result);
    }

    #[test]
    fn test_1712_example_2() {
        let nums = vec![1, 2, 2, 2, 5, 0];

        let result = 3;

        assert_eq!(Solution::ways_to_split(nums), result);
    }

    #[test]
    fn test_1712_example_3() {
        let nums = vec![3, 2, 1];

        let result = 0;

        assert_eq!(Solution::ways_to_split(nums), result);
    }
}

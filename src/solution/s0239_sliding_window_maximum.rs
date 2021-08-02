/**
 * [239] Sliding Window Maximum
 *
 * You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.
 * Return the max sliding window.
 *  
 * Example 1:
 *
 * Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
 * Output: [3,3,5,5,6,7]
 * Explanation:
 * Window position                Max
 * ---------------               -----
 * [1  3  -1] -3  5  3  6  7       3
 *  1 [3  -1  -3] 5  3  6  7       3
 *  1  3 [-1  -3  5] 3  6  7       5
 *  1  3  -1 [-3  5  3] 6  7       5
 *  1  3  -1  -3 [5  3  6] 7       6
 *  1  3  -1  -3  5 [3  6  7]      7
 *
 * Example 2:
 *
 * Input: nums = [1], k = 1
 * Output: [1]
 *
 * Example 3:
 *
 * Input: nums = [1,-1], k = 1
 * Output: [1,-1]
 *
 * Example 4:
 *
 * Input: nums = [9,11], k = 2
 * Output: [11]
 *
 * Example 5:
 *
 * Input: nums = [4,-2], k = 2
 * Output: [4]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^5
 * 	-10^4 <= nums[i] <= 10^4
 * 	1 <= k <= nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sliding-window-maximum/
// discuss: https://leetcode.com/problems/sliding-window-maximum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sliding-window-maximum/discuss/267853/Rust-4ms-deque
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut result = Vec::with_capacity(n - k + 1);
        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            while let Some(&id) = q.front() {
                if id + k <= i {
                    q.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&id) = q.back() {
                if nums[id] <= nums[i] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(i);
            if i >= k - 1 {
                if let Some(&id) = q.front() {
                    result.push(nums[id]);
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
    fn test_0239_example_1() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let k = 3;
        let result = vec![3, 3, 5, 5, 6, 7];

        assert_eq!(Solution::max_sliding_window(nums, k), result);
    }

    #[test]
    fn test_0239_example_2() {
        let nums = vec![1];
        let k = 1;
        let result = vec![1];

        assert_eq!(Solution::max_sliding_window(nums, k), result);
    }

    #[test]
    fn test_0239_example_3() {
        let nums = vec![1, -1];
        let k = 1;
        let result = vec![1, -1];

        assert_eq!(Solution::max_sliding_window(nums, k), result);
    }

    #[test]
    fn test_0239_example_4() {
        let nums = vec![9, 11];
        let k = 2;
        let result = vec![11];

        assert_eq!(Solution::max_sliding_window(nums, k), result);
    }

    #[test]
    fn test_0239_example_5() {
        let nums = vec![4, -2];
        let k = 2;
        let result = vec![4];

        assert_eq!(Solution::max_sliding_window(nums, k), result);
    }
}

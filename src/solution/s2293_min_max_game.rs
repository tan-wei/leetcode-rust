/**
 * [2293] Min Max Game
 *
 * You are given a 0-indexed integer array nums whose length is a power of 2.
 * Apply the following algorithm on nums:
 * <ol>
 * 	Let n be the length of nums. If n == 1, end the process. Otherwise, create a new 0-indexed integer array newNums of length n / 2.
 * 	For every even index i where 0 <= i < n / 2, assign the value of newNums[i] as min(nums[2 * i], nums[2 * i + 1]).
 * 	For every odd index i where 0 <= i < n / 2, assign the value of newNums[i] as max(nums[2 * i], nums[2 * i + 1]).
 * 	Replace the array nums with newNums.
 * 	Repeat the entire process starting from step 1.
 * </ol>
 * Return the last number that remains in nums after applying the algorithm.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/04/13/example1drawio-1.png" style="width: 500px; height: 240px;" />
 * Input: nums = [1,3,5,2,4,8,2,2]
 * Output: 1
 * Explanation: The following arrays are the results of applying the algorithm repeatedly.
 * First: nums = [1,5,4,2]
 * Second: nums = [1,4]
 * Third: nums = [1]
 * 1 is the last remaining number, so we return 1.
 *
 * Example 2:
 *
 * Input: nums = [3]
 * Output: 3
 * Explanation: 3 is already the last remaining number, so we return 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 1024
 * 	1 <= nums[i] <= 10^9
 * 	nums.length is a power of 2.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/min-max-game/
// discuss: https://leetcode.com/problems/min-max-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut n = nums.len() / 2;

        while n > 0 {
            for i in 0..n {
                if i % 2 == 0 {
                    nums[i] = std::cmp::min(nums[2 * i], nums[2 * i + 1]);
                } else {
                    nums[i] = std::cmp::max(nums[2 * i], nums[2 * i + 1]);
                }
            }
            n /= 2;
        }

        nums[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2293_example_1() {
        let nums = vec![1, 3, 5, 2, 4, 8, 2, 2];

        let result = 1;

        assert_eq!(Solution::min_max_game(nums), result);
    }

    #[test]
    fn test_2293_example_2() {
        let nums = vec![3];

        let result = 3;

        assert_eq!(Solution::min_max_game(nums), result);
    }
}

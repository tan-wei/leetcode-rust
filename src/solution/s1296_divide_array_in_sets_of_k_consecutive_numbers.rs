/**
 * [1296] Divide Array in Sets of K Consecutive Numbers
 *
 * Given an array of integers nums and a positive integer k, check whether it is possible to divide this array into sets of k consecutive numbers.
 * Return true if it is possible. Otherwise, return false.
 *
 * Example 1:
 *
 * Input: nums = [1,2,3,3,4,4,5,6], k = 4
 * Output: true
 * Explanation: Array can be divided into [1,2,3,4] and [3,4,5,6].
 *
 * Example 2:
 *
 * Input: nums = [3,2,1,2,3,4,3,4,5,9,10,11], k = 3
 * Output: true
 * Explanation: Array can be divided into [1,2,3] , [2,3,4] , [3,4,5] and [9,10,11].
 *
 * Example 3:
 *
 * Input: nums = [1,2,3,4], k = 3
 * Output: false
 * Explanation: Each array should be divided in subarrays of size 3.
 *
 *
 * Constraints:
 *
 * 	1 <= k <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 *
 *
 * Note: This question is the same as 846: <a href="https://leetcode.com/problems/hand-of-straights/" target="_blank">https://leetcode.com/problems/hand-of-straights/</a>
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/
// discuss: https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        if nums.len() % k != 0 {
            return false;
        }
        let mut map = std::collections::HashMap::new();
        for v in nums {
            *map.entry(v).or_insert(0) += 1;
        }

        let mut arr = map.into_iter().collect::<Vec<(i32, i32)>>();
        arr.sort();
        let mut que = arr
            .into_iter()
            .collect::<std::collections::VecDeque<(i32, i32)>>();

        while !que.is_empty() {
            if que.len() < k {
                return false;
            }

            let mut last = que[0].0 - 1;
            let mut memo = vec![];
            for i in 0..k {
                if que[i].0 != last + 1 {
                    return false;
                }
                if que[i].1 == 1 {
                    memo.push(i);
                }
                que[i].1 -= 1;
                last = que[i].0;
            }

            if memo.is_empty() {
                continue;
            }
            let mut last = memo[0];
            for i in 1..memo.len() {
                if last + 1 != memo[i] {
                    return false;
                }
                last = memo[i];
            }
            for _ in memo {
                que.pop_front();
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1296_example_1() {
        let nums = vec![1, 2, 3, 3, 4, 4, 5, 6];
        let k = 4;
        let result = true;

        assert_eq!(Solution::is_possible_divide(nums, k), result);
    }

    #[test]
    fn test_1296_example_2() {
        let nums = vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11];
        let k = 3;
        let result = true;

        assert_eq!(Solution::is_possible_divide(nums, k), result);
    }

    #[test]
    fn test_1296_example_3() {
        let nums = vec![1, 2, 3, 4];
        let k = 3;
        let result = false;

        assert_eq!(Solution::is_possible_divide(nums, k), result);
    }
}

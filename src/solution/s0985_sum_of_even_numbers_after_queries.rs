/**
 * [0985] Sum of Even Numbers After Queries
 *
 * You are given an integer array nums and an array queries where queries[i] = [vali, indexi].
 * For each query i, first, apply nums[indexi] = nums[indexi] + vali, then print the sum of the even values of nums.
 * Return an integer array answer where answer[i] is the answer to the i^th query.
 *  
 * Example 1:
 *
 * Input: nums = [1,2,3,4], queries = [[1,0],[-3,1],[-4,0],[2,3]]
 * Output: [8,6,2,4]
 * Explanation: At the beginning, the array is [1,2,3,4].
 * After adding 1 to nums[0], the array is [2,2,3,4], and the sum of even values is 2 + 2 + 4 = 8.
 * After adding -3 to nums[1], the array is [2,-1,3,4], and the sum of even values is 2 + 4 = 6.
 * After adding -4 to nums[0], the array is [-2,-1,3,4], and the sum of even values is -2 + 4 = 2.
 * After adding 2 to nums[3], the array is [-2,-1,3,6], and the sum of even values is -2 + 6 = 4.
 *
 * Example 2:
 *
 * Input: nums = [1], queries = [[4,0]]
 * Output: [0]
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10^4
 * 	-10^4 <= nums[i] <= 10^4
 * 	1 <= queries.length <= 10^4
 * 	-10^4 <= vali <= 10^4
 * 	0 <= indexi < nums.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-even-numbers-after-queries/
// discuss: https://leetcode.com/problems/sum-of-even-numbers-after-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/sum-of-even-numbers-after-queries/solutions/2603972/rust-keeping-sum/
    pub fn sum_even_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        let mut result = vec![];
        let mut sum = nums.iter().filter(|&x| x % 2 == 0).sum();
        for query in queries {
            let index = query[1] as usize;
            let val = query[0];
            match (nums[index], nums[index] + val) {
                (x, y) if x % 2 == 0 && y % 2 == 0 => sum += val,
                (x, y) if x % 2 == 0 && y % 2 != 0 => sum -= x,
                (x, y) if x % 2 != 0 && y % 2 == 0 => sum += y,
                _ => (),
            }
            nums[index] += val;
            result.push(sum);
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0985_example_1() {
        let nums = vec![1, 2, 3, 4];
        let queries = vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]];
        let result = vec![8, 6, 2, 4];

        assert_eq!(Solution::sum_even_after_queries(nums, queries), result);
    }

    #[test]
    fn test_0985_example_2() {
        let nums = vec![1];
        let queries = vec![vec![4, 0]];
        let result = vec![0];

        assert_eq!(Solution::sum_even_after_queries(nums, queries), result);
    }
}

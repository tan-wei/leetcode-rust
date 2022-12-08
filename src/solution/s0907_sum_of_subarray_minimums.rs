/**
 * [0907] Sum of Subarray Minimums
 *
 * Given an array of integers arr, find the sum of min(b), where b ranges over every (contiguous) subarray of arr. Since the answer may be large, return the answer modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: arr = [3,1,2,4]
 * Output: 17
 * Explanation:
 * Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4].
 * Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
 * Sum is 17.
 *
 * Example 2:
 *
 * Input: arr = [11,81,94,43,3]
 * Output: 444
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 3 * 10^4
 * 	1 <= arr[i] <= 3 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sum-of-subarray-minimums/
// discuss: https://leetcode.com/problems/sum-of-subarray-minimums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

const MOD: usize = 1_000_000_007;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        (arr.into_iter()
            .enumerate()
            .scan(vec![(-1, 0, 0)], |s, (i, x)| {
                while s.last().unwrap().0 >= x {
                    s.pop();
                }
                let (_, j, prev) = s.last().unwrap();
                let ans = prev + (i + 1 - j) * x as usize;
                s.push((x, i + 1, ans));
                Some(ans)
            })
            .sum::<usize>()
            % MOD) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0907_example_1() {
        let arr = vec![3, 1, 2, 4];
        let result = 17;

        assert_eq!(Solution::sum_subarray_mins(arr), result);
    }

    #[test]
    fn test_0907_example_2dc() {
        let arr = vec![11, 81, 94, 43, 3];
        let result = 444;

        assert_eq!(Solution::sum_subarray_mins(arr), result);
    }
}

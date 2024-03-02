/**
 * [1521] Find a Value of a Mysterious Function Closest to Target
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/09/change.png" style="width: 635px; height: 312px;" />
 * Winston was given the above mysterious function func. He has an integer array arr and an integer target and he wants to find the values l and r that make the value |func(arr, l, r) - target| minimum possible.
 * Return the minimum possible value of |func(arr, l, r) - target|.
 * Notice that func should be called with the values l and r where 0 <= l, r < arr.length.
 *  
 * Example 1:
 *
 * Input: arr = [9,12,3,7,15], target = 5
 * Output: 2
 * Explanation: Calling func with all the pairs of [l,r] = [[0,0],[1,1],[2,2],[3,3],[4,4],[0,1],[1,2],[2,3],[3,4],[0,2],[1,3],[2,4],[0,3],[1,4],[0,4]], Winston got the following results [9,12,3,7,15,8,0,3,7,0,0,3,0,0,0]. The value closest to 5 is 7 and 3, thus the minimum difference is 2.
 *
 * Example 2:
 *
 * Input: arr = [1000000,1000000,1000000], target = 1
 * Output: 999999
 * Explanation: Winston called the func with all possible values of [l,r] and he always got 1000000, thus the min difference is 999999.
 *
 * Example 3:
 *
 * Input: arr = [1,2,4,8,16], target = 0
 * Output: 0
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^5
 * 	1 <= arr[i] <= 10^6
 * 	0 <= target <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-a-value-of-a-mysterious-function-closest-to-target/
// discuss: https://leetcode.com/problems/find-a-value-of-a-mysterious-function-closest-to-target/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let len = arr.len();
        let mut tab = vec![vec![]; 30];
        for (i, &item0) in arr.iter().enumerate().take(len) {
            for (j, item) in tab.iter_mut().enumerate().take(30) {
                if (item0 >> j) & 1 == 1 {
                    item.push(i);
                }
            }
        }

        let mut dp = vec![0; len];
        let mut result = (arr[0] - target).abs();

        for i in (0..len).rev() {
            for j in 0..30 {
                if (arr[i] >> j) & 1 == 0 {
                    while !tab[j].is_empty() && tab[j].last().unwrap() > &i {
                        dp[*tab[j].last().unwrap()] -= 1 << j;
                        tab[j].pop();
                    }
                }
            }
            dp[i] = arr[i];
            let mut l = i;
            let mut r = len;
            while l < r {
                let m = l + (r - l) / 2;
                if dp[m] > target {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            if l == len {
                result = result.min((target - dp[l - 1]).abs());
            } else if l == i {
                result = result.min((target - dp[l]).abs());
            } else {
                result = result.min((target - dp[l]).abs().min((target - dp[l - 1]).abs()));
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
    fn test_1521_example_1() {
        let arr = vec![9, 12, 3, 7, 15];
        let target = 5;

        let result = 2;

        assert_eq!(Solution::closest_to_target(arr, target), result);
    }

    #[test]
    fn test_1521_example_2() {
        let arr = vec![1000000, 1000000, 1000000];
        let target = 1;

        let result = 999999;

        assert_eq!(Solution::closest_to_target(arr, target), result);
    }

    #[test]
    fn test_1521_example_3() {
        let arr = vec![1, 2, 4, 8, 16];
        let target = 0;

        let result = 0;

        assert_eq!(Solution::closest_to_target(arr, target), result);
    }
}

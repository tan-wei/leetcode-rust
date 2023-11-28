/**
 * [1399] Count Largest Group
 *
 * You are given an integer n.
 * Each number from 1 to n is grouped according to the sum of its digits.
 * Return the number of groups that have the largest size.
 *  
 * Example 1:
 *
 * Input: n = 13
 * Output: 4
 * Explanation: There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
 * [1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9].
 * There are 4 groups with largest size.
 *
 * Example 2:
 *
 * Input: n = 2
 * Output: 2
 * Explanation: There are 2 groups [1], [2] of size 1.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-largest-group/
// discuss: https://leetcode.com/problems/count-largest-group/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut h = [0; 37];
        let mut max = 1;
        for i in 1..=n {
            let mut j = i;
            let mut c = 0;
            while j > 0 {
                c += j % 10;
                j /= 10;
            }
            h[c as usize] += 1;
            max = std::cmp::max(max, h[c as usize]);
        }
        h.iter().filter(|&x| *x == max).count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1399_example_1() {
        let n = 13;

        let result = 4;

        assert_eq!(Solution::count_largest_group(n), result);
    }

    #[test]
    fn test_1399_example_2() {
        let n = 2;

        let result = 2;

        assert_eq!(Solution::count_largest_group(n), result);
    }
}

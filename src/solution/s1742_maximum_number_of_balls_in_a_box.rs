/**
 * [1742] Maximum Number of Balls in a Box
 *
 * You are working in a ball factory where you have n balls numbered from lowLimit up to highLimit inclusive (i.e., n == highLimit - lowLimit + 1), and an infinite number of boxes numbered from 1 to infinity.
 * Your job at this factory is to put each ball in the box with a number equal to the sum of digits of the ball's number. For example, the ball number 321 will be put in the box number 3 + 2 + 1 = 6 and the ball number 10 will be put in the box number 1 + 0 = 1.
 * Given two integers lowLimit and highLimit, return the number of balls in the box with the most balls.
 *  
 * Example 1:
 *
 * Input: lowLimit = 1, highLimit = 10
 * Output: 2
 * Explanation:
 * Box Number:  1 2 3 4 5 6 7 8 9 10 11 ...
 * Ball Count:  2 1 1 1 1 1 1 1 1 0  0  ...
 * Box 1 has the most number of balls with 2 balls.
 * Example 2:
 *
 * Input: lowLimit = 5, highLimit = 15
 * Output: 2
 * Explanation:
 * Box Number:  1 2 3 4 5 6 7 8 9 10 11 ...
 * Ball Count:  1 1 1 1 2 2 1 1 1 0  0  ...
 * Boxes 5 and 6 have the most number of balls with 2 balls in each.
 *
 * Example 3:
 *
 * Input: lowLimit = 19, highLimit = 28
 * Output: 2
 * Explanation:
 * Box Number:  1 2 3 4 5 6 7 8 9 10 11 12 ...
 * Ball Count:  0 1 1 1 1 1 1 1 1 2  0  0  ...
 * Box 10 has the most number of balls with 2 balls.
 *
 *  
 * Constraints:
 *
 * 	1 <= lowLimit <= highLimit <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-balls-in-a-box/
// discuss: https://leetcode.com/problems/maximum-number-of-balls-in-a-box/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut boxes = [0; 46];
        let sum_of_digits = |mut x: i32| {
            let mut s = 0;
            while x > 0 {
                s += x % 10;
                x /= 10;
            }
            s as usize
        };

        (low_limit..=high_limit).for_each(|val| boxes[sum_of_digits(val)] += 1);
        boxes.iter().fold(0, |max_val, &x| max_val.max(x))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1742_example_1() {
        let low_limit = 1;
        let high_limit = 10;

        let result = 2;

        assert_eq!(Solution::count_balls(low_limit, high_limit), result);
    }

    #[test]
    fn test_1742_example_2() {
        let low_limit = 5;
        let high_limit = 15;

        let result = 2;

        assert_eq!(Solution::count_balls(low_limit, high_limit), result);
    }

    #[test]
    fn test_1742_example_3() {
        let low_limit = 19;
        let high_limit = 28;

        let result = 2;

        assert_eq!(Solution::count_balls(low_limit, high_limit), result);
    }
}

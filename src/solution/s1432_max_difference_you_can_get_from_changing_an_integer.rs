/**
 * [1432] Max Difference You Can Get From Changing an Integer
 *
 * You are given an integer num. You will apply the following steps exactly two times:
 *
 * 	Pick a digit x (0 <= x <= 9).
 * 	Pick another digit y (0 <= y <= 9). The digit y can be equal to x.
 * 	Replace all the occurrences of x in the decimal representation of num by y.
 * 	The new integer cannot have any leading zeros, also the new integer cannot be 0.
 *
 * Let a and b be the results of applying the operations to num the first and second times, respectively.
 * Return the max difference between a and b.
 *  
 * Example 1:
 *
 * Input: num = 555
 * Output: 888
 * Explanation: The first time pick x = 5 and y = 9 and store the new integer in a.
 * The second time pick x = 5 and y = 1 and store the new integer in b.
 * We have now a = 999 and b = 111 and max difference = 888
 *
 * Example 2:
 *
 * Input: num = 9
 * Output: 8
 * Explanation: The first time pick x = 9 and y = 9 and store the new integer in a.
 * The second time pick x = 9 and y = 1 and store the new integer in b.
 * We have now a = 9 and b = 1 and max difference = 8
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/
// discuss: https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut a = vec![];
        let mut num = num;

        while num > 9 {
            a.push(num % 10);
            num /= 10;
        }

        a.push(num);
        a.reverse();

        let mut b = a.clone();
        let mut x = -1;
        let mut y = (-1, 0);

        for i in 0..a.len() {
            if x != -1 {
                if a[i] == x {
                    a[i] = 9;
                }
            } else {
                if a[i] != 9 {
                    x = a[i];
                    a[i] = 9;
                }
            }
        }

        if b[0] != 1 {
            y.0 = b[0];
            y.1 = 1;
            b[0] = 1;
        }

        for i in 1..b.len() {
            if y.0 != -1 {
                if b[i] == y.0 {
                    b[i] = y.1;
                }
            } else {
                if b[i] > 1 {
                    y.0 = b[i];
                    b[i] = 0;
                }
            }
        }

        let mut result = 0;

        for i in 0..a.len() - 1 {
            result += a[i] - b[i];
            result *= 10;
        }

        result + a.last().unwrap() - b.last().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1432_example_1() {
        let num = 555;

        let result = 888;

        assert_eq!(Solution::max_diff(num), result);
    }

    #[test]
    fn test_1432_example_2() {
        let num = 9;

        let result = 8;

        assert_eq!(Solution::max_diff(num), result);
    }
}

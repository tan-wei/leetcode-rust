/**
 * [0390] Elimination Game
 *
 * You have a list arr of all integers in the range [1, n] sorted in a strictly increasing order. Apply the following algorithm on arr:
 *
 * 	Starting from left to right, remove the first number and every other number afterward until you reach the end of the list.
 * 	Repeat the previous step again, but this time from right to left, remove the rightmost number and every other number from the remaining numbers.
 * 	Keep repeating the steps again, alternating left to right and right to left, until a single number remains.
 *
 * Given the integer n, return the last number that remains in arr.
 *  
 * Example 1:
 *
 * Input: n = 9
 * Output: 6
 * Explanation:
 * arr = [<u>1</u>, 2, <u>3</u>, 4, <u>5</u>, 6, <u>7</u>, 8, <u>9</u>]
 * arr = [2, <u>4</u>, 6, <u>8</u>]
 * arr = [<u>2</u>, 6]
 * arr = [6]
 *
 * Example 2:
 *
 * Input: n = 1
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/elimination-game/
// discuss: https://leetcode.com/problems/elimination-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        (0..)
            .try_fold((n, true, 1, 1), |(n, forward, step, next), _| {
                if n == 1 {
                    Err(next)
                } else {
                    Ok((
                        n / 2,
                        !forward,
                        step * 2,
                        next + (forward || n % 2 == 1) as i32 * step,
                    ))
                }
            })
            .unwrap_err()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0390_example_1() {
        let n = 9;
        let result = 6;

        assert_eq!(Solution::last_remaining(n), result);
    }

    #[test]
    fn test_0390_example_2() {
        let n = 1;
        let result = 1;

        assert_eq!(Solution::last_remaining(n), result);
    }
}

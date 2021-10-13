/**
 * [374] Guess Number Higher or Lower
 *
 * We are playing the Guess Game. The game is as follows:
 * I pick a number from 1 to n. You have to guess which number I picked.
 * Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
 * You call a pre-defined API int guess(int num), which returns 3 possible results:
 *
 * 	-1: The number I picked is lower than your guess (i.e. pick < num).
 * 	1: The number I picked is higher than your guess (i.e. pick > num).
 * 	0: The number I picked is equal to your guess (i.e. pick == num).
 *
 * Return the number that I picked.
 *  
 * Example 1:
 * Input: n = 10, pick = 6
 * Output: 6
 * Example 2:
 * Input: n = 1, pick = 1
 * Output: 1
 * Example 3:
 * Input: n = 2, pick = 1
 * Output: 1
 * Example 4:
 * Input: n = 2, pick = 2
 * Output: 2
 *  
 * Constraints:
 *
 * 	1 <= n <= 2^31 - 1
 * 	1 <= pick <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/guess-number-higher-or-lower/
// discuss: https://leetcode.com/problems/guess-number-higher-or-lower/discuss/?currentPage=1&orderBy=most_votes&query=
static mut PICK: i32 = 0;
unsafe fn guess(num: i32) -> i32 {
    match num.cmp(&PICK) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Less => 1,
    }
}

// submission codes start here

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guess_number(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let (mut left, mut right) = (1, n);
        loop {
            let mid = left + (right - left) / 2;
            match guess(mid) {
                -1 => {
                    right = mid - 1;
                }
                1 => {
                    left = mid + 1;
                }
                _ => {
                    return mid;
                }
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0374_example_1() {
        unsafe {
            PICK = 6;
        }
        let n = 10;

        unsafe {
            assert_eq!(Solution::guess_number(n), PICK);
        }
    }

    #[test]
    fn test_0374_example_2() {
        unsafe {
            PICK = 1;
        }
        let n = 1;
        unsafe {
            assert_eq!(Solution::guess_number(n), PICK);
        }
    }

    #[test]
    fn test_0374_example_3() {
        unsafe {
            PICK = 1;
        }
        let n = 2;

        unsafe {
            assert_eq!(Solution::guess_number(n), PICK);
        }
    }
}

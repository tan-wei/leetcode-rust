/**
 * [2523] Closest Prime Numbers in Range
 *
 * Given two positive integers left and right, find the two integers num1 and num2 such that:
 *
 * 	left <= num1 < num2 <= right .
 * 	Both num1 and num2 are <span data-keyword="prime-number">prime numbers</span>.
 * 	num2 - num1 is the minimum amongst all other pairs satisfying the above conditions.
 *
 * Return the positive integer array ans = [num1, num2]. If there are multiple pairs satisfying these conditions, return the one with the smallest num1 value. If no such numbers exist, return [-1, -1].
 *  
 * Example 1:
 *
 * Input: left = 10, right = 19
 * Output: [11,13]
 * Explanation: The prime numbers between 10 and 19 are 11, 13, 17, and 19.
 * The closest gap between any pair is 2, which can be achieved by [11,13] or [17,19].
 * Since 11 is smaller than 17, we return the first pair.
 *
 * Example 2:
 *
 * Input: left = 4, right = 6
 * Output: [-1,-1]
 * Explanation: There exists only one prime number in the given range, so the conditions cannot be satisfied.
 *
 *  
 * Constraints:
 *
 * 	1 <= left <= right <= 10^6
 *
 *  
 * <style type="text/css">.spoilerbutton {display:block; border:dashed; padding: 0px 0px; margin:10px 0px; font-size:150%; font-weight: bold; color:#000000; background-color:cyan; outline:0;
 * }
 * .spoiler {overflow:hidden;}
 * .spoiler > div {-webkit-transition: all 0s ease;-moz-transition: margin 0s ease;-o-transition: all 0s ease;transition: margin 0s ease;}
 * .spoilerbutton[value="Show Message"] + .spoiler > div {margin-top:-500%;}
 * .spoilerbutton[value="Hide Message"] + .spoiler {padding:5px;}
 * </style>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/closest-prime-numbers-in-range/
// discuss: https://leetcode.com/problems/closest-prime-numbers-in-range/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2523_example_1() {
        let left = 10;
        let right = 19;

        let result = vec![11, 13];

        assert_eq!(Solution::closest_primes(left, right), result);
    }

    #[test]
    #[ignore]
    fn test_2523_example_2() {
        let left = 4;
        let right = 6;

        let result = vec![-1, -1];

        assert_eq!(Solution::closest_primes(left, right), result);
    }
}

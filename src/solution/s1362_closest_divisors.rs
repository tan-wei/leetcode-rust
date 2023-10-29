/**
 * [1362] Closest Divisors
 *
 * Given an integer num, find the closest two integers in absolute difference whose product equals num + 1 or num + 2.
 * Return the two integers in any order.
 *  
 * Example 1:
 *
 * Input: num = 8
 * Output: [3,3]
 * Explanation: For num + 1 = 9, the closest divisors are 3 &amp; 3, for num + 2 = 10, the closest divisors are 2 &amp; 5, hence 3 &amp; 3 is chosen.
 *
 * Example 2:
 *
 * Input: num = 123
 * Output: [5,25]
 *
 * Example 3:
 *
 * Input: num = 999
 * Output: [40,25]
 *
 *  
 * Constraints:
 *
 * 	1 <= num <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/closest-divisors/
// discuss: https://leetcode.com/problems/closest-divisors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let mut abs = 10i32.pow(9) + 10;
        let mut result = vec![];
        for base in (num + 1)..=(num + 2) {
            let mut i = 1;
            while i * i <= base {
                if base % i == 0 {
                    let j = base / i;
                    if (j - i).abs() < abs {
                        abs = (j - i).abs();
                        result = vec![i, j];
                    }
                }
                i += 1;
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
    fn test_1362_example_1() {
        let num = 8;

        let result = vec![3, 3];

        assert_eq_sorted!(Solution::closest_divisors(num), result);
    }

    #[test]
    fn test_1362_example_2() {
        let num = 123;

        let result = vec![5, 25];

        assert_eq_sorted!(Solution::closest_divisors(num), result);
    }

    #[test]
    fn test_1362_example_3() {
        let num = 999;

        let result = vec![40, 25];

        assert_eq_sorted!(Solution::closest_divisors(num), result);
    }
}

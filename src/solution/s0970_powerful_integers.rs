/**
 * [0970] Powerful Integers
 *
 * Given three integers x, y, and bound, return a list of all the powerful integers that have a value less than or equal to bound.
 * An integer is powerful if it can be represented as x^i + y^j for some integers i >= 0 and j >= 0.
 * You may return the answer in any order. In your answer, each value should occur at most once.
 *  
 * Example 1:
 *
 * Input: x = 2, y = 3, bound = 10
 * Output: [2,3,4,5,7,9,10]
 * Explanation:
 * 2 = 2^0 + 3^0
 * 3 = 2^1 + 3^0
 * 4 = 2^0 + 3^1
 * 5 = 2^1 + 3^1
 * 7 = 2^2 + 3^1
 * 9 = 2^3 + 3^0
 * 10 = 2^0 + 3^2
 *
 * Example 2:
 *
 * Input: x = 3, y = 5, bound = 15
 * Output: [2,4,6,8,10,14]
 *
 *  
 * Constraints:
 *
 * 	1 <= x, y <= 100
 * 	0 <= bound <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/powerful-integers/
// discuss: https://leetcode.com/problems/powerful-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        std::iter::successors(Some(1), |n| Some(n * x).filter(|&xi| x > 1 && xi < bound))
            .flat_map(|xi| {
                std::iter::successors(Some(1), move |n| {
                    Some(n * y).filter(|&yj| y > 1 && yj <= bound - xi)
                })
                .filter_map(move |yj| Some(xi + yj).filter(|&sum| sum <= bound))
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0970_example_1() {
        let x = 2;
        let y = 3;
        let bound = 10;
        let result = vec![2, 3, 4, 5, 7, 9, 10];

        assert_eq_sorted!(Solution::powerful_integers(x, y, bound), result);
    }

    #[test]
    fn test_0970_example_2() {
        let x = 3;
        let y = 5;
        let bound = 15;
        let result = vec![2, 4, 6, 8, 10, 14];

        assert_eq_sorted!(Solution::powerful_integers(x, y, bound), result);
    }
}

/**
 * [1711] Count Good Meals
 *
 * A good meal is a meal that contains exactly two different food items with a sum of deliciousness equal to a power of two.
 * You can pick any two different foods to make a good meal.
 * Given an array of integers deliciousness where deliciousness[i] is the deliciousness of the i^​​​​​​th​​​​​​​​ item of food, return the number of different good meals you can make from this list modulo 10^9 + 7.
 * Note that items with different indices are considered different even if they have the same deliciousness value.
 *  
 * Example 1:
 *
 * Input: deliciousness = [1,3,5,7,9]
 * Output: 4
 * Explanation: The good meals are (1,3), (1,7), (3,5) and, (7,9).
 * Their respective sums are 4, 8, 8, and 16, all of which are powers of 2.
 *
 * Example 2:
 *
 * Input: deliciousness = [1,1,1,3,3,3,7]
 * Output: 15
 * Explanation: The good meals are (1,1) with 3 ways, (1,3) with 9 ways, and (1,7) with 3 ways.
 *  
 * Constraints:
 *
 * 	1 <= deliciousness.length <= 10^5
 * 	0 <= deliciousness[i] <= 2^20
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-good-meals/
// discuss: https://leetcode.com/problems/count-good-meals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let (mut prefix, pow, mut result) = (
            std::collections::HashMap::new(),
            (0..22).map(|i| 2_i32.pow(i)).collect::<Vec<_>>(),
            0,
        );

        for d in deliciousness {
            for &p in &pow {
                result = (result + prefix.get(&(p - d)).copied().unwrap_or(0)) % (1e9 as i32 + 7);
            }
            *prefix.entry(d).or_insert(0) += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1711_example_1() {
        let deliciousness = vec![1, 3, 5, 7, 9];

        let result = 4;

        assert_eq!(Solution::count_pairs(deliciousness), result);
    }

    #[test]
    fn test_1711_example_2() {
        let deliciousness = vec![1, 1, 1, 3, 3, 3, 7];

        let result = 15;

        assert_eq!(Solution::count_pairs(deliciousness), result);
    }
}

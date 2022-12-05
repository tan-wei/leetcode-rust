/**
 * [0904] Fruit Into Baskets
 *
 * You are visiting a farm that has a single row of fruit trees arranged from left to right. The trees are represented by an integer array fruits where fruits[i] is the type of fruit the i^th tree produces.
 * You want to collect as much fruit as possible. However, the owner has some strict rules that you must follow:
 *
 * 	You only have two baskets, and each basket can only hold a single type of fruit. There is no limit on the amount of fruit each basket can hold.
 * 	Starting from any tree of your choice, you must pick exactly one fruit from every tree (including the start tree) while moving to the right. The picked fruits must fit in one of your baskets.
 * 	Once you reach a tree with fruit that cannot fit in your baskets, you must stop.
 *
 * Given the integer array fruits, return the maximum number of fruits you can pick.
 *  
 * Example 1:
 *
 * Input: fruits = [<u>1,2,1</u>]
 * Output: 3
 * Explanation: We can pick from all 3 trees.
 *
 * Example 2:
 *
 * Input: fruits = [0,<u>1,2,2</u>]
 * Output: 3
 * Explanation: We can pick from trees [1,2,2].
 * If we had started at the first tree, we would only pick from trees [0,1].
 *
 * Example 3:
 *
 * Input: fruits = [1,<u>2,3,2,2</u>]
 * Output: 4
 * Explanation: We can pick from trees [2,3,2,2].
 * If we had started at the first tree, we would only pick from trees [1,2].
 *
 *  
 * Constraints:
 *
 * 	1 <= fruits.length <= 10^5
 * 	0 <= fruits[i] < fruits.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fruit-into-baskets/
// discuss: https://leetcode.com/problems/fruit-into-baskets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/fruit-into-baskets/solutions/1661313/rust/
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut frequency = std::collections::HashMap::new();
        let mut window_start = 0_usize;
        let mut result = 0_i32;
        for fruit in &fruits {
            let count = frequency.entry(*fruit).or_insert(0_i32);
            *count += 1;

            while frequency.len() > 2 {
                let fruit_to_be_removed = fruits[window_start];
                match frequency.remove(&fruit_to_be_removed) {
                    Some(v) if v > 1 => {
                        frequency.insert(fruit_to_be_removed, v - 1);
                    }
                    _ => {}
                }
                window_start += 1;
            }

            let mut total = 0;
            for (_, nums) in &frequency {
                total += nums;
            }

            result = result.max(total);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0904_example_1() {
        let fruits = vec![1, 2, 1];
        let result = 3;

        assert_eq!(Solution::total_fruit(fruits), result);
    }

    #[test]
    fn test_0904_example_2() {
        let fruits = vec![0, 1, 2, 2];
        let result = 3;

        assert_eq!(Solution::total_fruit(fruits), result);
    }

    #[test]
    fn test_0904_example_3() {
        let fruits = vec![1, 2, 3, 2, 2];
        let result = 4;

        assert_eq!(Solution::total_fruit(fruits), result);
    }
}

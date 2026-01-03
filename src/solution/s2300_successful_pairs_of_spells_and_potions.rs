/**
 * [2300] Successful Pairs of Spells and Potions
 *
 * You are given two positive integer arrays spells and potions, of length n and m respectively, where spells[i] represents the strength of the i^th spell and potions[j] represents the strength of the j^th potion.
 * You are also given an integer success. A spell and potion pair is considered successful if the product of their strengths is at least success.
 * Return an integer array pairs of length n where pairs[i] is the number of potions that will form a successful pair with the i^th spell.
 *  
 * Example 1:
 *
 * Input: spells = [5,1,3], potions = [1,2,3,4,5], success = 7
 * Output: [4,0,3]
 * Explanation:
 * - 0^th spell: 5 * [1,2,3,4,5] = [5,<u>10</u>,<u>15</u>,<u>20</u>,<u>25</u>]. 4 pairs are successful.
 * - 1^st spell: 1 * [1,2,3,4,5] = [1,2,3,4,5]. 0 pairs are successful.
 * - 2^nd spell: 3 * [1,2,3,4,5] = [3,6,<u>9</u>,<u>12</u>,<u>15</u>]. 3 pairs are successful.
 * Thus, [4,0,3] is returned.
 *
 * Example 2:
 *
 * Input: spells = [3,1,2], potions = [8,5,8], success = 16
 * Output: [2,0,2]
 * Explanation:
 * - 0^th spell: 3 * [8,5,8] = [<u>24</u>,15,<u>24</u>]. 2 pairs are successful.
 * - 1^st spell: 1 * [8,5,8] = [8,5,8]. 0 pairs are successful.
 * - 2^nd spell: 2 * [8,5,8] = [<u>16</u>,10,<u>16</u>]. 2 pairs are successful.
 * Thus, [2,0,2] is returned.
 *
 *  
 * Constraints:
 *
 * 	n == spells.length
 * 	m == potions.length
 * 	1 <= n, m <= 10^5
 * 	1 <= spells[i], potions[i] <= 10^5
 * 	1 <= success <= 10^10
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/successful-pairs-of-spells-and-potions/
// discuss: https://leetcode.com/problems/successful-pairs-of-spells-and-potions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions: Vec<i64> = potions.into_iter().map(|x| x as i64).collect();

        potions.sort_unstable();

        let m = potions.len() as i32;

        spells
            .into_iter()
            .map(|s| {
                let need = (success + s as i64 - 1) / s as i64;
                let i = potions.partition_point(|&p| p < need);
                m - i as i32
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2300_example_1() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;

        let result = vec![4, 0, 3];

        assert_eq!(Solution::successful_pairs(spells, potions, success), result);
    }

    #[test]
    fn test_2300_example_2() {
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;

        let result = vec![2, 0, 2];

        assert_eq!(Solution::successful_pairs(spells, potions, success), result);
    }
}

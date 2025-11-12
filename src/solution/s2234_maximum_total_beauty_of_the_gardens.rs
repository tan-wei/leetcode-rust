/**
 * [2234] Maximum Total Beauty of the Gardens
 *
 * Alice is a caretaker of n gardens and she wants to plant flowers to maximize the total beauty of all her gardens.
 * You are given a 0-indexed integer array flowers of size n, where flowers[i] is the number of flowers already planted in the i^th garden. Flowers that are already planted cannot be removed. You are then given another integer newFlowers, which is the maximum number of flowers that Alice can additionally plant. You are also given the integers target, full, and partial.
 * A garden is considered complete if it has at least target flowers. The total beauty of the gardens is then determined as the sum of the following:
 *
 * 	The number of complete gardens multiplied by full.
 * 	The minimum number of flowers in any of the incomplete gardens multiplied by partial. If there are no incomplete gardens, then this value will be 0.
 *
 * Return the maximum total beauty that Alice can obtain after planting at most newFlowers flowers.
 *  
 * Example 1:
 *
 * Input: flowers = [1,3,1,1], newFlowers = 7, target = 6, full = 12, partial = 1
 * Output: 14
 * Explanation: Alice can plant
 * - 2 flowers in the 0^th garden
 * - 3 flowers in the 1^st garden
 * - 1 flower in the 2^nd garden
 * - 1 flower in the 3^rd garden
 * The gardens will then be [3,6,2,2]. She planted a total of 2 + 3 + 1 + 1 = 7 flowers.
 * There is 1 garden that is complete.
 * The minimum number of flowers in the incomplete gardens is 2.
 * Thus, the total beauty is 1 * 12 + 2 * 1 = 12 + 2 = 14.
 * No other way of planting flowers can obtain a total beauty higher than 14.
 *
 * Example 2:
 *
 * Input: flowers = [2,4,5,3], newFlowers = 10, target = 5, full = 2, partial = 6
 * Output: 30
 * Explanation: Alice can plant
 * - 3 flowers in the 0^th garden
 * - 0 flowers in the 1^st garden
 * - 0 flowers in the 2^nd garden
 * - 2 flowers in the 3^rd garden
 * The gardens will then be [5,4,5,5]. She planted a total of 3 + 0 + 0 + 2 = 5 flowers.
 * There are 3 gardens that are complete.
 * The minimum number of flowers in the incomplete gardens is 4.
 * Thus, the total beauty is 3 * 2 + 4 * 6 = 6 + 24 = 30.
 * No other way of planting flowers can obtain a total beauty higher than 30.
 * Note that Alice could make all the gardens complete but in this case, she would obtain a lower total beauty.
 *
 *  
 * Constraints:
 *
 * 	1 <= flowers.length <= 10^5
 * 	1 <= flowers[i], target <= 10^5
 * 	1 <= newFlowers <= 10^10
 * 	1 <= full, partial <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-total-beauty-of-the-gardens/
// discuss: https://leetcode.com/problems/maximum-total-beauty-of-the-gardens/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_beauty(
        flowers: Vec<i32>,
        new_flowers: i64,
        target: i32,
        full: i32,
        partial: i32,
    ) -> i64 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2234_example_1() {
        let flowers = vec![1, 3, 1, 1];
        let new_flowers = 7;
        let target = 6;
        let full = 12;
        let partial = 1;

        let result = 14;

        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2234_example_2() {
        let flowers = vec![2, 4, 5, 3];
        let new_flowers = 10;
        let target = 5;
        let full = 2;
        let partial = 6;

        let result = 30;

        assert_eq!(
            Solution::maximum_beauty(flowers, new_flowers, target, full, partial),
            result
        );
    }
}

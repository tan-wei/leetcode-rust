/**
 * [2558] Take Gifts From the Richest Pile
 *
 * You are given an integer array gifts denoting the number of gifts in various piles. Every second, you do the following:
 *
 * 	Choose the pile with the maximum number of gifts.
 * 	If there is more than one pile with the maximum number of gifts, choose any.
 * 	Reduce the number of gifts in the pile to the floor of the square root of the original number of gifts in the pile.
 *
 * Return the number of gifts remaining after k seconds.
 *  
 * Example 1:
 *
 * Input: gifts = [25,64,9,4,100], k = 4
 * Output: 29
 * Explanation:
 * The gifts are taken in the following way:
 * - In the first second, the last pile is chosen and 10 gifts are left behind.
 * - Then the second pile is chosen and 8 gifts are left behind.
 * - After that the first pile is chosen and 5 gifts are left behind.
 * - Finally, the last pile is chosen again and 3 gifts are left behind.
 * The final remaining gifts are [5,8,9,4,3], so the total number of gifts remaining is 29.
 *
 * Example 2:
 *
 * Input: gifts = [1,1,1,1], k = 4
 * Output: 4
 * Explanation:
 * In this case, regardless which pile you choose, you have to leave behind 1 gift in each pile.
 * That is, you can't take any pile with you.
 * So, the total gifts remaining are 4.
 *
 *  
 * Constraints:
 *
 * 	1 <= gifts.length <= 10^3
 * 	1 <= gifts[i] <= 10^9
 * 	1 <= k <= 10^3
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/take-gifts-from-the-richest-pile/
// discuss: https://leetcode.com/problems/take-gifts-from-the-richest-pile/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        (0..k)
            .fold(
                gifts
                    .into_iter()
                    .map(|x| x as i64)
                    .collect::<std::collections::BinaryHeap<i64>>(),
                |mut q, _| {
                    if let Some(x) = q.pop() {
                        q.push((x as f64).sqrt() as i64);
                    }
                    q
                },
            )
            .into_iter()
            .sum::<i64>()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2558_example_1() {
        let gifts = vec![25, 64, 9, 4, 100];
        let k = 4;

        let result = 29;

        assert_eq!(Solution::pick_gifts(gifts, k), result);
    }

    #[test]
    fn test_2558_example_2() {
        let gifts = vec![1, 1, 1, 1];
        let k = 4;

        let result = 4;

        assert_eq!(Solution::pick_gifts(gifts, k), result);
    }
}

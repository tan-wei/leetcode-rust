/**
 * [1423] Maximum Points You Can Obtain from Cards
 *
 * There are several cards arranged in a row, and each card has an associated number of points. The points are given in the integer array cardPoints.
 * In one step, you can take one card from the beginning or from the end of the row. You have to take exactly k cards.
 * Your score is the sum of the points of the cards you have taken.
 * Given the integer array cardPoints and the integer k, return the maximum score you can obtain.
 *  
 * Example 1:
 *
 * Input: cardPoints = [1,2,3,4,5,6,1], k = 3
 * Output: 12
 * Explanation: After the first step, your score will always be 1. However, choosing the rightmost card first will maximize your total score. The optimal strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 12.
 *
 * Example 2:
 *
 * Input: cardPoints = [2,2,2], k = 2
 * Output: 4
 * Explanation: Regardless of which two cards you take, your score will always be 4.
 *
 * Example 3:
 *
 * Input: cardPoints = [9,7,7,9,7,7,9], k = 7
 * Output: 55
 * Explanation: You have to take all the cards. Your score is the sum of points of all cards.
 *
 *  
 * Constraints:
 *
 * 	1 <= cardPoints.length <= 10^5
 * 	1 <= cardPoints[i] <= 10^4
 * 	1 <= k <= cardPoints.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
// discuss: https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        let mut result = 0;
        for i in 0..k {
            result += card_points[i];
        }

        let mut temp = result;
        for i in 0..k {
            temp -= card_points[k - 1 - i];
            temp += card_points[n - 1 - i];
            result = result.max(temp);
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1423_example_1() {
        let card_points = vec![1, 2, 3, 4, 5, 6, 1];
        let k = 3;

        let result = 12;

        assert_eq!(Solution::max_score(card_points, k), result);
    }

    #[test]
    fn test_1423_example_2() {
        let card_points = vec![2, 2, 2];
        let k = 2;

        let result = 4;

        assert_eq!(Solution::max_score(card_points, k), result);
    }

    #[test]
    fn test_1423_example_3() {
        let card_points = vec![9, 7, 7, 9, 7, 7, 9];
        let k = 7;

        let result = 55;

        assert_eq!(Solution::max_score(card_points, k), result);
    }
}

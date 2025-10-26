/**
 * [2212] Maximum Points in an Archery Competition
 *
 * Alice and Bob are opponents in an archery competition. The competition has set the following rules:
 * <ol>
 * 	Alice first shoots numArrows arrows and then Bob shoots numArrows arrows.
 * 	The points are then calculated as follows:
 * 	<ol>
 * 		The target has integer scoring sections ranging from 0 to 11 inclusive.
 * 		For each section of the target with score k (in between 0 to 11), say Alice and Bob have shot ak and bk arrows on that section respectively. If ak >= bk, then Alice takes k points. If ak < bk, then Bob takes k points.
 * 		However, if ak == bk == 0, then nobody takes k points.
 * 	</ol>
 *
 * </ol>
 *
 *
 * 	For example, if Alice and Bob both shot 2 arrows on the section with score 11, then Alice takes 11 points. On the other hand, if Alice shot 0 arrows on the section with score 11 and Bob shot 2 arrows on that same section, then Bob takes 11 points.
 *
 *
 * You are given the integer numArrows and an integer array aliceArrows of size 12, which represents the number of arrows Alice shot on each scoring section from 0 to 11. Now, Bob wants to maximize the total number of points he can obtain.
 * Return the array bobArrows which represents the number of arrows Bob shot on each scoring section from 0 to 11. The sum of the values in bobArrows should equal numArrows.
 * If there are multiple ways for Bob to earn the maximum total points, return any one of them.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/24/ex1.jpg" style="width: 600px; height: 120px;" />
 * Input: numArrows = 9, aliceArrows = [1,1,0,1,0,0,2,1,0,1,2,0]
 * Output: [0,0,0,0,1,1,0,0,1,2,3,1]
 * Explanation: The table above shows how the competition is scored.
 * Bob earns a total point of 4 + 5 + 8 + 9 + 10 + 11 = 47.
 * It can be shown that Bob cannot obtain a score higher than 47 points.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/02/24/ex2new.jpg" style="width: 600px; height: 117px;" />
 * Input: numArrows = 3, aliceArrows = [0,0,1,0,0,0,0,0,0,0,0,2]
 * Output: [0,0,0,0,0,0,0,0,1,1,1,0]
 * Explanation: The table above shows how the competition is scored.
 * Bob earns a total point of 8 + 9 + 10 = 27.
 * It can be shown that Bob cannot obtain a score higher than 27 points.
 *
 *  
 * Constraints:
 *
 * 	1 <= numArrows <= 10^5
 * 	aliceArrows.length == bobArrows.length == 12
 * 	0 <= aliceArrows[i], bobArrows[i] <= numArrows
 * 	sum(aliceArrows[i]) == numArrows
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-points-in-an-archery-competition/
// discuss: https://leetcode.com/problems/maximum-points-in-an-archery-competition/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2212_example_1() {
        let num_arrows = 9;
        let alice_arrows = vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0];

        let result = vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 2, 3, 1];

        assert_eq!(
            Solution::maximum_bob_points(num_arrows, alice_arrows),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2212_example_2() {
        let num_arrows = 3;
        let alice_arrows = vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2];

        let result = vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0];

        assert_eq!(
            Solution::maximum_bob_points(num_arrows, alice_arrows),
            result
        );
    }
}

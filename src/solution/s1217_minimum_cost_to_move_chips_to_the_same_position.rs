/**
 * [1217] Minimum Cost to Move Chips to The Same Position
 *
 * We have n chips, where the position of the i^th chip is position[i].
 * We need to move all the chips to the same position. In one step, we can change the position of the i^th chip from position[i] to:
 *
 * 	position[i] + 2 or position[i] - 2 with cost = 0.
 * 	position[i] + 1 or position[i] - 1 with cost = 1.
 *
 * Return the minimum cost needed to move all the chips to the same position.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/15/chips_e1.jpg" style="width: 750px; height: 217px;" />
 * Input: position = [1,2,3]
 * Output: 1
 * Explanation: First step: Move the chip at position 3 to position 1 with cost = 0.
 * Second step: Move the chip at position 2 to position 1 with cost = 1.
 * Total cost is 1.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/15/chip_e2.jpg" style="width: 750px; height: 306px;" />
 * Input: position = [2,2,2,3,3]
 * Output: 2
 * Explanation: We can move the two chips at position  3 to position 2. Each move has cost = 1. The total cost = 2.
 *
 * Example 3:
 *
 * Input: position = [1,1000000000]
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= position.length <= 100
 * 	1 <= position[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/
// discuss: https://leetcode.com/problems/minimum-cost-to-move-chips-to-the-same-position/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        *position
            .iter()
            .fold([0, 0], |mut acc, &pos| {
                acc[(pos % 2) as usize] += 1;
                acc
            })
            .iter()
            .min()
            .unwrap() as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1217_example_1() {
        let position = vec![1, 2, 3];
        let result = 1;

        assert_eq!(Solution::min_cost_to_move_chips(position), result);
    }

    #[test]
    fn test_1217_example_2() {
        let position = vec![2, 2, 2, 3, 3];
        let result = 2;

        assert_eq!(Solution::min_cost_to_move_chips(position), result);
    }

    #[test]
    fn test_1217_example_3() {
        let position = vec![1, 1000000000];
        let result = 1;

        assert_eq!(Solution::min_cost_to_move_chips(position), result);
    }
}

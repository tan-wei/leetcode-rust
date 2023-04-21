/**
 * [1041] Robot Bounded In Circle
 *
 * On an infinite plane, a robot initially stands at (0, 0) and faces north. Note that:
 *
 * 	The north direction is the positive direction of the y-axis.
 * 	The south direction is the negative direction of the y-axis.
 * 	The east direction is the positive direction of the x-axis.
 * 	The west direction is the negative direction of the x-axis.
 *
 * The robot can receive one of three instructions:
 *
 * 	"G": go straight 1 unit.
 * 	"L": turn 90 degrees to the left (i.e., anti-clockwise direction).
 * 	"R": turn 90 degrees to the right (i.e., clockwise direction).
 *
 * The robot performs the instructions given in order, and repeats them forever.
 * Return true if and only if there exists a circle in the plane such that the robot never leaves the circle.
 *  
 * Example 1:
 *
 * Input: instructions = "GGLLGG"
 * Output: true
 * Explanation: The robot is initially at (0, 0) facing the north direction.
 * "G": move one step. Position: (0, 1). Direction: North.
 * "G": move one step. Position: (0, 2). Direction: North.
 * "L": turn 90 degrees anti-clockwise. Position: (0, 2). Direction: West.
 * "L": turn 90 degrees anti-clockwise. Position: (0, 2). Direction: South.
 * "G": move one step. Position: (0, 1). Direction: South.
 * "G": move one step. Position: (0, 0). Direction: South.
 * Repeating the instructions, the robot goes into the cycle: (0, 0) --> (0, 1) --> (0, 2) --> (0, 1) --> (0, 0).
 * Based on that, we return true.
 *
 * Example 2:
 *
 * Input: instructions = "GG"
 * Output: false
 * Explanation: The robot is initially at (0, 0) facing the north direction.
 * "G": move one step. Position: (0, 1). Direction: North.
 * "G": move one step. Position: (0, 2). Direction: North.
 * Repeating the instructions, keeps advancing in the north direction and does not go into cycles.
 * Based on that, we return false.
 *
 * Example 3:
 *
 * Input: instructions = "GL"
 * Output: true
 * Explanation: The robot is initially at (0, 0) facing the north direction.
 * "G": move one step. Position: (0, 1). Direction: North.
 * "L": turn 90 degrees anti-clockwise. Position: (0, 1). Direction: West.
 * "G": move one step. Position: (-1, 1). Direction: West.
 * "L": turn 90 degrees anti-clockwise. Position: (-1, 1). Direction: South.
 * "G": move one step. Position: (-1, 0). Direction: South.
 * "L": turn 90 degrees anti-clockwise. Position: (-1, 0). Direction: East.
 * "G": move one step. Position: (0, 0). Direction: East.
 * "L": turn 90 degrees anti-clockwise. Position: (0, 0). Direction: North.
 * Repeating the instructions, the robot goes into the cycle: (0, 0) --> (0, 1) --> (-1, 1) --> (-1, 0) --> (0, 0).
 * Based on that, we return true.
 *
 *  
 * Constraints:
 *
 * 	1 <= instructions.length <= 100
 * 	instructions[i] is 'G', 'L' or, 'R'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/robot-bounded-in-circle/
// discuss: https://leetcode.com/problems/robot-bounded-in-circle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut direction = (0, 1);
        let mut position = (0, 0);
        for c in instructions.chars() {
            match c {
                'G' => position = (position.0 + direction.0, position.1 + direction.1),
                'L' => direction = (-direction.1, direction.0),
                'R' => direction = (direction.1, -direction.0),
                _ => {}
            }
        }
        position == (0, 0) || direction != (0, 1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1041_example_1() {
        let instructions = "GGLLGG".to_string();
        let result = true;

        assert_eq!(Solution::is_robot_bounded(instructions), result);
    }

    #[test]
    fn test_1041_example_2() {
        let instructions = "GG".to_string();
        let result = false;

        assert_eq!(Solution::is_robot_bounded(instructions), result);
    }

    #[test]
    fn test_1041_example_3() {
        let instructions = "GL".to_string();
        let result = true;

        assert_eq!(Solution::is_robot_bounded(instructions), result);
    }
}

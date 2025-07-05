/**
 * [2069] Walking Robot Simulation II
 *
 * A width x height grid is on an XY-plane with the bottom-left cell at (0, 0) and the top-right cell at (width - 1, height - 1). The grid is aligned with the four cardinal directions ("North", "East", "South", and "West"). A robot is initially at cell (0, 0) facing direction "East".
 * The robot can be instructed to move for a specific number of steps. For each step, it does the following.
 * <ol>
 * 	Attempts to move forward one cell in the direction it is facing.
 * 	If the cell the robot is moving to is out of bounds, the robot instead turns 90 degrees counterclockwise and retries the step.
 * </ol>
 * After the robot finishes moving the number of steps required, it stops and awaits the next instruction.
 * Implement the Robot class:
 *
 * 	Robot(int width, int height) Initializes the width x height grid with the robot at (0, 0) facing "East".
 * 	void step(int num) Instructs the robot to move forward num steps.
 * 	int[] getPos() Returns the current cell the robot is at, as an array of length 2, [x, y].
 * 	String getDir() Returns the current direction of the robot, "North", "East", "South", or "West".
 *
 *  
 * Example 1:
 * <img alt="example-1" src="https://assets.leetcode.com/uploads/2021/10/09/example-1.png" style="width: 498px; height: 268px;" />
 * Input
 * ["Robot", "step", "step", "getPos", "getDir", "step", "step", "step", "getPos", "getDir"]
 * [[6, 3], [2], [2], [], [], [2], [1], [4], [], []]
 * Output
 * [null, null, null, [4, 0], "East", null, null, null, [1, 2], "West"]
 * Explanation
 * Robot robot = new Robot(6, 3); // Initialize the grid and the robot at (0, 0) facing East.
 * robot.step(2);  // It moves two steps East to (2, 0), and faces East.
 * robot.step(2);  // It moves two steps East to (4, 0), and faces East.
 * robot.getPos(); // return [4, 0]
 * robot.getDir(); // return "East"
 * robot.step(2);  // It moves one step East to (5, 0), and faces East.
 *                 // Moving the next step East would be out of bounds, so it turns and faces North.
 *                 // Then, it moves one step North to (5, 1), and faces North.
 * robot.step(1);  // It moves one step North to (5, 2), and faces North (not West).
 * robot.step(4);  // Moving the next step North would be out of bounds, so it turns and faces West.
 *                 // Then, it moves four steps West to (1, 2), and faces West.
 * robot.getPos(); // return [1, 2]
 * robot.getDir(); // return "West"
 *
 *  
 * Constraints:
 *
 * 	2 <= width, height <= 100
 * 	1 <= num <= 10^5
 * 	At most 10^4 calls in total will be made to step, getPos, and getDir.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/walking-robot-simulation-ii/
// discuss: https://leetcode.com/problems/walking-robot-simulation-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

enum Direction {
    North,
    East,
    South,
    West,
}

struct Robot {
    max_x: i32,
    max_y: i32,
    current_route_num: i32,
    position_cache: (i32, i32, Direction),
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            max_x: width - 1,
            max_y: height - 1,
            current_route_num: -1,
            position_cache: (0, 0, Direction::East),
        }
    }

    fn step(&mut self, num: i32) {
        if self.current_route_num < 0 {
            self.current_route_num = 0;
        }

        let perim = (self.max_x + 1) * 2 + (self.max_y - 1) * 2;

        self.current_route_num += num;
        self.current_route_num %= perim;
        self.set_position_data();
    }

    fn get_pos(&self) -> Vec<i32> {
        vec![self.position_cache.0.clone(), self.position_cache.1.clone()]
    }

    fn get_dir(&self) -> String {
        match self.position_cache.2 {
            Direction::North => "North",
            Direction::East => "East",
            Direction::South => "South",
            Direction::West => "West",
        }
        .to_string()
    }

    fn set_position_data(&mut self) {
        self.position_cache = if self.current_route_num < 0 {
            (0, 0, Direction::East)
        } else if self.current_route_num == 0 {
            (self.current_route_num, 0, Direction::South)
        } else if self.current_route_num <= self.max_x {
            (self.current_route_num, 0, Direction::East)
        } else if self.current_route_num <= self.max_x + self.max_y {
            (
                self.max_x,
                self.current_route_num - self.max_x,
                Direction::North,
            )
        } else if self.current_route_num <= 2 * self.max_x + self.max_y {
            (
                self.max_x - (self.current_route_num - self.max_x - self.max_y),
                self.max_y,
                Direction::West,
            )
        } else {
            (
                0,
                self.max_y - (self.current_route_num - 2 * self.max_x - self.max_y),
                Direction::South,
            )
        }
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2069_example_1() {
        let mut robot = Robot::new(6, 3); // Initialize the grid and the robot at (0, 0) facing East.
        robot.step(2); // It moves two steps East to (2, 0), and faces East.
        robot.step(2); // It moves two steps East to (4, 0), and faces East.
        assert_eq!(robot.get_pos(), vec![4, 0]); // return [4, 0]
        assert_eq!(robot.get_dir(), "East".to_string()); // return "East"
        robot.step(2); // It moves one step East to (5, 0), and faces East.
                       // Moving the next step East would be out of bounds, so it turns and faces North.
                       // Then, it moves one step North to (5, 1), and faces North.
        robot.step(1); // It moves one step North to (5, 2), and faces North (not West).
        robot.step(4); // Moving the next step North would be out of bounds, so it turns and faces West.
                       // Then, it moves four steps West to (1, 2), and faces West.
        assert_eq!(robot.get_pos(), vec![1, 2]); // return [1, 2]
        assert_eq!(robot.get_dir(), "West".to_string()); // return "West"
    }
}

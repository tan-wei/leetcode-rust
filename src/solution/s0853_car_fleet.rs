/**
 * [0853] Car Fleet
 *
 * There are n cars going to the same destination along a one-lane road. The destination is target miles away.
 * You are given two integer array position and speed, both of length n, where position[i] is the position of the i^th car and speed[i] is the speed of the i^th car (in miles per hour).
 * A car can never pass another car ahead of it, but it can catch up to it and drive bumper to bumper at the same speed. The faster car will slow down to match the slower car's speed. The distance between these two cars is ignored (i.e., they are assumed to have the same position).
 * A car fleet is some non-empty set of cars driving at the same position and same speed. Note that a single car is also a car fleet.
 * If a car catches up to a car fleet right at the destination point, it will still be considered as one car fleet.
 * Return the number of car fleets that will arrive at the destination.
 *  
 * <strong class="example">Example 1:
 *
 * Input: target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
 * Output: 3
 * Explanation:
 * The cars starting at 10 (speed 2) and 8 (speed 4) become a fleet, meeting each other at 12.
 * The car starting at 0 does not catch up to any other car, so it is a fleet by itself.
 * The cars starting at 5 (speed 1) and 3 (speed 3) become a fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches target.
 * Note that no other cars meet these fleets before the destination, so the answer is 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: target = 10, position = [3], speed = [3]
 * Output: 1
 * Explanation: There is only one car, hence there is only one fleet.
 *
 * <strong class="example">Example 3:
 *
 * Input: target = 100, position = [0,2,4], speed = [4,2,1]
 * Output: 1
 * Explanation:
 * The cars starting at 0 (speed 4) and 2 (speed 2) become a fleet, meeting each other at 4. The fleet moves at speed 2.
 * Then, the fleet (speed 2) and the car starting at 4 (speed 1) become one fleet, meeting each other at 6. The fleet moves at speed 1 until it reaches target.
 *
 *  
 * Constraints:
 *
 * 	n == position.length == speed.length
 * 	1 <= n <= 10^5
 * 	0 < target <= 10^6
 * 	0 <= position[i] < target
 * 	All the values of position are unique.
 * 	0 < speed[i] <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/car-fleet/
// discuss: https://leetcode.com/problems/car-fleet/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut records = Vec::new();
        for i in 0..position.len() {
            records.push((position[i], (target - position[i]) as f32 / speed[i] as f32));
        }

        records.sort_by_key(|r| r.0);
        records.reverse();
        let mut longest_time = 0f32;
        let mut count = 0;
        for record in records {
            if record.1 > longest_time {
                count += 1;
                longest_time = record.1;
            }
        }
        count
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0853_example_1() {
        let target = 12;
        let position = vec![10, 8, 0, 5, 3];
        let speed = vec![2, 4, 1, 1, 3];
        let result = 3;

        assert_eq!(Solution::car_fleet(target, position, speed), result);
    }

    #[test]
    fn test_0853_example_2() {
        let target = 10;
        let position = vec![3];
        let speed = vec![3];
        let result = 1;

        assert_eq!(Solution::car_fleet(target, position, speed), result);
    }

    #[test]
    fn test_0853_example_3() {
        let target = 100;
        let position = vec![0, 2, 4];
        let speed = vec![4, 2, 1];
        let result = 1;

        assert_eq!(Solution::car_fleet(target, position, speed), result);
    }
}

/**
 * [0871] Minimum Number of Refueling Stops
 *
 * A car travels from a starting position to a destination which is target miles east of the starting position.
 * There are gas stations along the way. The gas stations are represented as an array stations where stations[i] = [positioni, fueli] indicates that the i^th gas station is positioni miles east of the starting position and has fueli liters of gas.
 * The car starts with an infinite tank of gas, which initially has startFuel liters of fuel in it. It uses one liter of gas per one mile that it drives. When the car reaches a gas station, it may stop and refuel, transferring all the gas from the station into the car.
 * Return the minimum number of refueling stops the car must make in order to reach its destination. If it cannot reach the destination, return -1.
 * Note that if the car reaches a gas station with 0 fuel left, the car can still refuel there. If the car reaches the destination with 0 fuel left, it is still considered to have arrived.
 *  
 * Example 1:
 *
 * Input: target = 1, startFuel = 1, stations = []
 * Output: 0
 * Explanation: We can reach the target without refueling.
 *
 * Example 2:
 *
 * Input: target = 100, startFuel = 1, stations = [[10,100]]
 * Output: -1
 * Explanation: We can not reach the target (or even the first gas station).
 *
 * Example 3:
 *
 * Input: target = 100, startFuel = 10, stations = [[10,60],[20,30],[30,30],[60,40]]
 * Output: 2
 * Explanation: We start with 10 liters of fuel.
 * We drive to position 10, expending 10 liters of fuel.  We refuel from 0 liters to 60 liters of gas.
 * Then, we drive from position 10 to position 60 (expending 50 liters of fuel),
 * and refuel from 10 liters to 50 liters of gas.  We then drive to and reach the target.
 * We made 2 refueling stops along the way, so we return 2.
 *
 *  
 * Constraints:
 *
 * 	1 <= target, startFuel <= 10^9
 * 	0 <= stations.length <= 500
 * 	1 <= positioni < positioni+1 < target
 * 	1 <= fueli < 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-refueling-stops/
// discuss: https://leetcode.com/problems/minimum-number-of-refueling-stops/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/minimum-number-of-refueling-stops/solutions/1267126/
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut pos = 0;
        let mut fuel = start_fuel;
        let mut result = 0;
        let mut available_fuels = std::collections::BinaryHeap::new();
        let mut station_idx = 0;
        while pos + fuel < target {
            pos += fuel;
            while station_idx < stations.len() && stations[station_idx][0] <= pos {
                available_fuels.push(stations[station_idx][1]);
                station_idx += 1;
            }
            fuel = match available_fuels.pop() {
                Some(a) => a,
                None => return -1,
            };
            result += 1;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0871_example_1() {
        let target = 1;
        let start_fuel = 1;
        let stations = vec![];
        let result = 0;

        assert_eq!(
            Solution::min_refuel_stops(target, start_fuel, stations),
            result
        );
    }

    #[test]
    fn test_0871_example_2() {
        let target = 100;
        let start_fuel = 1;
        let stations = vec![vec![10, 100]];
        let result = -1;

        assert_eq!(
            Solution::min_refuel_stops(target, start_fuel, stations),
            result
        );
    }

    #[test]
    fn test_0871_example_3() {
        let target = 100;
        let start_fuel = 10;
        let stations = vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]];
        let result = 2;

        assert_eq!(
            Solution::min_refuel_stops(target, start_fuel, stations),
            result
        );
    }
}

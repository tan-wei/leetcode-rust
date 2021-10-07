/**
 * [365] Water and Jug Problem
 *
 * You are given two jugs with capacities jug1Capacity and jug2Capacity liters. There is an infinite amount of water supply available. Determine whether it is possible to measure exactly targetCapacity liters using these two jugs.
 * If targetCapacity liters of water are measurable, you must have targetCapacity liters of water contained within one or both buckets by the end.
 * Operations allowed:
 *
 * 	Fill any of the jugs with water.
 * 	Empty any of the jugs.
 * 	Pour water from one jug into another till the other jug is completely full, or the first jug itself is empty.
 *
 *  
 * Example 1:
 *
 * Input: jug1Capacity = 3, jug2Capacity = 5, targetCapacity = 4
 * Output: true
 * Explanation: The famous <a href="https://www.youtube.com/watch?v=BVtQNK_ZUJg&amp;ab_channel=notnek01" target="_blank">Die Hard</a> example
 *
 * Example 2:
 *
 * Input: jug1Capacity = 2, jug2Capacity = 6, targetCapacity = 5
 * Output: false
 *
 * Example 3:
 *
 * Input: jug1Capacity = 1, jug2Capacity = 2, targetCapacity = 3
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= jug1Capacity, jug2Capacity, targetCapacity <= 10^6
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/water-and-jug-problem/
// discuss: https://leetcode.com/problems/water-and-jug-problem/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        fn gcd(x: i32, y: i32) -> i32 {
            if x == y {
                return x;
            } else if x == 0 {
                return y;
            } else if y == 0 {
                return x;
            } else {
                match (x & 1_i32, y & 1_i32) {
                    (0, 0) => return gcd(x >> 1, y >> 1) << 1,
                    (0, 1) => return gcd(x >> 1, y),
                    (1, 0) => return gcd(x, y >> 1),
                    (1, 1) => return gcd((x - y).abs() >> 1, x.min(y)),
                    _ => unreachable!(),
                }
            }
        }

        target_capacity == 0
            || (jug1_capacity + jug2_capacity >= target_capacity
                && (target_capacity % gcd(jug1_capacity, jug2_capacity)) == 0)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0365_example_1() {
        let jug1_capacity = 3;
        let jug2_capacity = 5;
        let target_capacity = 4;
        let result = true;

        assert_eq!(
            Solution::can_measure_water(jug1_capacity, jug2_capacity, target_capacity),
            result
        );
    }

    #[test]
    fn test_0365_example_2() {
        let jug1_capacity = 2;
        let jug2_capacity = 6;
        let target_capacity = 5;
        let result = false;

        assert_eq!(
            Solution::can_measure_water(jug1_capacity, jug2_capacity, target_capacity),
            result
        );
    }

    #[test]
    fn test_0365_example_3() {
        let jug1_capacity = 1;
        let jug2_capacity = 2;
        let target_capacity = 3;
        let result = true;

        assert_eq!(
            Solution::can_measure_water(jug1_capacity, jug2_capacity, target_capacity),
            result
        );
    }
}

/**
 * [0735] Asteroid Collision
 *
 * We are given an array asteroids of integers representing asteroids in a row.
 * For each asteroid, the absolute value represents its size, and the sign represents its direction (positive meaning right, negative meaning left). Each asteroid moves at the same speed.
 * Find out the state of the asteroids after all collisions. If two asteroids meet, the smaller one will explode. If both are the same size, both will explode. Two asteroids moving in the same direction will never meet.
 *  
 * Example 1:
 *
 * Input: asteroids = [5,10,-5]
 * Output: [5,10]
 * Explanation: The 10 and -5 collide resulting in 10. The 5 and 10 never collide.
 *
 * Example 2:
 *
 * Input: asteroids = [8,-8]
 * Output: []
 * Explanation: The 8 and -8 collide exploding each other.
 *
 * Example 3:
 *
 * Input: asteroids = [10,2,-5]
 * Output: [10]
 * Explanation: The 2 and -5 collide resulting in -5. The 10 and -5 collide resulting in 10.
 *
 *  
 * Constraints:
 *
 * 	2 <= asteroids.length <= 10^4
 * 	-1000 <= asteroids[i] <= 1000
 * 	asteroids[i] != 0
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/asteroid-collision/
// discuss: https://leetcode.com/problems/asteroid-collision/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for &asteroid in asteroids.iter() {
            result.push(asteroid);
            while result.len() > 1 && result[result.len() - 2] > 0 && result[result.len() - 1] < 0 {
                let l = result.pop().unwrap();
                let r = result.pop().unwrap();
                match r.cmp(&-l) {
                    std::cmp::Ordering::Less => result.push(l),
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => result.push(r),
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0735_example_1() {
        let asteroids = vec![5, 10, -5];
        let result = vec![5, 10];

        assert_eq!(Solution::asteroid_collision(asteroids), result);
    }

    #[test]
    fn test_0735_example_2() {
        let asteroids = vec![8, -8];
        let result: Vec<i32> = vec![];

        assert_eq!(Solution::asteroid_collision(asteroids), result);
    }

    #[test]
    fn test_0735_example_3() {
        let asteroids = vec![10, 2, -5];
        let result = vec![10];

        assert_eq!(Solution::asteroid_collision(asteroids), result);
    }
}

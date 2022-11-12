/**
 * [0881] Boats to Save People
 *
 * You are given an array people where people[i] is the weight of the i^th person, and an infinite number of boats where each boat can carry a maximum weight of limit. Each boat carries at most two people at the same time, provided the sum of the weight of those people is at most limit.
 * Return the minimum number of boats to carry every given person.
 *  
 * Example 1:
 *
 * Input: people = [1,2], limit = 3
 * Output: 1
 * Explanation: 1 boat (1, 2)
 *
 * Example 2:
 *
 * Input: people = [3,2,2,1], limit = 3
 * Output: 3
 * Explanation: 3 boats (1, 2), (2) and (3)
 *
 * Example 3:
 *
 * Input: people = [3,5,3,4], limit = 5
 * Output: 4
 * Explanation: 4 boats (3), (3), (4), (5)
 *
 *  
 * Constraints:
 *
 * 	1 <= people.length <= 5 * 10^4
 * 	1 <= people[i] <= limit <= 3 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/boats-to-save-people/
// discuss: https://leetcode.com/problems/boats-to-save-people/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let (mut lo, mut hi) = (0, people.len() - 1);
        let mut result = 0;
        while lo < hi {
            if people[lo] + people[hi] <= limit {
                lo += 1;
            }
            hi -= 1;
            result += 1;
        }
        result + if lo == hi { 1 } else { 0 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0881_example_1() {
        let people = vec![1, 2];
        let limit = 3;
        let result = 1;

        assert_eq!(Solution::num_rescue_boats(people, limit), result);
    }

    #[test]
    fn test_0881_example_2() {
        let people = vec![3, 2, 2, 1];
        let limit = 3;
        let result = 3;

        assert_eq!(Solution::num_rescue_boats(people, limit), result);
    }

    #[test]
    fn test_0881_example_3() {
        let people = vec![3, 2, 2, 1];
        let limit = 3;
        let result = 3;

        assert_eq!(Solution::num_rescue_boats(people, limit), result);
    }
}

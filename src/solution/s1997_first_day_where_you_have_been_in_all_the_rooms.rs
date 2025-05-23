/**
 * [1997] First Day Where You Have Been in All the Rooms
 *
 * There are n rooms you need to visit, labeled from 0 to n - 1. Each day is labeled, starting from 0. You will go in and visit one room a day.
 * Initially on day 0, you visit room 0. The order you visit the rooms for the coming days is determined by the following rules and a given 0-indexed array nextVisit of length n:
 *
 * 	Assuming that on a day, you visit room i,
 * 	if you have been in room i an odd number of times (including the current visit), on the next day you will visit a room with a lower or equal room number specified by nextVisit[i] where 0 <= nextVisit[i] <= i;
 * 	if you have been in room i an even number of times (including the current visit), on the next day you will visit room (i + 1) mod n.
 *
 * Return the label of the first day where you have been in all the rooms. It can be shown that such a day exists. Since the answer may be very large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: nextVisit = [0,0]
 * Output: 2
 * Explanation:
 * - On day 0, you visit room 0. The total times you have been in room 0 is 1, which is odd.
 *   On the next day you will visit room nextVisit[0] = 0
 * - On day 1, you visit room 0, The total times you have been in room 0 is 2, which is even.
 *   On the next day you will visit room (0 + 1) mod 2 = 1
 * - On day 2, you visit room 1. This is the first day where you have been in all the rooms.
 *
 * Example 2:
 *
 * Input: nextVisit = [0,0,2]
 * Output: 6
 * Explanation:
 * Your room visiting order for each day is: [0,0,1,0,0,1,2,...].
 * Day 6 is the first day where you have been in all the rooms.
 *
 * Example 3:
 *
 * Input: nextVisit = [0,1,2,0]
 * Output: 6
 * Explanation:
 * Your room visiting order for each day is: [0,0,1,1,2,2,3,...].
 * Day 6 is the first day where you have been in all the rooms.
 *
 *  
 * Constraints:
 *
 * 	n == nextVisit.length
 * 	2 <= n <= 10^5
 * 	0 <= nextVisit[i] <= i
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/first-day-where-you-have-been-in-all-the-rooms/
// discuss: https://leetcode.com/problems/first-day-where-you-have-been-in-all-the-rooms/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/first-day-where-you-have-been-in-all-the-rooms/solutions/3242780/just-a-runnable-solution/
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let mod_num = 1_000_000_007;
        let mut dp = vec![0_i64; next_visit.len() + 1];
        for i in 1..next_visit.len() {
            dp[i] = (2 + 2 * dp[i - 1] - dp[next_visit[i - 1] as usize] + mod_num) % mod_num;
        }
        dp[next_visit.len() - 1] as _
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1997_example_1() {
        let next_visit = vec![0, 0];

        let result = 2;

        assert_eq!(Solution::first_day_been_in_all_rooms(next_visit), result);
    }

    #[test]
    fn test_1997_example_2() {
        let next_visit = vec![0, 0, 2];

        let result = 6;

        assert_eq!(Solution::first_day_been_in_all_rooms(next_visit), result);
    }

    #[test]
    fn test_1997_example_3() {
        let next_visit = vec![0, 1, 2, 0];

        let result = 6;

        assert_eq!(Solution::first_day_been_in_all_rooms(next_visit), result);
    }
}

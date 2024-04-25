/**
 * [1575] Count All Possible Routes
 *
 * You are given an array of distinct positive integers locations where locations[i] represents the position of city i. You are also given integers start, finish and fuel representing the starting city, ending city, and the initial amount of fuel you have, respectively.
 * At each step, if you are at city i, you can pick any city j such that j != i and 0 <= j < locations.length and move to city j. Moving from city i to city j reduces the amount of fuel you have by |locations[i] - locations[j]|. Please notice that |x| denotes the absolute value of x.
 * Notice that fuel cannot become negative at any point in time, and that you are allowed to visit any city more than once (including start and finish).
 * Return the count of all possible routes from start to finish. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: locations = [2,3,6,8,4], start = 1, finish = 3, fuel = 5
 * Output: 4
 * Explanation: The following are all possible routes, each uses 5 units of fuel:
 * 1 -> 3
 * 1 -> 2 -> 3
 * 1 -> 4 -> 3
 * 1 -> 4 -> 2 -> 3
 *
 * Example 2:
 *
 * Input: locations = [4,3,1], start = 1, finish = 0, fuel = 6
 * Output: 5
 * Explanation: The following are all possible routes:
 * 1 -> 0, used fuel = 1
 * 1 -> 2 -> 0, used fuel = 5
 * 1 -> 2 -> 1 -> 0, used fuel = 5
 * 1 -> 0 -> 1 -> 0, used fuel = 3
 * 1 -> 0 -> 1 -> 0 -> 1 -> 0, used fuel = 5
 *
 * Example 3:
 *
 * Input: locations = [5,2,1], start = 0, finish = 2, fuel = 3
 * Output: 0
 * Explanation: It is impossible to get from 0 to 2 using only 3 units of fuel since the shortest route needs 4 units of fuel.
 *
 *  
 * Constraints:
 *
 * 	2 <= locations.length <= 100
 * 	1 <= locations[i] <= 10^9
 * 	All integers in locations are distinct.
 * 	0 <= start, finish < locations.length
 * 	1 <= fuel <= 200
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-all-possible-routes/
// discuss: https://leetcode.com/problems/count-all-possible-routes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/count-all-possible-routes/solutions/3678810/rust-dp/
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let mut dp = vec![vec![-1; fuel as usize + 1]; locations.len()];
        Self::dfs_helper(start as usize, finish as usize, fuel, &locations, &mut dp)
    }

    fn dfs_helper(
        cur: usize,
        finish: usize,
        fuel: i32,
        locations: &[i32],
        dp: &mut [Vec<i32>],
    ) -> i32 {
        if dp[cur][fuel as usize] == -1 {
            let mut t = if cur == finish { 1 } else { 0 };
            for i in 0..locations.len() {
                if i != cur && fuel >= (locations[i] - locations[cur]).abs() {
                    t = (t + Self::dfs_helper(
                        i,
                        finish,
                        fuel - (locations[i] - locations[cur]).abs(),
                        locations,
                        dp,
                    )) % 1_000_000_007;
                }
            }
            dp[cur][fuel as usize] = t;
        }
        dp[cur][fuel as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1575_example_1() {
        let locations = vec![2, 3, 6, 8, 4];
        let start = 1;
        let finish = 3;
        let fuel = 5;

        let result = 4;

        assert_eq!(
            Solution::count_routes(locations, start, finish, fuel),
            result
        );
    }

    #[test]
    fn test_1575_example_2() {
        let locations = vec![4, 3, 1];
        let start = 1;
        let finish = 0;
        let fuel = 6;

        let result = 5;

        assert_eq!(
            Solution::count_routes(locations, start, finish, fuel),
            result
        );
    }

    #[test]
    fn test_1575_example_3() {
        let locations = vec![5, 2, 1];
        let start = 0;
        let finish = 2;
        let fuel = 3;

        let result = 0;

        assert_eq!(
            Solution::count_routes(locations, start, finish, fuel),
            result
        );
    }
}

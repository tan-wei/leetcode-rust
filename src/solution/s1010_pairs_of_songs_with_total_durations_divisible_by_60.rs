/**
 * [1010] Pairs of Songs With Total Durations Divisible by 60
 *
 * You are given a list of songs where the i^th song has a duration of time[i] seconds.
 * Return the number of pairs of songs for which their total duration in seconds is divisible by 60. Formally, we want the number of indices i, j such that i < j with (time[i] + time[j]) % 60 == 0.
 *  
 * Example 1:
 *
 * Input: time = [30,20,150,100,40]
 * Output: 3
 * Explanation: Three pairs have a total duration divisible by 60:
 * (time[0] = 30, time[2] = 150): total duration 180
 * (time[1] = 20, time[3] = 100): total duration 120
 * (time[1] = 20, time[4] = 40): total duration 60
 *
 * Example 2:
 *
 * Input: time = [60,60,60]
 * Output: 3
 * Explanation: All three pairs have a total duration of 120, which is divisible by 60.
 *
 *  
 * Constraints:
 *
 * 	1 <= time.length <= 6 * 10^4
 * 	1 <= time[i] <= 500
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/
// discuss: https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut result: i64 = 0;
        let mut dist = std::collections::HashMap::new();
        for t in time {
            let remainer = t % 60;
            *dist.entry(remainer).or_insert(0) += 1;
        }
        for i in 1..=29 {
            if let Some(l_count) = dist.get(&i) {
                if let Some(r_count) = dist.get(&(60 - i)) {
                    result += l_count * r_count;
                }
            }
        }
        if let Some(c) = dist.get(&0) {
            let c = *c;
            if c > 1 {
                result += c * (c - 1) / 2;
            }
        }
        if let Some(c) = dist.get(&30) {
            let c = *c;
            if c > 1 {
                result += c * (c - 1) / 2;
            }
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1010_example_1() {
        let time = vec![30, 20, 150, 100, 40];
        let result = 3;

        assert_eq!(Solution::num_pairs_divisible_by60(time), result);
    }

    #[test]
    fn test_1010_example_2() {
        let time = vec![60, 60, 60];
        let result = 3;

        assert_eq!(Solution::num_pairs_divisible_by60(time), result);
    }

    #[test]
    fn test_1010_additional_1() {
        let time = vec![60; 60_000];
        let result = 1799970000;

        assert_eq!(Solution::num_pairs_divisible_by60(time), result);
    }
}

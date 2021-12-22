/**
 * [0458] Poor Pigs
 *
 * There are buckets buckets of liquid, where exactly one of the buckets is poisonous. To figure out which one is poisonous, you feed some number of (poor) pigs the liquid to see whether they will die or not. Unfortunately, you only have minutesToTest minutes to determine which bucket is poisonous.
 * You can feed the pigs according to these steps:
 * <ol>
 * 	Choose some live pigs to feed.
 * 	For each pig, choose which buckets to feed it. The pig will consume all the chosen buckets simultaneously and will take no time.
 * 	Wait for minutesToDie minutes. You may not feed any other pigs during this time.
 * 	After minutesToDie minutes have passed, any pigs that have been fed the poisonous bucket will die, and all others will survive.
 * 	Repeat this process until you run out of time.
 * </ol>
 * Given buckets, minutesToDie, and minutesToTest, return the minimum number of pigs needed to figure out which bucket is poisonous within the allotted time.
 *  
 * Example 1:
 * Input: buckets = 1000, minutesToDie = 15, minutesToTest = 60
 * Output: 5
 * Example 2:
 * Input: buckets = 4, minutesToDie = 15, minutesToTest = 15
 * Output: 2
 * Example 3:
 * Input: buckets = 4, minutesToDie = 15, minutesToTest = 30
 * Output: 2
 *  
 * Constraints:
 *
 * 	1 <= buckets <= 1000
 * 	1 <= minutesToDie <= minutesToTest <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/poor-pigs/
// discuss: https://leetcode.com/problems/poor-pigs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let times = minutes_to_test / minutes_to_die + 1;
        let mut result = 0;
        while times.pow(result) < buckets {
            result += 1;
        }
        result as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0458_example_1() {
        let buckets = 1000;
        let minutes_to_die = 15;
        let minutes_to_test = 60;

        let result = 5;

        assert_eq!(
            Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test),
            result
        );
    }

    #[test]
    fn test_0458_example_2() {
        let buckets = 4;
        let minutes_to_die = 15;
        let minutes_to_test = 15;

        let result = 2;

        assert_eq!(
            Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test),
            result
        );
    }

    #[test]
    fn test_0458_example_3() {
        let buckets = 4;
        let minutes_to_die = 15;
        let minutes_to_test = 30;

        let result = 2;

        assert_eq!(
            Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test),
            result
        );
    }
}

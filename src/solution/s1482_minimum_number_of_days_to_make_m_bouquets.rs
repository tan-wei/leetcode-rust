/**
 * [1482] Minimum Number of Days to Make m Bouquets
 *
 * You are given an integer array bloomDay, an integer m and an integer k.
 * You want to make m bouquets. To make a bouquet, you need to use k adjacent flowers from the garden.
 * The garden consists of n flowers, the i^th flower will bloom in the bloomDay[i] and then can be used in exactly one bouquet.
 * Return the minimum number of days you need to wait to be able to make m bouquets from the garden. If it is impossible to make m bouquets return -1.
 *  
 * Example 1:
 *
 * Input: bloomDay = [1,10,3,10,2], m = 3, k = 1
 * Output: 3
 * Explanation: Let us see what happened in the first three days. x means flower bloomed and _ means flower did not bloom in the garden.
 * We need 3 bouquets each should contain 1 flower.
 * After day 1: [x, _, _, _, _]   // we can only make one bouquet.
 * After day 2: [x, _, _, _, x]   // we can only make two bouquets.
 * After day 3: [x, _, x, _, x]   // we can make 3 bouquets. The answer is 3.
 *
 * Example 2:
 *
 * Input: bloomDay = [1,10,3,10,2], m = 3, k = 2
 * Output: -1
 * Explanation: We need 3 bouquets each has 2 flowers, that means we need 6 flowers. We only have 5 flowers so it is impossible to get the needed bouquets and we return -1.
 *
 * Example 3:
 *
 * Input: bloomDay = [7,7,7,7,12,7,7], m = 2, k = 3
 * Output: 12
 * Explanation: We need 2 bouquets each should have 3 flowers.
 * Here is the garden after the 7 and 12 days:
 * After day 7: [x, x, x, x, _, x, x]
 * We can make one bouquet of the first three flowers that bloomed. We cannot make another bouquet from the last three flowers that bloomed because they are not adjacent.
 * After day 12: [x, x, x, x, x, x, x]
 * It is obvious that we can make two bouquets in different ways.
 *
 *  
 * Constraints:
 *
 * 	bloomDay.length == n
 * 	1 <= n <= 10^5
 * 	1 <= bloomDay[i] <= 10^9
 * 	1 <= m <= 10^6
 * 	1 <= k <= n
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/
// discuss: https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let gg = *bloom_day.iter().max().unwrap();
        let (mut min, mut max) = (0, gg + 1);
        let mut mid;
        while min < max {
            mid = (min + max) / 2;

            if Self::can_bloom(mid, &bloom_day, m, k) {
                max = mid;
            } else {
                min = mid + 1;
            }
        }

        if min > gg { -1 } else { min }
    }
    fn can_bloom(day: i32, days: &Vec<i32>, m: i32, k: i32) -> bool {
        let k = k as usize;
        let (mut prev, mut ans) = (0, 0);

        for i in 0..days.len() {
            if days[i] <= day {
                if 1 + i - prev == k {
                    ans += 1;
                    prev = i + 1;
                }
            } else {
                prev = i + 1;
            }
            if ans == m {
                return true;
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1482_example_1() {
        let bloom_day = vec![1, 10, 3, 10, 2];
        let m = 3;
        let k = 1;

        let result = 3;

        assert_eq!(Solution::min_days(bloom_day, m, k), result);
    }

    #[test]
    fn test_1482_example_2() {
        let bloom_day = vec![1, 10, 3, 10, 2];
        let m = 3;
        let k = 2;

        let result = -1;

        assert_eq!(Solution::min_days(bloom_day, m, k), result);
    }

    #[test]
    fn test_1482_example_3() {
        let bloom_day = vec![7, 7, 7, 7, 12, 7, 7];
        let m = 2;
        let k = 3;

        let result = 12;

        assert_eq!(Solution::min_days(bloom_day, m, k), result);
    }
}

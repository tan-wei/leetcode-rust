/**
 * [1705] Maximum Number of Eaten Apples
 *
 * There is a special kind of apple tree that grows apples every day for n days. On the i^th day, the tree grows apples[i] apples that will rot after days[i] days, that is on day i + days[i] the apples will be rotten and cannot be eaten. On some days, the apple tree does not grow any apples, which are denoted by apples[i] == 0 and days[i] == 0.
 * You decided to eat at most one apple a day (to keep the doctors away). Note that you can keep eating after the first n days.
 * Given two integer arrays days and apples of length n, return the maximum number of apples you can eat.
 *  
 * Example 1:
 *
 * Input: apples = [1,2,3,5,2], days = [3,2,1,4,2]
 * Output: 7
 * Explanation: You can eat 7 apples:
 * - On the first day, you eat an apple that grew on the first day.
 * - On the second day, you eat an apple that grew on the second day.
 * - On the third day, you eat an apple that grew on the second day. After this day, the apples that grew on the third day rot.
 * - On the fourth to the seventh days, you eat apples that grew on the fourth day.
 *
 * Example 2:
 *
 * Input: apples = [3,0,0,0,0,2], days = [3,0,0,0,0,2]
 * Output: 5
 * Explanation: You can eat 5 apples:
 * - On the first to the third day you eat apples that grew on the first day.
 * - Do nothing on the fouth and fifth days.
 * - On the sixth and seventh days you eat apples that grew on the sixth day.
 *
 *  
 * Constraints:
 *
 * 	n == apples.length == days.length
 * 	1 <= n <= 2 * 10^4
 * 	0 <= apples[i], days[i] <= 2 * 10^4
 * 	days[i] = 0 if and only if apples[i] = 0.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-eaten-apples/
// discuss: https://leetcode.com/problems/maximum-number-of-eaten-apples/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-number-of-eaten-apples/solutions/2770248/rust-solution-using-binaryheap/
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        let n = apples.len();
        let mut result = 0;
        for i in 0..n {
            let num = apples[i] as usize;
            let day = days[i] as usize;

            if num != 0 {
                heap.push((std::cmp::Reverse(i + day - 1), num));
            }

            while let Some((std::cmp::Reverse(cd), num)) = heap.pop() {
                if cd >= i {
                    result += 1;
                    if num != 1 {
                        heap.push((std::cmp::Reverse(cd), num - 1));
                    }
                    break;
                }
            }
        }

        let mut i = n;

        while let Some((std::cmp::Reverse(cd), num)) = heap.pop() {
            if cd >= i {
                let diff = cd - i + 1;
                let v = std::cmp::min(diff, num);
                result += v;
                i += v;
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
    fn test_1705_example_1() {
        let apples = vec![1, 2, 3, 5, 2];
        let days = vec![3, 2, 1, 4, 2];

        let result = 7;

        assert_eq!(Solution::eaten_apples(apples, days), result);
    }

    #[test]
    fn test_1705_example_2() {
        let apples = vec![3, 0, 0, 0, 0, 2];
        let days = vec![3, 0, 0, 0, 0, 2];

        let result = 5;

        assert_eq!(Solution::eaten_apples(apples, days), result);
    }
}

/**
 * [1383] Maximum Performance of a Team
 *
 * You are given two integers n and k and two integer arrays speed and efficiency both of length n. There are n engineers numbered from 1 to n. speed[i] and efficiency[i] represent the speed and efficiency of the i^th engineer respectively.
 * Choose at most k different engineers out of the n engineers to form a team with the maximum performance.
 * The performance of a team is the sum of its engineers' speeds multiplied by the minimum efficiency among its engineers.
 * Return the maximum performance of this team. Since the answer can be a huge number, return it modulo 10^9 + 7.
 *  
 * Example 1:
 *
 * Input: n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 2
 * Output: 60
 * Explanation:
 * We have the maximum performance of the team by selecting engineer 2 (with speed=10 and efficiency=4) and engineer 5 (with speed=5 and efficiency=7). That is, performance = (10 + 5) * min(4, 7) = 60.
 *
 * Example 2:
 *
 * Input: n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 3
 * Output: 68
 * Explanation:
 * This is the same example as the first but k = 3. We can select engineer 1, engineer 2 and engineer 5 to get the maximum performance of the team. That is, performance = (2 + 10 + 5) * min(5, 4, 7) = 68.
 *
 * Example 3:
 *
 * Input: n = 6, speed = [2,10,3,1,5,8], efficiency = [5,4,3,9,7,2], k = 4
 * Output: 72
 *
 *  
 * Constraints:
 *
 * 	1 <= k <= n <= 10^5
 * 	speed.length == n
 * 	efficiency.length == n
 * 	1 <= speed[i] <= 10^5
 * 	1 <= efficiency[i] <= 10^8
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-performance-of-a-team/
// discuss: https://leetcode.com/problems/maximum-performance-of-a-team/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/maximum-performance-of-a-team/solutions/2560565/python-rust-priority-queue/
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut pairs = efficiency
            .iter()
            .map(|x| *x as u64)
            .zip(speed.iter().map(|x| *x as u64))
            .collect::<Vec<(u64, u64)>>();
        pairs.sort_unstable();

        let mut best: u64 = 0;
        let mut heap = std::collections::BinaryHeap::new();
        let mut cur: u64 = 0;
        let k = (k as usize);

        for (e, s) in pairs.iter().rev() {
            heap.push(std::cmp::Reverse(s));
            cur += s;
            if heap.len() > k {
                cur -= heap.pop().unwrap().0;
            }
            if e * cur > best {
                best = e * cur;
            }
        }

        (best % 1_000_000_007) as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1383_example_1() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 2;

        let result = 60;

        assert_eq!(Solution::max_performance(n, speed, efficiency, k), result);
    }

    #[test]
    fn test_1383_example_2() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 3;

        let result = 68;

        assert_eq!(Solution::max_performance(n, speed, efficiency, k), result);
    }

    #[test]
    fn test_1383_example_3() {
        let n = 6;
        let speed = vec![2, 10, 3, 1, 5, 8];
        let efficiency = vec![5, 4, 3, 9, 7, 2];
        let k = 4;

        let result = 72;

        assert_eq!(Solution::max_performance(n, speed, efficiency, k), result);
    }
}

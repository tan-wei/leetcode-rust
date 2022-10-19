/**
 * [0857] Minimum Cost to Hire K Workers
 *
 * There are n workers. You are given two integer arrays quality and wage where quality[i] is the quality of the i^th worker and wage[i] is the minimum wage expectation for the i^th worker.
 * We want to hire exactly k workers to form a paid group. To hire a group of k workers, we must pay them according to the following rules:
 * <ol>
 * 	Every worker in the paid group should be paid in the ratio of their quality compared to other workers in the paid group.
 * 	Every worker in the paid group must be paid at least their minimum wage expectation.
 * </ol>
 * Given the integer k, return the least amount of money needed to form a paid group satisfying the above conditions. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * <strong class="example">Example 1:
 *
 * Input: quality = [10,20,5], wage = [70,50,30], k = 2
 * Output: 105.00000
 * Explanation: We pay 70 to 0^th worker and 35 to 2^nd worker.
 *
 * <strong class="example">Example 2:
 *
 * Input: quality = [3,1,10,10,1], wage = [4,8,2,2,7], k = 3
 * Output: 30.66667
 * Explanation: We pay 4 to 0^th worker, 13.33333 to 2^nd and 3^rd workers separately.
 *
 *  
 * Constraints:
 *
 * 	n == quality.length == wage.length
 * 	1 <= k <= n <= 10^4
 * 	1 <= quality[i], wage[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-hire-k-workers/
// discuss: https://leetcode.com/problems/minimum-cost-to-hire-k-workers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/minimum-cost-to-hire-k-workers/solutions/837915/rust-translated-8ms-100/

const EPSILON: f64 = 1.0e-5;

#[derive(Debug, Default)]
struct Worker {
    ratio: f64,
    quality: i32,
    wage: i32,
}

impl Eq for Worker {}

impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        (self.ratio - other.ratio).abs() < EPSILON
    }
}

impl PartialOrd for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.ratio - other.ratio > EPSILON {
            std::cmp::Ordering::Greater
        } else if other.ratio - self.ratio > EPSILON {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let n = quality.len();
        let mut workers = vec![];
        for i in 0..n {
            let worker = Worker {
                ratio: wage[i] as f64 / quality[i] as f64,
                quality: quality[i],
                wage: wage[i],
            };
            workers.push(worker);
        }
        workers.sort();
        let mut result = 1.0e9;
        let mut sum = 0;
        let mut heap = std::collections::BinaryHeap::<i32>::new();
        for worker in &workers {
            heap.push(worker.quality);
            sum += worker.quality;
            if heap.len() as i32 > k {
                sum -= heap.pop().unwrap();
            }
            if heap.len() as i32 == k {
                let x = sum as f64 * worker.ratio;
                if x < result {
                    result = x;
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
    fn test_0857_example_1() {
        let quality = vec![10, 20, 5];
        let wage = vec![70, 50, 30];
        let k = 2;
        let result = 105.000000000000000;

        assert_f64_near!(Solution::mincost_to_hire_workers(quality, wage, k), result);
    }

    #[test]
    fn test_0857_example_2() {
        let quality = vec![3, 1, 10, 10, 1];
        let wage = vec![4, 8, 2, 2, 7];
        let k = 3;
        let result = 30.666666666666664;

        assert_f64_near!(Solution::mincost_to_hire_workers(quality, wage, k), result);
    }
}

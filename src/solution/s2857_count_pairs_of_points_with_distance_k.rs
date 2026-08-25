/**
 * [2857] Count Pairs of Points With Distance k
 *
 * You are given a 2D integer array coordinates and an integer k, where coordinates[i] = [xi, yi] are the coordinates of the i^th point in a 2D plane.
 * We define the distance between two points (x1, y1) and (x2, y2) as (x1 XOR x2) + (y1 XOR y2) where XOR is the bitwise XOR operation.
 * Return the number of pairs (i, j) such that i < j and the distance between points i and j is equal to k.
 *  
 * Example 1:
 *
 * Input: coordinates = [[1,2],[4,2],[1,3],[5,2]], k = 5
 * Output: 2
 * Explanation: We can choose the following pairs:
 * - (0,1): Because we have (1 XOR 4) + (2 XOR 2) = 5.
 * - (2,3): Because we have (1 XOR 5) + (3 XOR 2) = 5.
 *
 * Example 2:
 *
 * Input: coordinates = [[1,3],[1,3],[1,3],[1,3],[1,3]], k = 0
 * Output: 10
 * Explanation: Any two chosen pairs will have a distance of 0. There are 10 ways to choose two pairs.
 *
 *  
 * Constraints:
 *
 * 	2 <= coordinates.length <= 50000
 * 	0 <= xi, yi <= 10^6
 * 	0 <= k <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-pairs-of-points-with-distance-k/
// discuss: https://leetcode.com/problems/count-pairs-of-points-with-distance-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/count-pairs-of-points-with-distance-k/solutions/4184117/just-a-runnable-solution-by-ssrlive-728s/
    pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut hm = std::collections::HashMap::new();
        let mut result = 0;

        for c in &coordinates {
            for kx in 0..=k {
                result += *hm.get(&(c[0] ^ kx, c[1] ^ (k - kx))).unwrap_or(&0);
            }
            *hm.entry((c[0], c[1])).or_insert(0) += 1;
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2857_example_1() {
        let coordinates = vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]];
        let k = 5;

        let result = 2;

        assert_eq!(Solution::count_pairs(coordinates, k), result);
    }

    #[test]
    fn test_2857_example_2() {
        let coordinates = vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3]];
        let k = 0;

        let result = 10;

        assert_eq!(Solution::count_pairs(coordinates, k), result);
    }
}

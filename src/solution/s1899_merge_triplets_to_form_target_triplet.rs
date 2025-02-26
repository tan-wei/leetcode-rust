/**
 * [1899] Merge Triplets to Form Target Triplet
 *
 * A triplet is an array of three integers. You are given a 2D integer array triplets, where triplets[i] = [ai, bi, ci] describes the i^th triplet. You are also given an integer array target = [x, y, z] that describes the triplet you want to obtain.
 * To obtain target, you may apply the following operation on triplets any number of times (possibly zero):
 *
 * 	Choose two indices (0-indexed) i and j (i != j) and update triplets[j] to become [max(ai, aj), max(bi, bj), max(ci, cj)].
 *
 * 		For example, if triplets[i] = [2, 5, 3] and triplets[j] = [1, 7, 5], triplets[j] will be updated to [max(2, 1), max(5, 7), max(3, 5)] = [2, 7, 5].
 *
 *
 *
 * Return true if it is possible to obtain the target triplet [x, y, z] as an element of triplets, or false otherwise.
 *  
 * Example 1:
 *
 * Input: triplets = [[2,5,3],[1,8,4],[1,7,5]], target = [2,7,5]
 * Output: true
 * Explanation: Perform the following operations:
 * - Choose the first and last triplets [<u>[2,5,3]</u>,[1,8,4],<u>[1,7,5]</u>]. Update the last triplet to be [max(2,1), max(5,7), max(3,5)] = [2,7,5]. triplets = [[2,5,3],[1,8,4],<u>[2,7,5]</u>]
 * The target triplet [2,7,5] is now an element of triplets.
 *
 * Example 2:
 *
 * Input: triplets = [[3,4,5],[4,5,6]], target = [3,2,5]
 * Output: false
 * Explanation: It is impossible to have [3,2,5] as an element because there is no 2 in any of the triplets.
 *
 * Example 3:
 *
 * Input: triplets = [[2,5,3],[2,3,4],[1,2,5],[5,2,3]], target = [5,5,5]
 * Output: true
 * Explanation: Perform the following operations:
 * - Choose the first and third triplets [<u>[2,5,3]</u>,[2,3,4],<u>[1,2,5]</u>,[5,2,3]]. Update the third triplet to be [max(2,1), max(5,2), max(3,5)] = [2,5,5]. triplets = [[2,5,3],[2,3,4],<u>[2,5,5]</u>,[5,2,3]].
 * - Choose the third and fourth triplets [[2,5,3],[2,3,4],<u>[2,5,5]</u>,<u>[5,2,3]</u>]. Update the fourth triplet to be [max(2,5), max(5,2), max(5,3)] = [5,5,5]. triplets = [[2,5,3],[2,3,4],[2,5,5],<u>[5,5,5]</u>].
 * The target triplet [5,5,5] is now an element of triplets.
 *
 *  
 * Constraints:
 *
 * 	1 <= triplets.length <= 10^5
 * 	triplets[i].length == target.length == 3
 * 	1 <= ai, bi, ci, x, y, z <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-triplets-to-form-target-triplet/
// discuss: https://leetcode.com/problems/merge-triplets-to-form-target-triplet/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/merge-triplets-to-form-target-triplet/solutions/5418612/rust-one-liner-45ms-10-08mb/
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        target
            == triplets
                .into_iter()
                .filter(|triplet| triplet.iter().zip(target.iter()).all(|(a, b)| a <= b))
                .fold(vec![0; 3], |mut acc, x| {
                    acc.into_iter().zip(x).map(|(a, b)| a.max(b)).collect()
                })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1899_example_1() {
        let triplets = vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]];
        let target = vec![2, 7, 5];

        let result = true;

        assert_eq!(Solution::merge_triplets(triplets, target), result);
    }

    #[test]
    fn test_1899_example_2() {
        let triplets = vec![vec![3, 4, 5], vec![4, 5, 6]];
        let target = vec![3, 2, 5];

        let result = false;

        assert_eq!(Solution::merge_triplets(triplets, target), result);
    }

    #[test]
    fn test_1899_example_3() {
        let triplets = vec![vec![2, 5, 3], vec![2, 3, 4], vec![1, 2, 5], vec![5, 2, 3]];
        let target = vec![5, 5, 5];

        let result = true;

        assert_eq!(Solution::merge_triplets(triplets, target), result);
    }
}

/**
 * [1725] Number Of Rectangles That Can Form The Largest Square
 *
 * You are given an array rectangles where rectangles[i] = [li, wi] represents the i^th rectangle of length li and width wi.
 *
 * You can cut the i^th rectangle to form a square with a side length of k if both k <= li and k <= wi. For example, if you have a rectangle [4,6], you can cut it to get a square with a side length of at most 4.
 *
 * Let maxLen be the side length of the largest square you can obtain from any of the given rectangles.
 *
 * Return the number of rectangles that can make a square with a side length of maxLen.
 *
 *  
 * Example 1:
 *
 *
 * Input: rectangles = [[5,8],[3,9],[5,12],[16,5]]
 * Output: 3
 * Explanation: The largest squares you can get from each rectangle are of lengths [5,3,5,5].
 * The largest possible square is of length 5, and you can get it out of 3 rectangles.
 *
 *
 * Example 2:
 *
 *
 * Input: rectangles = [[2,3],[3,7],[4,3],[3,7]]
 * Output: 3
 *
 *
 *  
 * Constraints:
 *
 *
 * 	1 <= rectangles.length <= 1000
 * 	rectangles[i].length == 2
 * 	1 <= li, wi <= 10^9
 * 	li != wi
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/
// discuss: https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        rectangles
            .into_iter()
            .fold((0, 0), |(m, count), r| {
                let w = r[0].min(r[1]);
                match w.cmp(&m) {
                    std::cmp::Ordering::Equal => (m, count + 1),
                    std::cmp::Ordering::Greater => (w, 1),
                    std::cmp::Ordering::Less => (m, count),
                }
            })
            .1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1725_example_1() {
        let rectangles = vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]];

        let result = 3;

        assert_eq!(Solution::count_good_rectangles(rectangles), result);
    }

    #[test]
    fn test_1725_example_2() {
        let rectangles = vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]];

        let result = 3;

        assert_eq!(Solution::count_good_rectangles(rectangles), result);
    }
}

/**
 * [1691] Maximum Height by Stacking Cuboids
 *
 * Given n cuboids where the dimensions of the i^th cuboid is cuboids[i] = [widthi, lengthi, heighti] (0-indexed). Choose a subset of cuboids and place them on each other.
 * You can place cuboid i on cuboid j if widthi <= widthj and lengthi <= lengthj and heighti <= heightj. You can rearrange any cuboid's dimensions by rotating it to put it on another cuboid.
 * Return the maximum height of the stacked cuboids.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/10/21/image.jpg" style="width: 420px; height: 299px;" />
 *
 * Input: cuboids = [[50,45,20],[95,37,53],[45,23,12]]
 * Output: 190
 * Explanation:
 * Cuboid 1 is placed on the bottom with the 53x37 side facing down with height 95.
 * Cuboid 0 is placed next with the 45x20 side facing down with height 50.
 * Cuboid 2 is placed next with the 23x12 side facing down with height 45.
 * The total height is 95 + 50 + 45 = 190.
 *
 * Example 2:
 *
 * Input: cuboids = [[38,25,45],[76,35,3]]
 * Output: 76
 * Explanation:
 * You can't place any of the cuboids on the other.
 * We choose cuboid 1 and rotate it so that the 35x3 side is facing down and its height is 76.
 *
 * Example 3:
 *
 * Input: cuboids = [[7,11,17],[7,17,11],[11,7,17],[11,17,7],[17,7,11],[17,11,7]]
 * Output: 102
 * Explanation:
 * After rearranging the cuboids, you can see that all cuboids have the same dimension.
 * You can place the 11x7 side down on all cuboids so their heights are 17.
 * The maximum height of stacked cuboids is 6 * 17 = 102.
 *
 *  
 * Constraints:
 *
 * 	n == cuboids.length
 * 	1 <= n <= 100
 * 	1 <= widthi, lengthi, heighti <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-height-by-stacking-cuboids/
// discuss: https://leetcode.com/problems/maximum-height-by-stacking-cuboids/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        cuboids
            .into_iter()
            .map(|mut cuboid| {
                cuboid.sort_unstable();
                (cuboid[2], cuboid[1], cuboid[0])
            })
            .collect::<std::collections::BinaryHeap<(i32, i32, i32)>>()
            .into_sorted_vec()
            .into_iter()
            .rev()
            .fold(Vec::new(), |mut dp, (x, y, z)| {
                dp.push((
                    x + dp
                        .iter()
                        .map(|&(px, py, pz)| px * (py >= y && pz >= z) as i32)
                        .max()
                        .unwrap_or(0),
                    y,
                    z,
                ));
                dp
            })
            .into_iter()
            .max()
            .unwrap()
            .0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1691_example_1() {
        let cuboids = vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]];

        let result = 190;

        assert_eq!(Solution::max_height(cuboids), result);
    }

    #[test]
    fn test_1691_example_2() {
        let cuboids = vec![vec![38, 25, 45], vec![76, 35, 3]];

        let result = 76;

        assert_eq!(Solution::max_height(cuboids), result);
    }

    #[test]
    fn test_1691_example_3() {
        let cuboids = vec![
            vec![7, 11, 17],
            vec![7, 17, 11],
            vec![11, 7, 17],
            vec![11, 17, 7],
            vec![17, 7, 11],
            vec![17, 11, 7],
        ];

        let result = 102;

        assert_eq!(Solution::max_height(cuboids), result);
    }
}

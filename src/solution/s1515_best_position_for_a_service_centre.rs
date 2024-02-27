/**
 * [1515] Best Position for a Service Centre
 *
 * A delivery company wants to build a new service center in a new city. The company knows the positions of all the customers in this city on a 2D-Map and wants to build the new center in a position such that the sum of the euclidean distances to all customers is minimum.
 * Given an array positions where positions[i] = [xi, yi] is the position of the ith customer on the map, return the minimum sum of the euclidean distances to all customers.
 * In other words, you need to choose the position of the service center [xcentre, ycentre] such that the following formula is minimized:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/25/q4_edited.jpg" />
 * Answers within 10^-5 of the actual value will be accepted.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/25/q4_e1.jpg" style="width: 377px; height: 362px;" />
 * Input: positions = [[0,1],[1,0],[1,2],[2,1]]
 * Output: 4.00000
 * Explanation: As shown, you can see that choosing [xcentre, ycentre] = [1, 1] will make the distance to each customer = 1, the sum of all distances is 4 which is the minimum possible we can achieve.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/25/q4_e3.jpg" style="width: 419px; height: 419px;" />
 * Input: positions = [[1,1],[3,3]]
 * Output: 2.82843
 * Explanation: The minimum possible sum of distances = sqrt(2) + sqrt(2) = 2.82843
 *
 *  
 * Constraints:
 *
 * 	1 <= positions.length <= 50
 * 	positions[i].length == 2
 * 	0 <= xi, yi <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-position-for-a-service-centre/
// discuss: https://leetcode.com/problems/best-position-for-a-service-centre/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/best-position-for-a-service-centre/solutions/3162833/just-a-runnable-solution/
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let mut x = 0.0;
        let mut y = 0.0;
        for p in &positions {
            x += p[0] as f64;
            y += p[1] as f64;
        }
        x /= positions.len() as f64;
        y /= positions.len() as f64;
        let mut step = 100.0;
        let mut min_dist = std::f64::MAX;
        while step > 1e-6 {
            let mut found = false;
            for dx in &[-1.0, 0.0, 1.0] {
                for dy in &[-1.0, 0.0, 1.0] {
                    let nx = x + dx * step;
                    let ny = y + dy * step;
                    let dist = Self::get_dist_sum(&positions, nx, ny);
                    if dist < min_dist {
                        min_dist = dist;
                        x = nx;
                        y = ny;
                        found = true;
                    }
                }
            }
            if !found {
                step /= 2.0;
            }
        }
        min_dist
    }

    fn get_dist_sum(positions: &Vec<Vec<i32>>, x: f64, y: f64) -> f64 {
        let mut sum = 0.0;
        for p in positions {
            sum += ((p[0] as f64 - x).powi(2) + (p[1] as f64 - y).powi(2)).sqrt();
        }
        sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1515_example_1() {
        let positions = vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]];

        let result = 4.00000;

        assert_f64_near!(Solution::get_min_dist_sum(positions), result);
    }

    #[test]
    fn test_1515_example_2() {
        let positions = vec![vec![1, 1], vec![3, 3]];

        let result = 2.82842712474619;

        assert_f64_near!(Solution::get_min_dist_sum(positions), result);
    }
}

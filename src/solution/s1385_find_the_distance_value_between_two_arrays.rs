/**
 * [1385] Find the Distance Value Between Two Arrays
 *
 * Given two integer arrays arr1 and arr2, and the integer d, return the distance value between the two arrays.
 * The distance value is defined as the number of elements arr1[i] such that there is not any element arr2[j] where |arr1[i]-arr2[j]| <= d.
 *  
 * Example 1:
 *
 * Input: arr1 = [4,5,8], arr2 = [10,9,1,8], d = 2
 * Output: 2
 * Explanation:
 * For arr1[0]=4 we have:
 * |4-10|=6 > d=2
 * |4-9|=5 > d=2
 * |4-1|=3 > d=2
 * |4-8|=4 > d=2
 * For arr1[1]=5 we have:
 * |5-10|=5 > d=2
 * |5-9|=4 > d=2
 * |5-1|=4 > d=2
 * |5-8|=3 > d=2
 * For arr1[2]=8 we have:
 * |8-10|=2 <= d=2
 * |8-9|=1 <= d=2
 * |8-1|=7 > d=2
 * |8-8|=0 <= d=2
 *
 * Example 2:
 *
 * Input: arr1 = [1,4,2,3], arr2 = [-4,-3,6,10,20,30], d = 3
 * Output: 2
 *
 * Example 3:
 *
 * Input: arr1 = [2,1,100,3], arr2 = [-5,-2,10,-3,7], d = 6
 * Output: 1
 *
 *  
 * Constraints:
 *
 * 	1 <= arr1.length, arr2.length <= 500
 * 	-1000 <= arr1[i], arr2[j] <= 1000
 * 	0 <= d <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-distance-value-between-two-arrays/
// discuss: https://leetcode.com/problems/find-the-distance-value-between-two-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        arr1.iter()
            .filter(|&x| arr2.iter().all(|&y| (x - y).abs() > d))
            .count() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1385_example_1() {
        let arr1 = vec![4, 5, 8];
        let arr2 = vec![10, 9, 1, 8];
        let d = 2;

        let result = 2;

        assert_eq!(Solution::find_the_distance_value(arr1, arr2, d), result)
    }

    #[test]
    fn test_1385_example_2() {
        let arr1 = vec![1, 4, 2, 3];
        let arr2 = vec![-4, -3, 6, 10, 20, 30];
        let d = 3;

        let result = 2;

        assert_eq!(Solution::find_the_distance_value(arr1, arr2, d), result)
    }

    #[test]
    fn test_1385_example_3() {
        let arr1 = vec![2, 1, 100, 3];
        let arr2 = vec![-5, -2, 10, -3, 7];
        let d = 6;

        let result = 1;

        assert_eq!(Solution::find_the_distance_value(arr1, arr2, d), result)
    }
}

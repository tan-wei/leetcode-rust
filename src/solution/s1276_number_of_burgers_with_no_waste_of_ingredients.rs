/**
 * [1276] Number of Burgers with No Waste of Ingredients
 *
 * Given two integers tomatoSlices and cheeseSlices. The ingredients of different burgers are as follows:
 *
 * 	Jumbo Burger: 4 tomato slices and 1 cheese slice.
 * 	Small Burger: 2 Tomato slices and 1 cheese slice.
 *
 * Return [total_jumbo, total_small] so that the number of remaining tomatoSlices equal to 0 and the number of remaining cheeseSlices equal to 0. If it is not possible to make the remaining tomatoSlices and cheeseSlices equal to 0 return [].
 *
 * Example 1:
 *
 * Input: tomatoSlices = 16, cheeseSlices = 7
 * Output: [1,6]
 * Explantion: To make one jumbo burger and 6 small burgers we need 4*1 + 2*6 = 16 tomato and 1 + 6 = 7 cheese.
 * There will be no remaining ingredients.
 *
 * Example 2:
 *
 * Input: tomatoSlices = 17, cheeseSlices = 4
 * Output: []
 * Explantion: There will be no way to use all ingredients to make small and jumbo burgers.
 *
 * Example 3:
 *
 * Input: tomatoSlices = 4, cheeseSlices = 17
 * Output: []
 * Explantion: Making 1 jumbo burger there will be 16 cheese remaining and making 2 small burgers there will be 15 cheese remaining.
 *
 *
 * Constraints:
 *
 * 	0 <= tomatoSlices, cheeseSlices <= 10^7
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-burgers-with-no-waste-of-ingredients/
// discuss: https://leetcode.com/problems/number-of-burgers-with-no-waste-of-ingredients/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 2 != 0 {
            return Vec::<i32>::new();
        }
        let x = tomato_slices / 2 - cheese_slices;
        let y = 2 * cheese_slices - tomato_slices / 2;
        if x < 0 || y < 0 {
            return Vec::<i32>::new();
        };
        vec![x, y]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1276_example_1() {
        let tomato_slices = 16;
        let cheese_slices = 7;
        let result = vec![1, 6];

        assert_eq!(
            Solution::num_of_burgers(tomato_slices, cheese_slices),
            result
        );
    }

    #[test]
    fn test_1276_example_2() {
        let tomato_slices = 17;
        let cheese_slices = 4;
        let result = vec![];

        assert_eq!(
            Solution::num_of_burgers(tomato_slices, cheese_slices),
            result
        );
    }

    #[test]
    fn test_1276_example_3() {
        let tomato_slices = 4;
        let cheese_slices = 17;
        let result = vec![];

        assert_eq!(
            Solution::num_of_burgers(tomato_slices, cheese_slices),
            result
        );
    }
}

/**
 * [2525] Categorize Box According to Criteria
 *
 * Given four integers length, width, height, and mass, representing the dimensions and mass of a box, respectively, return a string representing the category of the box.
 *
 * 	The box is "Bulky" if:
 *
 * 		Any of the dimensions of the box is greater or equal to 10^4.
 * 		Or, the volume of the box is greater or equal to 10^9.
 *
 *
 * 	If the mass of the box is greater or equal to 100, it is "Heavy".
 * 	If the box is both "Bulky" and "Heavy", then its category is "Both".
 * 	If the box is neither "Bulky" nor "Heavy", then its category is "Neither".
 * 	If the box is "Bulky" but not "Heavy", then its category is "Bulky".
 * 	If the box is "Heavy" but not "Bulky", then its category is "Heavy".
 *
 * Note that the volume of the box is the product of its length, width and height.
 *  
 * Example 1:
 *
 * Input: length = 1000, width = 35, height = 700, mass = 300
 * Output: "Heavy"
 * Explanation:
 * None of the dimensions of the box is greater or equal to 10^4.
 * Its volume = 24500000 <= 10^9. So it cannot be categorized as "Bulky".
 * However mass >= 100, so the box is "Heavy".
 * Since the box is not "Bulky" but "Heavy", we return "Heavy".
 * Example 2:
 *
 * Input: length = 200, width = 50, height = 800, mass = 50
 * Output: "Neither"
 * Explanation:
 * None of the dimensions of the box is greater or equal to 10^4.
 * Its volume = 8 * 10^6 <= 10^9. So it cannot be categorized as "Bulky".
 * Its mass is also less than 100, so it cannot be categorized as "Heavy" either.
 * Since its neither of the two above categories, we return "Neither".
 *  
 * Constraints:
 *
 * 	1 <= length, width, height <= 10^5
 * 	1 <= mass <= 10^3
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/categorize-box-according-to-criteria/
// discuss: https://leetcode.com/problems/categorize-box-according-to-criteria/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        match (
            [length, width, height].into_iter().any(|d| d >= 10_000)
                || [length, width, height]
                    .into_iter()
                    .reduce(i32::saturating_mul)
                    .is_some_and(|p| p >= 1_000_000_000),
            mass >= 100,
        ) {
            (true, true) => "Both",
            (true, false) => "Bulky",
            (false, true) => "Heavy",
            (false, false) => "Neither",
        }
        .into()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2525_example_1() {
        let length = 1000;
        let width = 35;
        let height = 700;
        let mass = 300;

        let result = "Heavy".to_string();

        assert_eq!(
            Solution::categorize_box(length, width, height, mass),
            result
        );
    }

    #[test]
    fn test_2525_example_2() {
        let length = 100;
        let width = 50;
        let height = 800;
        let mass = 50;

        let result = "Neither".to_string();

        assert_eq!(
            Solution::categorize_box(length, width, height, mass),
            result
        );
    }
}

/**
 * [2303] Calculate Amount Paid in Taxes
 *
 * You are given a 0-indexed 2D integer array brackets where brackets[i] = [upperi, percenti] means that the i^th tax bracket has an upper bound of upperi and is taxed at a rate of percenti. The brackets are sorted by upper bound (i.e. upperi-1 < upperi for 0 < i < brackets.length).
 * Tax is calculated as follows:
 *
 * 	The first upper0 dollars earned are taxed at a rate of percent0.
 * 	The next upper1 - upper0 dollars earned are taxed at a rate of percent1.
 * 	The next upper2 - upper1 dollars earned are taxed at a rate of percent2.
 * 	And so on.
 *
 * You are given an integer income representing the amount of money you earned. Return the amount of money that you have to pay in taxes. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * Example 1:
 *
 * Input: brackets = [[3,50],[7,10],[12,25]], income = 10
 * Output: 2.65000
 * Explanation:
 * Based on your income, you have 3 dollars in the 1^st tax bracket, 4 dollars in the 2^nd tax bracket, and 3 dollars in the 3^rd tax bracket.
 * The tax rate for the three tax brackets is 50%, 10%, and 25%, respectively.
 * In total, you pay $3 * 50% + $4 * 10% + $3 * 25% = $2.65 in taxes.
 *
 * Example 2:
 *
 * Input: brackets = [[1,0],[4,25],[5,50]], income = 2
 * Output: 0.25000
 * Explanation:
 * Based on your income, you have 1 dollar in the 1^st tax bracket and 1 dollar in the 2^nd tax bracket.
 * The tax rate for the two tax brackets is 0% and 25%, respectively.
 * In total, you pay $1 * 0% + $1 * 25% = $0.25 in taxes.
 *
 * Example 3:
 *
 * Input: brackets = [[2,50]], income = 0
 * Output: 0.00000
 * Explanation:
 * You have no income to tax, so you have to pay a total of $0 in taxes.
 *
 *  
 * Constraints:
 *
 * 	1 <= brackets.length <= 100
 * 	1 <= upperi <= 1000
 * 	0 <= percenti <= 100
 * 	0 <= income <= 1000
 * 	upperi is sorted in ascending order.
 * 	All the values of upperi are unique.
 * 	The upper bound of the last tax bracket is greater than or equal to income.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/calculate-amount-paid-in-taxes/
// discuss: https://leetcode.com/problems/calculate-amount-paid-in-taxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut income = income;
        [vec![vec![0, 0]], brackets]
            .concat()
            .windows(2)
            .map(|win| {
                let dif = win[1][0] - win[0][0];
                let tax = (dif.min(income) as f64) * win[1][1] as f64 / 100.0;
                income = (income - dif).max(0);
                tax
            })
            .sum()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2303_example_1() {
        let brackets = vec![vec![3, 50], vec![7, 10], vec![12, 25]];
        let income = 10;

        let result = 2.65000;

        assert_f64_near!(Solution::calculate_tax(brackets, income), result);
    }

    #[test]
    fn test_2303_example_2() {
        let brackets = vec![vec![1, 0], vec![4, 25], vec![5, 50]];
        let income = 2;

        let result = 0.25000;

        assert_f64_near!(Solution::calculate_tax(brackets, income), result);
    }

    #[test]
    fn test_2303_example_3() {
        let brackets = vec![vec![2, 50]];
        let income = 0;

        let result = 0.00000;

        assert_f64_near!(Solution::calculate_tax(brackets, income), result);
    }
}

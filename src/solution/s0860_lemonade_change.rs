/**
 * [0860] Lemonade Change
 *
 * At a lemonade stand, each lemonade costs $5. Customers are standing in a queue to buy from you and order one at a time (in the order specified by bills). Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill. You must provide the correct change to each customer so that the net transaction is that the customer pays $5.
 * Note that you do not have any change in hand at first.
 * Given an integer array bills where bills[i] is the bill the i^th customer pays, return true if you can provide every customer with the correct change, or false otherwise.
 *  
 * <strong class="example">Example 1:
 *
 * Input: bills = [5,5,5,10,20]
 * Output: true
 * Explanation:
 * From the first 3 customers, we collect three $5 bills in order.
 * From the fourth customer, we collect a $10 bill and give back a $5.
 * From the fifth customer, we give a $10 bill and a $5 bill.
 * Since all customers got correct change, we output true.
 *
 * <strong class="example">Example 2:
 *
 * Input: bills = [5,5,10,10,20]
 * Output: false
 * Explanation:
 * From the first two customers in order, we collect two $5 bills.
 * For the next two customers in order, we collect a $10 bill and give back a $5 bill.
 * For the last customer, we can not give the change of $15 back because we only have two $10 bills.
 * Since not every customer received the correct change, the answer is false.
 *
 *  
 * Constraints:
 *
 * 	1 <= bills.length <= 10^5
 * 	bills[i] is either 5, 10, or 20.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lemonade-change/
// discuss: https://leetcode.com/problems/lemonade-change/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = (0, 0, 0);
        for bill in bills {
            if bill == 5 {
                change.0 += 1;
            } else if bill == 10 {
                if change.0 < 1 {
                    return false;
                }
                change.0 -= 1;
                change.1 += 1;
            } else if bill == 20 {
                if change.0 > 0 && change.1 > 0 {
                    change.0 -= 1;
                    change.1 -= 1;
                } else if change.0 > 2 {
                    change.0 -= 3;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0860_example_1() {
        let bills = vec![5, 5, 5, 10, 20];
        let result = true;

        assert_eq!(Solution::lemonade_change(bills), result);
    }

    #[test]
    fn test_0860_example_2() {
        let bills = vec![5, 5, 10, 10, 20];
        let result = false;

        assert_eq!(Solution::lemonade_change(bills), result);
    }
}

/**
 * [1105] Filling Bookcase Shelves
 *
 * You are given an array books where books[i] = [thicknessi, heighti] indicates the thickness and height of the i^th book. You are also given an integer shelfWidth.
 * We want to place these books in order onto bookcase shelves that have a total width shelfWidth.
 * We choose some of the books to place on this shelf such that the sum of their thickness is less than or equal to shelfWidth, then build another level of the shelf of the bookcase so that the total height of the bookcase has increased by the maximum height of the books we just put down. We repeat this process until there are no more books to place.
 * Note that at each step of the above process, the order of the books we place is the same order as the given sequence of books.
 *
 * 	For example, if we have an ordered list of 5 books, we might place the first and second book onto the first shelf, the third book on the second shelf, and the fourth and fifth book on the last shelf.
 *
 * Return the minimum possible height that the total bookshelf can be after placing shelves in this manner.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/06/24/shelves.png" style="height: 500px; width: 337px;" />
 * Input: books = [[1,1],[2,3],[2,3],[1,1],[1,1],[1,1],[1,2]], shelfWidth = 4
 * Output: 6
 * Explanation:
 * The sum of the heights of the 3 shelves is 1 + 3 + 2 = 6.
 * Notice that book number 2 does not have to be on the first shelf.
 *
 * Example 2:
 *
 * Input: books = [[1,3],[2,4],[3,2]], shelfWidth = 6
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= books.length <= 1000
 * 	1 <= thicknessi <= shelfWidth <= 1000
 * 	1 <= heighti <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/filling-bookcase-shelves/
// discuss: https://leetcode.com/problems/filling-bookcase-shelves/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[n] = 0;

        for i in (0..n).rev() {
            let mut sum = 0;
            let mut max_height = 0;

            for j in (i..n) {
                sum += books[j][0];
                max_height = max_height.max(books[j][1]);

                if sum > shelf_width {
                    break;
                }

                dp[i] = dp[i].min(dp[j + 1] + max_height);
            }
        }

        dp[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1105_example_1() {
        let books = vec![
            vec![1, 1],
            vec![2, 3],
            vec![2, 3],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![1, 2],
        ];
        let shelf_width = 4;
        let result = 6;

        assert_eq!(Solution::min_height_shelves(books, shelf_width), result);
    }

    #[test]
    fn test_1105_example_2() {
        let books = vec![vec![1, 3], vec![2, 4], vec![3, 2]];
        let shelf_width = 6;
        let result = 4;

        assert_eq!(Solution::min_height_shelves(books, shelf_width), result);
    }
}

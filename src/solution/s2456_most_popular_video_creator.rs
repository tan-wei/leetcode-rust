/**
 * [2456] Most Popular Video Creator
 *
 * You are given two string arrays creators and ids, and an integer array views, all of length n. The i^th video on a platform was created by creators[i], has an id of ids[i], and has views[i] views.
 * The popularity of a creator is the sum of the number of views on all of the creator's videos. Find the creator with the highest popularity and the id of their most viewed video.
 *
 * 	If multiple creators have the highest popularity, find all of them.
 * 	If multiple videos have the highest view count for a creator, find the lexicographically smallest id.
 *
 * Note: It is possible for different videos to have the same id, meaning that ids do not uniquely identify a video. For example, two videos with the same ID are considered as distinct videos with their own viewcount.
 * Return a 2D array of strings answer where answer[i] = [creatorsi, idi] means that creatorsi has the highest popularity and idi is the id of their most popular video. The answer can be returned in any order.
 *  
 * Example 1:
 * <div class="example-block">
 * Input: <span class="example-io">creators = ["alice","bob","alice","chris"], ids = ["one","two","three","four"], views = [5,10,5,4]</span>
 * Output: <span class="example-io">[["alice","one"],["bob","two"]]</span>
 * Explanation:
 * The popularity of alice is 5 + 5 = 10.<br />
 * The popularity of bob is 10.<br />
 * The popularity of chris is 4.<br />
 * alice and bob are the most popular creators.<br />
 * For bob, the video with the highest view count is "two".<br />
 * For alice, the videos with the highest view count are "one" and "three". Since "one" is lexicographically smaller than "three", it is included in the answer.
 * </div>
 * Example 2:
 * <div class="example-block">
 * Input: <span class="example-io">creators = ["alice","alice","alice"], ids = ["a","b","c"], views = [1,2,2]</span>
 * Output: <span class="example-io">[["alice","b"]]</span>
 * Explanation:
 * The videos with id "b" and "c" have the highest view count.<br />
 * Since "b" is lexicographically smaller than "c", it is included in the answer.
 * </div>
 *  
 * Constraints:
 *
 * 	n == creators.length == ids.length == views.length
 * 	1 <= n <= 10^5
 * 	1 <= creators[i].length, ids[i].length <= 5
 * 	creators[i] and ids[i] consist only of lowercase English letters.
 * 	0 <= views[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/most-popular-video-creator/
// discuss: https://leetcode.com/problems/most-popular-video-creator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn most_popular_creator(
        creators: Vec<String>,
        ids: Vec<String>,
        views: Vec<i32>,
    ) -> Vec<Vec<String>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2456_example_1() {
        let creators = vec_string!["alice", "bob", "alice", "chris"];
        let ids = vec_string!["one", "two", "three", "four"];
        let views = vec![5, 10, 5, 4];

        let result = vec![vec_string!["alice", "one"], vec_string!["bob", "two"]];

        assert_eq!(Solution::most_popular_creator(creators, ids, views), result);
    }

    #[test]
    #[ignore]
    fn test_2456_example_2() {
        let creators = vec_string!["alice", "alice", "alice"];
        let ids = vec_string!["a", "b", "c"];
        let views = vec![1, 2, 3];

        let result = vec![vec_string!["alice", "b"]];

        assert_eq!(Solution::most_popular_creator(creators, ids, views), result);
    }
}

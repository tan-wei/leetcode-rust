/**
 * [2424] Longest Uploaded Prefix
 *
 * You are given a stream of n videos, each represented by a distinct number from 1 to n that you need to "upload" to a server. You need to implement a data structure that calculates the length of the longest uploaded prefix at various points in the upload process.
 * We consider i to be an uploaded prefix if all videos in the range 1 to i (inclusive) have been uploaded to the server. The longest uploaded prefix is the maximum value of i that satisfies this definition.<br />
 * <br />
 * Implement the LUPrefix class:
 *
 * 	LUPrefix(int n) Initializes the object for a stream of n videos.
 * 	void upload(int video) Uploads video to the server.
 * 	int longest() Returns the length of the longest uploaded prefix defined above.
 *
 *  
 * Example 1:
 *
 * Input
 * ["LUPrefix", "upload", "longest", "upload", "longest", "upload", "longest"]
 * [[4], [3], [], [1], [], [2], []]
 * Output
 * [null, null, 0, null, 1, null, 3]
 * Explanation
 * LUPrefix server = new LUPrefix(4);   // Initialize a stream of 4 videos.
 * server.upload(3);                    // Upload video 3.
 * server.longest();                    // Since video 1 has not been uploaded yet, there is no prefix.
 *                                      // So, we return 0.
 * server.upload(1);                    // Upload video 1.
 * server.longest();                    // The prefix [1] is the longest uploaded prefix, so we return 1.
 * server.upload(2);                    // Upload video 2.
 * server.longest();                    // The prefix [1,2,3] is the longest uploaded prefix, so we return 3.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	1 <= video <= n
 * 	All values of video are distinct.
 * 	At most 2 * 10^5 calls in total will be made to upload and longest.
 * 	At least one call will be made to longest.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-uploaded-prefix/
// discuss: https://leetcode.com/problems/longest-uploaded-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/longest-uploaded-prefix/solutions/2655341/rust-fastest-100-solution-using-boolean-is2la/

struct LUPrefix {
    prefixes: Vec<bool>,
    longest_prefix: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LUPrefix {
    fn new(n: i32) -> Self {
        Self {
            prefixes: vec![false; n as usize + 2],
            longest_prefix: 0,
        }
    }

    fn upload(&mut self, video: i32) {
        self.prefixes[video as usize] = true;
    }

    fn longest(&mut self) -> i32 {
        while self.prefixes[self.longest_prefix as usize + 1] {
            self.longest_prefix += 1;
        }

        self.longest_prefix
    }
}

/**
 * Your LUPrefix object will be instantiated and called as such:
 * let obj = LUPrefix::new(n);
 * obj.upload(video);
 * let ret_2: i32 = obj.longest();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2424_example_1() {
        let mut server = LUPrefix::new(4); // Initialize a stream of 4 videos.

        server.upload(3); // Upload video 3.

        assert_eq!(server.longest(), 0); // Since video 1 has not been uploaded yet, there is no prefix.
        // So, we return 0.

        server.upload(1); // Upload video 1.

        assert_eq!(server.longest(), 1); // The prefix [1] is the longest uploaded prefix, so we return 1.

        server.upload(2); // Upload video 2.

        assert_eq!(server.longest(), 3); // The prefix [1,2,3] is the longest uploaded prefix, so we return 3.
    }
}

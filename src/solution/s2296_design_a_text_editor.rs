/**
 * [2296] Design a Text Editor
 *
 * Design a text editor with a cursor that can do the following:
 *
 * 	Add text to where the cursor is.
 * 	Delete text from where the cursor is (simulating the backspace key).
 * 	Move the cursor either left or right.
 *
 * When deleting text, only characters to the left of the cursor will be deleted. The cursor will also remain within the actual text and cannot be moved beyond it. More formally, we have that 0 <= cursor.position <= currentText.length always holds.
 * Implement the TextEditor class:
 *
 * 	TextEditor() Initializes the object with empty text.
 * 	void addText(string text) Appends text to where the cursor is. The cursor ends to the right of text.
 * 	int deleteText(int k) Deletes k characters to the left of the cursor. Returns the number of characters actually deleted.
 * 	string cursorLeft(int k) Moves the cursor to the left k times. Returns the last min(10, len) characters to the left of the cursor, where len is the number of characters to the left of the cursor.
 * 	string cursorRight(int k) Moves the cursor to the right k times. Returns the last min(10, len) characters to the left of the cursor, where len is the number of characters to the left of the cursor.
 *
 *  
 * Example 1:
 *
 * Input
 * ["TextEditor", "addText", "deleteText", "addText", "cursorRight", "cursorLeft", "deleteText", "cursorLeft", "cursorRight"]
 * [[], ["leetcode"], [4], ["practice"], [3], [8], [10], [2], [6]]
 * Output
 * [null, null, 4, null, "etpractice", "leet", 4, "", "practi"]
 * Explanation
 * TextEditor textEditor = new TextEditor(); // The current text is "|". (The '|' character represents the cursor)
 * textEditor.addText("leetcode"); // The current text is "leetcode|".
 * textEditor.deleteText(4); // return 4
 *                           // The current text is "leet|".
 *                           // 4 characters were deleted.
 * textEditor.addText("practice"); // The current text is "leetpractice|".
 * textEditor.cursorRight(3); // return "etpractice"
 *                            // The current text is "leetpractice|".
 *                            // The cursor cannot be moved beyond the actual text and thus did not move.
 *                            // "etpractice" is the last 10 characters to the left of the cursor.
 * textEditor.cursorLeft(8); // return "leet"
 *                           // The current text is "leet|practice".
 *                           // "leet" is the last min(10, 4) = 4 characters to the left of the cursor.
 * textEditor.deleteText(10); // return 4
 *                            // The current text is "|practice".
 *                            // Only 4 characters were deleted.
 * textEditor.cursorLeft(2); // return ""
 *                           // The current text is "|practice".
 *                           // The cursor cannot be moved beyond the actual text and thus did not move.
 *                           // "" is the last min(10, 0) = 0 characters to the left of the cursor.
 * textEditor.cursorRight(6); // return "practi"
 *                            // The current text is "practi|ce".
 *                            // "practi" is the last min(10, 6) = 6 characters to the left of the cursor.
 *
 *  
 * Constraints:
 *
 * 	1 <= text.length, k <= 40
 * 	text consists of lowercase English letters.
 * 	At most 2 * 10^4 calls in total will be made to addText, deleteText, cursorLeft and cursorRight.
 *
 *  
 * Follow-up: Could you find a solution with time complexity of O(k) per call?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/design-a-text-editor/
// discuss: https://leetcode.com/problems/design-a-text-editor/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Credit: https://leetcode.com/problems/design-a-text-editor/solutions/2112768/rust-two-stacks-by-tomijaga-twtt/

struct TextEditor {
    left: Vec<char>,
    right: Vec<char>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TextEditor {
    fn new() -> Self {
        Self {
            left: vec![],
            right: vec![],
        }
    }

    fn add_text(&mut self, text: String) {
        for char in text.chars() {
            self.left.push(char);
        }
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let len = std::cmp::min(self.left.len() as i32, k);
        for i in 0..len {
            self.left.pop();
        }

        len
    }

    fn cursor_left(&mut self, k: i32) -> String {
        let len = std::cmp::min(self.left.len() as i32, k);

        for i in 0..len {
            let char = self.left.pop().unwrap();
            self.right.push(char);
        }

        let mut slice = &self.left[std::cmp::max(self.left.len() as i32 - 10, 0) as usize..];
        slice.iter().collect::<String>()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        let len = std::cmp::min(self.right.len() as i32, k);

        for i in 0..len {
            let char = self.right.pop().unwrap();
            self.left.push(char);
        }

        self.cursor_left(0)
    }
}

/**
 * Your TextEditor object will be instantiated and called as such:
 * let obj = TextEditor::new();
 * obj.add_text(text);
 * let ret_2: i32 = obj.delete_text(k);
 * let ret_3: String = obj.cursor_left(k);
 * let ret_4: String = obj.cursor_right(k);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2296_example_1() {
        let mut text_editor = TextEditor::new(); // The current text is "|". (The '|' character represents the cursor)
        text_editor.add_text("leetcode".to_string()); // The current text is "leetcode|".
        assert_eq!(text_editor.delete_text(4), 4); // return 4
                                                   // The current text is "leet|".
                                                   // 4 characters were deleted.
        text_editor.add_text("practice".to_string()); // The current text is "leetpractice|".
        assert_eq!(text_editor.cursor_right(3), "etpractice".to_string()); // return "etpractice"
                                                                           // The current text is "leetpractice|".
                                                                           // The cursor cannot be moved beyond the actual text and thus did not move.
                                                                           // "etpractice" is the last 10 characters to the left of the cursor.
        assert_eq!(text_editor.cursor_left(8), "leet".to_string()); // return "leet"
                                                                    // The current text is "leet|practice".
                                                                    // "leet" is the last min(10, 4) = 4 characters to the left of the cursor.
        assert_eq!(text_editor.delete_text(10), 4); // return 4
                                                    // The current text is "|practice".
                                                    // Only 4 characters were deleted.
        assert_eq!(text_editor.cursor_left(2), "".to_string()); // return ""
                                                                // The current text is "|practice".
                                                                // The cursor cannot be moved beyond the actual text and thus did not move.
                                                                // "" is the last min(10, 0) = 0 characters to the left of the cursor.
        assert_eq!(text_editor.cursor_right(6), "practi".to_string()); // return "practi"
                                                                       // The current text is "practi|ce".
                                                                       // "practi" is the last min(10, 6) = 6 characters to the left of the cursor.}
    }
}

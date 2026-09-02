#!/usr/bin/env python3
"""
Clean HTML tags and entities from doc-comment blocks in LeetCode solution files.

Only processes the /** ... */ doc comment at the top of each file.
Leaves code sections untouched.
"""

import re
import os
import glob

SOLUTION_DIR = os.path.join(os.path.dirname(os.path.dirname(__file__)), "src", "solution")


def clean_html_in_doc_comment(content: str) -> str:
    """
    Find the first /** ... */ doc comment block and clean HTML from it.
    """
    # Match the opening /* or /** through closing */ at the start of the file
    match = re.match(r'^(\s*/\*[*]?.*?\*/)', content, re.DOTALL)
    if not match:
        return content

    doc_block = match.group(1)
    cleaned_block = clean_html(doc_block)
    return content.replace(doc_block, cleaned_block, 1)


def clean_html(text: str) -> str:
    """Remove HTML tags and replace HTML entities in text."""
    # Replace <sup> with ^ before stripping other tags
    text = text.replace("<sup>", "^").replace("</sup>", "")

    # Remove all HTML tags (with or without attributes)
    text = re.sub(r'</?[a-zA-Z][^>]*>', '', text)

    # HTML entities — ordered so longer/more specific ones are replaced first
    text = text.replace("&amp;", "&")
    text = text.replace("&nbsp;", " ")
    text = text.replace("&gt;", ">")
    text = text.replace("&lt;", "<")
    text = text.replace("&quot;", "\"")
    text = text.replace("&minus;", "-")
    text = text.replace("&#39;", "'")
    text = text.replace("&apos;", "'")
    text = text.replace("&frasl;", "/")
    text = text.replace("&le;", "\u2264")      # ≤
    text = text.replace("&ge;", "\u2265")      # ≥
    text = text.replace("&ldquo;", "\u201c")   # "
    text = text.replace("&rdquo;", "\u201d")   # "
    text = text.replace("&thinsp;", " ")
    text = text.replace("&rarr;", "\u2192")    # →
    text = text.replace("&larr;", "\u2190")    # ←
    text = text.replace("&uarr;", "\u2191")    # ↑
    text = text.replace("&darr;", "\u2193")    # ↓
    text = text.replace("&harr;", "\u2194")    # ↔
    text = text.replace("&rArr;", "\u21d2")    # ⇒
    text = text.replace("&lArr;", "\u21d0")    # ⇐
    text = text.replace("&hArr;", "\u21d4")    # ⇔
    text = text.replace("&times;", "\u00d7")   # ×
    text = text.replace("&plusmn;", "\u00b1")  # ±
    text = text.replace("&ne;", "\u2260")      # ≠
    text = text.replace("&infin;", "\u221e")   # ∞
    text = text.replace("&radic;", "\u221a")   # √
    text = text.replace("&cup;", "\u222a")     # ∪
    text = text.replace("&cap;", "\u2229")     # ∩
    text = text.replace("&empty;", "\u2205")   # ∅
    text = text.replace("&hellip;", "\u2026")  # …
    text = text.replace("&rsquo;", "\u2019")   # '
    text = text.replace("&lfloor;", "\u230a")  # ⌊
    text = text.replace("&rfloor;", "\u230b")  # ⌋
    text = text.replace("&emsp;", "\u2003")    # em space

    return text


def main():
    solution_files = sorted(glob.glob(os.path.join(SOLUTION_DIR, "s*.rs")))
    modified_count = 0
    total_changes = 0

    for filepath in solution_files:
        with open(filepath, 'r', encoding='utf-8') as f:
            original = f.read()

        cleaned = clean_html_in_doc_comment(original)

        if cleaned != original:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(cleaned)
            modified_count += 1

            # Count changes
            orig_doc = re.match(r'^(\s*/\*\*.*?\*/)', original, re.DOTALL)
            clean_doc = re.match(r'^(\s*/\*\*.*?\*/)', cleaned, re.DOTALL)
            if orig_doc and clean_doc:
                changes = sum(1 for a, b in zip(orig_doc.group(1), clean_doc.group(1)) if a != b)
                total_changes += changes

            filename = os.path.basename(filepath)
            print(f"  ✓ {filename}")

    print(f"\nDone! Modified {modified_count} files.")


if __name__ == "__main__":
    main()
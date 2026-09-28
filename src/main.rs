use std::io::{self, Read};

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("failed to read input");

    let text = text.trim_end_matches(|character| character == '\n' || character == '\r');
    println!("{}", longest_palindromic_substring(text));
}

fn is_palendrome(word: &str) -> bool {
    for ((i, first_c), (j, second_c)) in word.char_indices().zip(word.char_indices().rev()) {
        if i == j || j < i {
            break;
        }
        if first_c != second_c {
            return false;
        }
    }
    true
}

fn longest_palindromic_substring(text: &str) -> &str {
    let byte_len = text.len();

    let mut longest_seq = 0;
    let mut longest_slice = &text[0..0];
    for start in 0..byte_len {
        if !text.is_char_boundary(start) {
            continue;
        }

        for end in (start..byte_len).rev() {
            if !text.is_char_boundary(end + 1) {
                continue;
            }

            let slice = &text[start..end + 1];
            let slice_len = slice.chars().count();
            if is_palendrome(slice) && slice_len > longest_seq {
                longest_slice = slice;
                longest_seq = slice_len;
            }
        }
    }
    longest_slice
}

#[cfg(test)]
#[path = "../tests/unit/palindrome.rs"]
mod palindrome_tests;

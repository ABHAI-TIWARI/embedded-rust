#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharTopic {
    pub title: &'static str,
    pub rust_example: &'static str,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharDemoResults {
    pub ascii_char: char,
    pub unicode_char: char,
    pub unicode_len_utf8: usize,
    pub upper_char: char,
    pub digit_to_number: Option<u32>,
}

const TOPICS: [CharTopic; 5] = [
    CharTopic {
        title: "Character literal",
        rust_example: "let c: char = 'A';",
        explanation: "Rust `char` is a Unicode scalar value, not a single byte like C `char`.",
    },
    CharTopic {
        title: "Unicode char",
        rust_example: "let heart: char = '❤';",
        explanation: "Rust supports Unicode characters directly in `char` literals.",
    },
    CharTopic {
        title: "UTF-8 encoded length",
        rust_example: "let n = '❤'.len_utf8();",
        explanation: "A `char` can use 1-4 bytes in UTF-8; `len_utf8` returns encoded byte size.",
    },
    CharTopic {
        title: "Character transformations",
        rust_example: "let up = 'a'.to_ascii_uppercase();",
        explanation: "ASCII transformation helpers are available for common protocol/text operations.",
    },
    CharTopic {
        title: "Digit parsing",
        rust_example: "let d = '7'.to_digit(10);",
        explanation: "`to_digit(radix)` converts numeric chars into numbers safely with `Option`.",
    },
];

pub fn char_topics() -> &'static [CharTopic] {
    &TOPICS
}

pub fn demo_ascii_char() -> char {
    'A'
}

pub fn demo_unicode_char() -> char {
    '❤'
}

pub fn demo_unicode_utf8_len() -> usize {
    demo_unicode_char().len_utf8()
}

pub fn demo_ascii_uppercase() -> char {
    'a'.to_ascii_uppercase()
}

pub fn demo_digit_to_number() -> Option<u32> {
    '7'.to_digit(10)
}

pub fn run_char_demo() -> CharDemoResults {
    CharDemoResults {
        ascii_char: demo_ascii_char(),
        unicode_char: demo_unicode_char(),
        unicode_len_utf8: demo_unicode_utf8_len(),
        upper_char: demo_ascii_uppercase(),
        digit_to_number: demo_digit_to_number(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        char_topics, demo_ascii_char, demo_ascii_uppercase, demo_digit_to_number, demo_unicode_char,
        demo_unicode_utf8_len, run_char_demo,
    };

    #[test]
    fn has_expected_topics() {
        assert_eq!(char_topics().len(), 5);
    }

    #[test]
    fn ascii_char_demo_is_correct() {
        assert_eq!(demo_ascii_char(), 'A');
    }

    #[test]
    fn unicode_char_demo_is_correct() {
        assert_eq!(demo_unicode_char(), '❤');
    }

    #[test]
    fn unicode_utf8_length_is_correct() {
        assert_eq!(demo_unicode_utf8_len(), 3);
    }

    #[test]
    fn uppercase_demo_is_correct() {
        assert_eq!(demo_ascii_uppercase(), 'A');
    }

    #[test]
    fn digit_conversion_demo_is_correct() {
        assert_eq!(demo_digit_to_number(), Some(7));
    }

    #[test]
    fn aggregate_demo_is_correct() {
        let summary = run_char_demo();
        assert_eq!(summary.ascii_char, 'A');
        assert_eq!(summary.unicode_char, '❤');
        assert_eq!(summary.unicode_len_utf8, 3);
        assert_eq!(summary.upper_char, 'A');
        assert_eq!(summary.digit_to_number, Some(7));
    }
}

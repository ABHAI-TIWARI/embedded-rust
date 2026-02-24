// `derive(...)` provides debug formatting, copy/clone convenience,
// and equality traits useful in tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharTopic {
    // Topic title shown in logs.
    pub title: &'static str,
    // Example snippet for this char concept.
    pub rust_example: &'static str,
    // Short explanatory text.
    pub explanation: &'static str,
}

// Same derived traits for result struct to keep testing and logging simple.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharDemoResults {
    // ASCII char demo result.
    pub ascii_char: char,
    // Unicode char demo result.
    pub unicode_char: char,
    // UTF-8 length of unicode char.
    pub unicode_len_utf8: usize,
    // ASCII uppercase transformation result.
    pub upper_char: char,
    // Numeric conversion result from char.
    pub digit_to_number: Option<u32>,
}

// Tutorial topic list for char basics.
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

// Returns all char tutorial topics.
pub fn char_topics() -> &'static [CharTopic] {
    &TOPICS
}

// Demonstrates ASCII char literal.
pub fn demo_ascii_char() -> char {
    'A'
}

// Demonstrates Unicode char literal.
pub fn demo_unicode_char() -> char {
    '❤'
}

// Demonstrates UTF-8 encoded byte length of a char.
pub fn demo_unicode_utf8_len() -> usize {
    demo_unicode_char().len_utf8()
}

// Demonstrates ASCII uppercase conversion.
pub fn demo_ascii_uppercase() -> char {
    'a'.to_ascii_uppercase()
}

// Demonstrates digit-to-number conversion.
pub fn demo_digit_to_number() -> Option<u32> {
    '7'.to_digit(10)
}

// Runs all char demos and aggregates outputs.
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
    // Import module APIs for testing.
    use super::{
        char_topics, demo_ascii_char, demo_ascii_uppercase, demo_digit_to_number, demo_unicode_char,
        demo_unicode_utf8_len, run_char_demo,
    };

    // Verifies topic count remains expected.
    #[test]
    fn has_expected_topics() {
        assert_eq!(char_topics().len(), 5);
    }

    // Verifies ASCII char demo.
    #[test]
    fn ascii_char_demo_is_correct() {
        assert_eq!(demo_ascii_char(), 'A');
    }

    // Verifies Unicode char demo.
    #[test]
    fn unicode_char_demo_is_correct() {
        assert_eq!(demo_unicode_char(), '❤');
    }

    // Verifies UTF-8 byte-length calculation.
    #[test]
    fn unicode_utf8_length_is_correct() {
        assert_eq!(demo_unicode_utf8_len(), 3);
    }

    // Verifies ASCII uppercase transformation.
    #[test]
    fn uppercase_demo_is_correct() {
        assert_eq!(demo_ascii_uppercase(), 'A');
    }

    // Verifies digit conversion from char.
    #[test]
    fn digit_conversion_demo_is_correct() {
        assert_eq!(demo_digit_to_number(), Some(7));
    }

    // Verifies aggregate summary output.
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

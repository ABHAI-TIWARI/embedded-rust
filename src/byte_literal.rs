#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteLiteralTopic {
    // Topic title for logging.
    pub title: &'static str,
    // Small Rust code example for this topic.
    pub rust_example: &'static str,
    // Short explanation shown in logs/docs.
    pub explanation: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteLiteralDemoResults {
    // ASCII value of single-byte literal demo.
    pub ascii_a: u8,
    // Escaped newline byte value.
    pub newline: u8,
    // Byte string length.
    pub bytes_len: usize,
    // First byte in byte string.
    pub bytes_first: u8,
    // Sum of bytes in HELLO message.
    pub hello_sum: u16,
}

// Tutorial topics for byte literals.
const TOPICS: [ByteLiteralTopic; 4] = [
    ByteLiteralTopic {
        title: "Single-byte literal",
        rust_example: "let b = b'A';",
        explanation: "`b'A'` produces one byte (`u8`) using ASCII value of 'A' (65).",
    },
    ByteLiteralTopic {
        title: "Escaped byte literal",
        rust_example: "let nl = b'\\n';",
        explanation: "Escapes are supported in byte literals, e.g. newline is 10.",
    },
    ByteLiteralTopic {
        title: "Byte string literal",
        rust_example: "let msg = b\"HELLO\";",
        explanation: "`b\"...\"` creates a byte slice (`&[u8; N]`) useful for UART/protocol buffers.",
    },
    ByteLiteralTopic {
        title: "Hex byte literal",
        rust_example: "let marker = b'\\x7F';",
        explanation: "Hex escape gives exact byte value, common in embedded protocols.",
    },
];

// Returns all byte literal topics.
pub fn byte_literal_topics() -> &'static [ByteLiteralTopic] {
    &TOPICS
}

// Demonstrates `b'A'` single-byte literal.
pub fn demo_single_byte_literal() -> u8 {
    b'A'
}

// Demonstrates escaped newline byte literal.
pub fn demo_escaped_newline_literal() -> u8 {
    b'\n'
}

// Demonstrates byte string literal.
pub fn demo_byte_string_literal() -> &'static [u8; 5] {
    b"HELLO"
}

// Demonstrates iterating over bytes and summing values.
pub fn demo_byte_string_sum() -> u16 {
    let msg = demo_byte_string_literal();
    msg.iter().map(|value| *value as u16).sum()
}

// Runs all byte-literal demos and aggregates outputs.
pub fn run_byte_literal_demo() -> ByteLiteralDemoResults {
    let bytes = demo_byte_string_literal();
    ByteLiteralDemoResults {
        ascii_a: demo_single_byte_literal(),
        newline: demo_escaped_newline_literal(),
        bytes_len: bytes.len(),
        bytes_first: bytes[0],
        hello_sum: demo_byte_string_sum(),
    }
}

#[cfg(test)]
mod tests {
    // Import module APIs for tests.
    use super::{
        byte_literal_topics, demo_byte_string_literal, demo_byte_string_sum,
        demo_escaped_newline_literal, demo_single_byte_literal, run_byte_literal_demo,
    };

    // Verifies topic list length.
    #[test]
    fn has_expected_topic_count() {
        assert_eq!(byte_literal_topics().len(), 4);
    }

    // Verifies ASCII value of `b'A'`.
    #[test]
    fn single_byte_literal_is_ascii_a() {
        assert_eq!(demo_single_byte_literal(), 65);
    }

    // Verifies newline byte value.
    #[test]
    fn escaped_newline_literal_is_ten() {
        assert_eq!(demo_escaped_newline_literal(), 10);
    }

    // Verifies byte string demo content.
    #[test]
    fn byte_string_demo_is_hello() {
        assert_eq!(demo_byte_string_literal(), b"HELLO");
    }

    // Verifies sum of HELLO byte values.
    #[test]
    fn byte_string_sum_matches_expected() {
        assert_eq!(demo_byte_string_sum(), 372);
    }

    // Verifies aggregate summary correctness.
    #[test]
    fn aggregate_demo_result_is_correct() {
        let summary = run_byte_literal_demo();
        assert_eq!(summary.ascii_a, 65);
        assert_eq!(summary.newline, 10);
        assert_eq!(summary.bytes_len, 5);
        assert_eq!(summary.bytes_first, b'H');
        assert_eq!(summary.hello_sum, 372);
    }
}

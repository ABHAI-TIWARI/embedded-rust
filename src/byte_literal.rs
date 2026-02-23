#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteLiteralTopic {
    pub title: &'static str,
    pub rust_example: &'static str,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByteLiteralDemoResults {
    pub ascii_a: u8,
    pub newline: u8,
    pub bytes_len: usize,
    pub bytes_first: u8,
    pub hello_sum: u16,
}

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

pub fn byte_literal_topics() -> &'static [ByteLiteralTopic] {
    &TOPICS
}

pub fn demo_single_byte_literal() -> u8 {
    b'A'
}

pub fn demo_escaped_newline_literal() -> u8 {
    b'\n'
}

pub fn demo_byte_string_literal() -> &'static [u8; 5] {
    b"HELLO"
}

pub fn demo_byte_string_sum() -> u16 {
    let msg = demo_byte_string_literal();
    msg.iter().map(|value| *value as u16).sum()
}

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
    use super::{
        byte_literal_topics, demo_byte_string_literal, demo_byte_string_sum,
        demo_escaped_newline_literal, demo_single_byte_literal, run_byte_literal_demo,
    };

    #[test]
    fn has_expected_topic_count() {
        assert_eq!(byte_literal_topics().len(), 4);
    }

    #[test]
    fn single_byte_literal_is_ascii_a() {
        assert_eq!(demo_single_byte_literal(), 65);
    }

    #[test]
    fn escaped_newline_literal_is_ten() {
        assert_eq!(demo_escaped_newline_literal(), 10);
    }

    #[test]
    fn byte_string_demo_is_hello() {
        assert_eq!(demo_byte_string_literal(), b"HELLO");
    }

    #[test]
    fn byte_string_sum_matches_expected() {
        assert_eq!(demo_byte_string_sum(), 372);
    }

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

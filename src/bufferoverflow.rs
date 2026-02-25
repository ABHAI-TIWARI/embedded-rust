// Buffer overflow tutorial module for embedded Rust learners coming from C.
// Goal: demonstrate how Rust safe code prevents classic out-of-bounds writes.

// Each topic entry is printed from main.rs as a mini lesson.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferOverflowTopic {
    // Topic heading.
    pub title: &'static str,
    // Tiny Rust snippet shown in logs.
    pub rust_example: &'static str,
    // Explanation that includes C comparison.
    pub explanation: &'static str,
}

// Aggregate output from executable demo functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferOverflowDemoResults {
    // Number of bytes copied using safe bounded logic.
    pub safe_copied_len: usize,
    // Indicates truncated copy happened instead of overflow.
    pub copy_was_truncated: bool,
    // Result from checked index write demo.
    pub checked_write_ok: bool,
    // Result from checked index write failure demo.
    pub checked_write_fail: bool,
}

// Tutorial topics that explain overflow prevention.
const TOPICS: [BufferOverflowTopic; 5] = [
    BufferOverflowTopic {
        title: "What is buffer overflow",
        rust_example: "let mut buf = [0u8; 8];",
        explanation: "In C, writing past `buf[7]` can corrupt memory. Rust safe code blocks this pattern.",
    },
    BufferOverflowTopic {
        title: "Array indexing checks",
        rust_example: "buf[index]",
        explanation: "Rust performs bounds checks in safe indexing. C usually does not check automatically.",
    },
    BufferOverflowTopic {
        title: "Safe bounded copy",
        rust_example: "let n = min(src.len(), dst.len());",
        explanation: "Rust style computes safe copy length first; C often relies on careful manual counting.",
    },
    BufferOverflowTopic {
        title: "checked access APIs",
        rust_example: "buf.get_mut(index)",
        explanation: "`get_mut` returns Option, so out-of-range access is handled safely instead of memory corruption.",
    },
    BufferOverflowTopic {
        title: "No hidden pointer arithmetic",
        rust_example: "for i in 0..len { dst[i] = src[i]; }",
        explanation: "Rust safe loops avoid raw pointer arithmetic by default; C pointer math can be error-prone.",
    },
];

// Returns all tutorial topics.
pub fn bufferoverflow_topics() -> &'static [BufferOverflowTopic] {
    &TOPICS
}

// Safe bounded copy demo.
// C comparison:
// - Unsafe C style might call memcpy(dst, src, src_len) without checking dst size.
// - This Rust version always copies only the minimum valid length.
pub fn demo_safe_bounded_copy(src: &[u8], dst: &mut [u8]) -> usize {
    let copy_len = core::cmp::min(src.len(), dst.len());
    let mut i = 0;
    while i < copy_len {
        dst[i] = src[i];
        i += 1;
    }
    copy_len
}

// Checked single-byte write demo.
// Returns true when write succeeded, false when index was out of range.
// C comparison:
// - In C: dst[index] = value; could overflow if index is invalid.
// - In Rust: get_mut(index) forces explicit range handling via Option.
pub fn demo_checked_index_write(dst: &mut [u8], index: usize, value: u8) -> bool {
    if let Some(slot) = dst.get_mut(index) {
        *slot = value;
        true
    } else {
        false
    }
}

// Runs practical demos and returns summary data for startup logs.
pub fn run_bufferoverflow_demo() -> BufferOverflowDemoResults {
    // Source has 10 bytes.
    let src = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Destination has only 6 bytes.
    let mut dst = [0u8; 6];

    let copied = demo_safe_bounded_copy(&src, &mut dst);
    let truncated = src.len() > dst.len();

    let write_ok = demo_checked_index_write(&mut dst, 2, 99);
    let write_fail = !demo_checked_index_write(&mut dst, 100, 55);

    BufferOverflowDemoResults {
        safe_copied_len: copied,
        copy_was_truncated: truncated,
        checked_write_ok: write_ok,
        checked_write_fail: write_fail,
    }
}

#[cfg(test)]
mod tests {
    // Import public APIs for unit tests.
    use super::{
        bufferoverflow_topics, demo_checked_index_write, demo_safe_bounded_copy,
        run_bufferoverflow_demo,
    };

    // Verify topic count remains stable.
    #[test]
    fn has_expected_topic_count() {
        assert_eq!(bufferoverflow_topics().len(), 5);
    }

    // Verify bounded copy copies min(src, dst) length.
    #[test]
    fn bounded_copy_truncates_safely() {
        let src = [10u8, 20, 30, 40, 50];
        let mut dst = [0u8; 3];
        let copied = demo_safe_bounded_copy(&src, &mut dst);
        assert_eq!(copied, 3);
        assert_eq!(dst, [10u8, 20, 30]);
    }

    // Verify checked write succeeds for valid index.
    #[test]
    fn checked_write_success() {
        let mut dst = [1u8, 2, 3];
        let ok = demo_checked_index_write(&mut dst, 1, 99);
        assert!(ok);
        assert_eq!(dst, [1u8, 99, 3]);
    }

    // Verify checked write fails safely for invalid index.
    #[test]
    fn checked_write_failure() {
        let mut dst = [1u8, 2, 3];
        let ok = demo_checked_index_write(&mut dst, 9, 77);
        assert!(!ok);
        assert_eq!(dst, [1u8, 2, 3]);
    }

    // Verify aggregate demo summary values.
    #[test]
    fn aggregate_demo_is_correct() {
        let summary = run_bufferoverflow_demo();
        assert_eq!(summary.safe_copied_len, 6);
        assert!(summary.copy_was_truncated);
        assert!(summary.checked_write_ok);
        assert!(summary.checked_write_fail);
    }
}

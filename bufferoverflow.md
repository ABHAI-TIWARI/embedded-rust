# Buffer Overflow Tutorial (Rust vs C) — Line-by-Line

This tutorial explains `src/bufferoverflow.rs` line-by-line for embedded C learners.

## 1) Why this tutorial exists

In C, buffer overflow is a common class of bugs:
- writing beyond array bounds,
- copying more bytes than destination can hold,
- unchecked index writes.

In Rust safe code, these operations are guarded by bounds checks and safe APIs.

---

## 2) Full file with numbered lines

```rust
01 // Buffer overflow tutorial module for embedded Rust learners coming from C.
02 // Goal: demonstrate how Rust safe code prevents classic out-of-bounds writes.
03 
04 // Each topic entry is printed from main.rs as a mini lesson.
05 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
06 pub struct BufferOverflowTopic {
07     // Topic heading.
08     pub title: &'static str,
09     // Tiny Rust snippet shown in logs.
10     pub rust_example: &'static str,
11     // Explanation that includes C comparison.
12     pub explanation: &'static str,
13 }
14 
15 // Aggregate output from executable demo functions.
16 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
17 pub struct BufferOverflowDemoResults {
18     // Number of bytes copied using safe bounded logic.
19     pub safe_copied_len: usize,
20     // Indicates truncated copy happened instead of overflow.
21     pub copy_was_truncated: bool,
22     // Result from checked index write demo.
23     pub checked_write_ok: bool,
24     // Result from checked index write failure demo.
25     pub checked_write_fail: bool,
26 }
27 
28 // Tutorial topics that explain overflow prevention.
29 const TOPICS: [BufferOverflowTopic; 5] = [
30     BufferOverflowTopic {
31         title: "What is buffer overflow",
32         rust_example: "let mut buf = [0u8; 8];",
33         explanation: "In C, writing past `buf[7]` can corrupt memory. Rust safe code blocks this pattern.",
34     },
35     BufferOverflowTopic {
36         title: "Array indexing checks",
37         rust_example: "buf[index]",
38         explanation: "Rust performs bounds checks in safe indexing. C usually does not check automatically.",
39     },
40     BufferOverflowTopic {
41         title: "Safe bounded copy",
42         rust_example: "let n = min(src.len(), dst.len());",
43         explanation: "Rust style computes safe copy length first; C often relies on careful manual counting.",
44     },
45     BufferOverflowTopic {
46         title: "checked access APIs",
47         rust_example: "buf.get_mut(index)",
48         explanation: "`get_mut` returns Option, so out-of-range access is handled safely instead of memory corruption.",
49     },
50     BufferOverflowTopic {
51         title: "No hidden pointer arithmetic",
52         rust_example: "for i in 0..len { dst[i] = src[i]; }",
53         explanation: "Rust safe loops avoid raw pointer arithmetic by default; C pointer math can be error-prone.",
54     },
55 ];
56 
57 // Returns all tutorial topics.
58 pub fn bufferoverflow_topics() -> &'static [BufferOverflowTopic] {
59     &TOPICS
60 }
61 
62 // Safe bounded copy demo.
63 // C comparison:
64 // - Unsafe C style might call memcpy(dst, src, src_len) without checking dst size.
65 // - This Rust version always copies only the minimum valid length.
66 pub fn demo_safe_bounded_copy(src: &[u8], dst: &mut [u8]) -> usize {
67     let copy_len = core::cmp::min(src.len(), dst.len());
68     let mut i = 0;
69     while i < copy_len {
70         dst[i] = src[i];
71         i += 1;
72     }
73     copy_len
74 }
75 
76 // Checked single-byte write demo.
77 // Returns true when write succeeded, false when index was out of range.
78 // C comparison:
79 // - In C: dst[index] = value; could overflow if index is invalid.
80 // - In Rust: get_mut(index) forces explicit range handling via Option.
81 pub fn demo_checked_index_write(dst: &mut [u8], index: usize, value: u8) -> bool {
82     if let Some(slot) = dst.get_mut(index) {
83         *slot = value;
84         true
85     } else {
86         false
87     }
88 }
89 
90 // Runs practical demos and returns summary data for startup logs.
91 pub fn run_bufferoverflow_demo() -> BufferOverflowDemoResults {
92     // Source has 10 bytes.
93     let src = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
94     // Destination has only 6 bytes.
95     let mut dst = [0u8; 6];
96 
97     let copied = demo_safe_bounded_copy(&src, &mut dst);
98     let truncated = src.len() > dst.len();
99 
100     let write_ok = demo_checked_index_write(&mut dst, 2, 99);
101     let write_fail = !demo_checked_index_write(&mut dst, 100, 55);
102 
103     BufferOverflowDemoResults {
104         safe_copied_len: copied,
105         copy_was_truncated: truncated,
106         checked_write_ok: write_ok,
107         checked_write_fail: write_fail,
108     }
109 }
110 
111 #[cfg(test)]
112 mod tests {
113     // Import public APIs for unit tests.
114     use super::{
115         bufferoverflow_topics, demo_checked_index_write, demo_safe_bounded_copy,
116         run_bufferoverflow_demo,
117     };
118 
119     // Verify topic count remains stable.
120     #[test]
121     fn has_expected_topic_count() {
122         assert_eq!(bufferoverflow_topics().len(), 5);
123     }
124 
125     // Verify bounded copy copies min(src, dst) length.
126     #[test]
127     fn bounded_copy_truncates_safely() {
128         let src = [10u8, 20, 30, 40, 50];
129         let mut dst = [0u8; 3];
130         let copied = demo_safe_bounded_copy(&src, &mut dst);
131         assert_eq!(copied, 3);
132         assert_eq!(dst, [10u8, 20, 30]);
133     }
134 
135     // Verify checked write succeeds for valid index.
136     #[test]
137     fn checked_write_success() {
138         let mut dst = [1u8, 2, 3];
139         let ok = demo_checked_index_write(&mut dst, 1, 99);
140         assert!(ok);
141         assert_eq!(dst, [1u8, 99, 3]);
142     }
143 
144     // Verify checked write fails safely for invalid index.
145     #[test]
146     fn checked_write_failure() {
147         let mut dst = [1u8, 2, 3];
148         let ok = demo_checked_index_write(&mut dst, 9, 77);
149         assert!(!ok);
150         assert_eq!(dst, [1u8, 2, 3]);
151     }
152 
153     // Verify aggregate demo summary values.
154     #[test]
155     fn aggregate_demo_is_correct() {
156         let summary = run_bufferoverflow_demo();
157         assert_eq!(summary.safe_copied_len, 6);
158         assert!(summary.copy_was_truncated);
159         assert!(summary.checked_write_ok);
160         assert!(summary.checked_write_fail);
161     }
162 }
```

---

## 3) Detailed explanation by section

### Lines 1-26: Data models
- `BufferOverflowTopic` stores text used for teaching output.
- `BufferOverflowDemoResults` stores numeric/boolean outcomes from practical demos.
- `derive(...)` allows easy debug printing and test equality comparisons.

### Lines 28-55: Teaching topics
- `TOPICS` is a fixed list of mini-lessons.
- Each item includes a Rust snippet and a C comparison.

### Lines 57-60: Topic accessor
- `bufferoverflow_topics()` returns a static slice for iteration in `main.rs`.

### Lines 62-74: Safe bounded copy
- Computes `copy_len = min(src.len(), dst.len())`.
- Copies only valid indices.
- Prevents destination overrun by design.

### Lines 76-88: Checked index write
- Uses `get_mut(index)` which returns `Option<&mut u8>`.
- `Some(slot)` means valid index; `None` means out-of-range.
- No undefined memory writes in safe Rust path.

### Lines 90-109: Aggregate demo
- Demonstrates truncated copy (safe behavior).
- Demonstrates valid and invalid writes with explicit result flags.

### Lines 111-162: Unit tests
- Validate topic count.
- Validate copy truncation behavior.
- Validate checked write success/failure behavior.
- Validate final summary struct output.

---

## 4) Embedded C comparison summary

- C risk: unchecked indexing and unchecked `memcpy` lengths can corrupt memory.
- Rust safe alternative: bounds checks + `Option` APIs + explicit length capping.
- Result: safer default behavior without raw pointer arithmetic in normal code.

---

## 5) Where it is called in firmware

`src/main.rs` now does:
- iterate `bufferoverflow_topics()` and log each concept,
- call `run_bufferoverflow_demo()` and log result values,
- then continue into LED blink superloop.

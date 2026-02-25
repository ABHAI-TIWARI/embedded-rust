# Rust Constants Tutorial (Line-by-Line)

This tutorial explains the `src/constants.rs` module in detail for an embedded-C learner moving to Rust.

File covered: `src/constants.rs`

## 1) Full Code (with line numbers)

```rust
01 // This struct represents one tutorial topic for Rust constants.
02 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
03 pub struct ConstantTopic {
04     // Topic title shown in logs.
05     pub title: &'static str,
06     // Example snippet displayed to learner.
07     pub rust_example: &'static str,
08     // Short explanation for embedded-C beginners.
09     pub explanation: &'static str,
10 }
11 
12 // This struct collects computed outputs from constant demos.
13 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
14 pub struct ConstantDemoResults {
15     // Board supply voltage constant.
16     pub supply_voltage_mv: u16,
17     // Retry limit constant.
18     pub max_retry_count: u8,
19     // Timeout constant.
20     pub uart_timeout_ms: u32,
21     // Static string banner constant.
22     pub firmware_banner: &'static str,
23     // Compile-time calculated constant result.
24     pub acquisition_window_ms: u32,
25     // Local function-level constant demo result.
26     pub local_constant_offset: u32,
27 }
28 
29 // Global compile-time constant: supply voltage in millivolts.
30 const SUPPLY_VOLTAGE_MV: u16 = 3300;
31 
32 // Global compile-time constant: max number of retries.
33 const MAX_RETRY_COUNT: u8 = 3;
34 
35 // Global compile-time constant: UART timeout in milliseconds.
36 const UART_TIMEOUT_MS: u32 = 1000;
37 
38 // Global compile-time constant string with static lifetime.
39 const FIRMWARE_BANNER: &str = "ESP32 Rust Constants Demo";
40 
41 // Two constants used in compile-time arithmetic expression.
42 const SAMPLE_TIME_MS: u32 = 25;
43 const SAMPLE_COUNT: u32 = 40;
44 
45 // This value is computed at compile time, not runtime.
46 const ACQUISITION_WINDOW_MS: u32 = SAMPLE_TIME_MS * SAMPLE_COUNT;
47 
48 // Topic list printed from main.rs for tutorial-style output.
49 const TOPICS: [ConstantTopic; 6] = [
50     ConstantTopic {
51         title: "What is a constant",
52         rust_example: "const SUPPLY_VOLTAGE_MV: u16 = 3300;",
53         explanation: "`const` values are inlined at compile time and cannot be modified.",
54     },
55     ConstantTopic {
56         title: "Naming convention",
57         rust_example: "const MAX_RETRY_COUNT: u8 = 3;",
58         explanation: "Rust style uses UPPER_CASE names for constants.",
59     },
60     ConstantTopic {
61         title: "Type is required",
62         rust_example: "const UART_TIMEOUT_MS: u32 = 1000;",
63         explanation: "Unlike many `let` variables, constants require explicit type annotation.",
64     },
65     ConstantTopic {
66         title: "String constants",
67         rust_example: "const FIRMWARE_BANNER: &str = \"...\";",
68         explanation: "String constants are usually `&'static str` and live for full program duration.",
69     },
70     ConstantTopic {
71         title: "Compile-time expressions",
72         rust_example: "const ACQUISITION_WINDOW_MS: u32 = SAMPLE_TIME_MS * SAMPLE_COUNT;",
73         explanation: "Rust evaluates constant expressions at compile time.",
74     },
75     ConstantTopic {
76         title: "Local function constants",
77         rust_example: "const LOCAL_OFFSET: u32 = 50;",
78         explanation: "You can define constants inside functions for scoped compile-time values.",
79     },
80 ];
81 
82 // Return all constant tutorial topics.
83 pub fn constant_topics() -> &'static [ConstantTopic] {
84     &TOPICS
85 }
86 
87 // Demo function returning a global constant.
88 pub fn demo_supply_voltage() -> u16 {
89     SUPPLY_VOLTAGE_MV
90 }
91 
92 // Demo function returning retry-count constant.
93 pub fn demo_max_retries() -> u8 {
94     MAX_RETRY_COUNT
95 }
96 
97 // Demo function returning timeout constant.
98 pub fn demo_uart_timeout() -> u32 {
99     UART_TIMEOUT_MS
100 }
101 
102 // Demo function returning string constant.
103 pub fn demo_firmware_banner() -> &'static str {
104     FIRMWARE_BANNER
105 }
106 
107 // Demo function returning compile-time computed constant.
108 pub fn demo_acquisition_window() -> u32 {
109     ACQUISITION_WINDOW_MS
110 }
111 
112 // Demo function showing function-scoped constant usage.
113 pub fn demo_local_constant() -> u32 {
114     // This constant exists only inside this function.
115     const LOCAL_OFFSET: u32 = 50;
116     // Return compile-time constant plus another value.
117     LOCAL_OFFSET + 10
118 }
119 
120 // Run all constant demos and aggregate them into one struct.
121 pub fn run_constants_demo() -> ConstantDemoResults {
122     ConstantDemoResults {
123         supply_voltage_mv: demo_supply_voltage(),
124         max_retry_count: demo_max_retries(),
125         uart_timeout_ms: demo_uart_timeout(),
126         firmware_banner: demo_firmware_banner(),
127         acquisition_window_ms: demo_acquisition_window(),
128         local_constant_offset: demo_local_constant(),
129     }
130 }
131 
132 #[cfg(test)]
133 mod tests {
134     // Import all public demo APIs for unit tests.
135     use super::{
136         constant_topics, demo_acquisition_window, demo_firmware_banner, demo_local_constant,
137         demo_max_retries, demo_supply_voltage, demo_uart_timeout, run_constants_demo,
138     };
139 
140     // Validate expected topic count.
141     #[test]
142     fn has_expected_topic_count() {
143         assert_eq!(constant_topics().len(), 6);
144     }
145 
146     // Validate basic numeric constants.
147     #[test]
148     fn numeric_constants_are_correct() {
149         assert_eq!(demo_supply_voltage(), 3300);
150         assert_eq!(demo_max_retries(), 3);
151         assert_eq!(demo_uart_timeout(), 1000);
152     }
153 
154     // Validate string constant.
155     #[test]
156     fn banner_constant_is_correct() {
157         assert_eq!(demo_firmware_banner(), "ESP32 Rust Constants Demo");
158     }
159 
160     // Validate compile-time expression result.
161     #[test]
162     fn acquisition_window_constant_is_correct() {
163         assert_eq!(demo_acquisition_window(), 1000);
164     }
165 
166     // Validate local-scoped constant demo.
167     #[test]
168     fn local_constant_demo_is_correct() {
169         assert_eq!(demo_local_constant(), 60);
170     }
171 
172     // Validate aggregate result struct fields.
173     #[test]
174     fn aggregate_constants_demo_is_correct() {
175         let summary = run_constants_demo();
176         assert_eq!(summary.supply_voltage_mv, 3300);
177         assert_eq!(summary.max_retry_count, 3);
178         assert_eq!(summary.uart_timeout_ms, 1000);
179         assert_eq!(summary.firmware_banner, "ESP32 Rust Constants Demo");
180         assert_eq!(summary.acquisition_window_ms, 1000);
181         assert_eq!(summary.local_constant_offset, 60);
182     }
183 }
```

## 2) Line-by-Line Explanation

- Lines 1-10: Define `ConstantTopic`, a small record used to print tutorial entries.
- Line 2: `derive` auto-adds debug printing, copying, and equality checks.
- Lines 12-27: Define `ConstantDemoResults`, which stores all demo outputs in one place.
- Lines 29-46: Declare global compile-time constants (`const`). These are immutable and known at compile time.
- Lines 41-46: Show compile-time expression constants (`SAMPLE_TIME_MS * SAMPLE_COUNT`).
- Lines 48-80: Build fixed tutorial topic array that `main.rs` can iterate and print.
- Lines 82-85: Return all topics as a static slice.
- Lines 87-110: Simple demo getters that expose constants.
- Lines 112-118: Function-level local constant (`LOCAL_OFFSET`) to show scoped constant usage.
- Lines 120-130: Run all demos and return one aggregate result struct.
- Lines 132-183: Unit tests to validate constants and results stay correct.

## 3) Key Embedded-C to Rust Mapping

- C `#define` / `const` values map to Rust `const` with explicit type.
- Rust constants are immutable and must always have a type.
- UPPER_CASE naming for constants is standard Rust style.
- Function-local constants are possible and useful for local compile-time values.

## 4) Where It Is Called

`src/main.rs` calls:
- `constant_topics()` to print tutorial sections
- `run_constants_demo()` to print computed constant-based outputs

This happens during startup before the LED blink superloop.

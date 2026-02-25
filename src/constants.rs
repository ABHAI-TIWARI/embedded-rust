// This struct represents one tutorial topic for Rust constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantTopic {
    // Topic title shown in logs.
    pub title: &'static str,
    // Example snippet displayed to learner.
    pub rust_example: &'static str,
    // Short explanation for embedded-C beginners.
    pub explanation: &'static str,
}

// This struct collects computed outputs from constant demos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstantDemoResults {
    // Board supply voltage constant.
    pub supply_voltage_mv: u16,
    // Retry limit constant.
    pub max_retry_count: u8,
    // Timeout constant.
    pub uart_timeout_ms: u32,
    // Static string banner constant.
    pub firmware_banner: &'static str,
    // Compile-time calculated constant result.
    pub acquisition_window_ms: u32,
    // Local function-level constant demo result.
    pub local_constant_offset: u32,
}

// Global compile-time constant: supply voltage in millivolts.
const SUPPLY_VOLTAGE_MV: u16 = 3300;

// Global compile-time constant: max number of retries.
const MAX_RETRY_COUNT: u8 = 3;

// Global compile-time constant: UART timeout in milliseconds.
const UART_TIMEOUT_MS: u32 = 1000;

// Global compile-time constant string with static lifetime.
const FIRMWARE_BANNER: &str = "ESP32 Rust Constants Demo";

// Two constants used in compile-time arithmetic expression.
const SAMPLE_TIME_MS: u32 = 25;
const SAMPLE_COUNT: u32 = 40;

// This value is computed at compile time, not runtime.
const ACQUISITION_WINDOW_MS: u32 = SAMPLE_TIME_MS * SAMPLE_COUNT;

// Topic list printed from main.rs for tutorial-style output.
const TOPICS: [ConstantTopic; 6] = [
    ConstantTopic {
        title: "What is a constant",
        rust_example: "const SUPPLY_VOLTAGE_MV: u16 = 3300;",
        explanation: "`const` values are inlined at compile time and cannot be modified.",
    },
    ConstantTopic {
        title: "Naming convention",
        rust_example: "const MAX_RETRY_COUNT: u8 = 3;",
        explanation: "Rust style uses UPPER_CASE names for constants.",
    },
    ConstantTopic {
        title: "Type is required",
        rust_example: "const UART_TIMEOUT_MS: u32 = 1000;",
        explanation: "Unlike many `let` variables, constants require explicit type annotation.",
    },
    ConstantTopic {
        title: "String constants",
        rust_example: "const FIRMWARE_BANNER: &str = \"...\";",
        explanation: "String constants are usually `&'static str` and live for full program duration.",
    },
    ConstantTopic {
        title: "Compile-time expressions",
        rust_example: "const ACQUISITION_WINDOW_MS: u32 = SAMPLE_TIME_MS * SAMPLE_COUNT;",
        explanation: "Rust evaluates constant expressions at compile time.",
    },
    ConstantTopic {
        title: "Local function constants",
        rust_example: "const LOCAL_OFFSET: u32 = 50;",
        explanation: "You can define constants inside functions for scoped compile-time values.",
    },
];

// Return all constant tutorial topics.
pub fn constant_topics() -> &'static [ConstantTopic] {
    &TOPICS
}

// Demo function returning a global constant.
pub fn demo_supply_voltage() -> u16 {
    SUPPLY_VOLTAGE_MV
}

// Demo function returning retry-count constant.
pub fn demo_max_retries() -> u8 {
    MAX_RETRY_COUNT
}

// Demo function returning timeout constant.
pub fn demo_uart_timeout() -> u32 {
    UART_TIMEOUT_MS
}

// Demo function returning string constant.
pub fn demo_firmware_banner() -> &'static str {
    FIRMWARE_BANNER
}

// Demo function returning compile-time computed constant.
pub fn demo_acquisition_window() -> u32 {
    ACQUISITION_WINDOW_MS
}

// Demo function showing function-scoped constant usage.
pub fn demo_local_constant() -> u32 {
    // This constant exists only inside this function.
    const LOCAL_OFFSET: u32 = 50;
    // Return compile-time constant plus another value.
    LOCAL_OFFSET + 10
}

// Run all constant demos and aggregate them into one struct.
pub fn run_constants_demo() -> ConstantDemoResults {
    ConstantDemoResults {
        supply_voltage_mv: demo_supply_voltage(),
        max_retry_count: demo_max_retries(),
        uart_timeout_ms: demo_uart_timeout(),
        firmware_banner: demo_firmware_banner(),
        acquisition_window_ms: demo_acquisition_window(),
        local_constant_offset: demo_local_constant(),
    }
}

#[cfg(test)]
mod tests {
    // Import all public demo APIs for unit tests.
    use super::{
        constant_topics, demo_acquisition_window, demo_firmware_banner, demo_local_constant,
        demo_max_retries, demo_supply_voltage, demo_uart_timeout, run_constants_demo,
    };

    // Validate expected topic count.
    #[test]
    fn has_expected_topic_count() {
        assert_eq!(constant_topics().len(), 6);
    }

    // Validate basic numeric constants.
    #[test]
    fn numeric_constants_are_correct() {
        assert_eq!(demo_supply_voltage(), 3300);
        assert_eq!(demo_max_retries(), 3);
        assert_eq!(demo_uart_timeout(), 1000);
    }

    // Validate string constant.
    #[test]
    fn banner_constant_is_correct() {
        assert_eq!(demo_firmware_banner(), "ESP32 Rust Constants Demo");
    }

    // Validate compile-time expression result.
    #[test]
    fn acquisition_window_constant_is_correct() {
        assert_eq!(demo_acquisition_window(), 1000);
    }

    // Validate local-scoped constant demo.
    #[test]
    fn local_constant_demo_is_correct() {
        assert_eq!(demo_local_constant(), 60);
    }

    // Validate aggregate result struct fields.
    #[test]
    fn aggregate_constants_demo_is_correct() {
        let summary = run_constants_demo();
        assert_eq!(summary.supply_voltage_mv, 3300);
        assert_eq!(summary.max_retry_count, 3);
        assert_eq!(summary.uart_timeout_ms, 1000);
        assert_eq!(summary.firmware_banner, "ESP32 Rust Constants Demo");
        assert_eq!(summary.acquisition_window_ms, 1000);
        assert_eq!(summary.local_constant_offset, 60);
    }
}

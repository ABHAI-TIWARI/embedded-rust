use core::sync::atomic::{AtomicU32, Ordering};

// This struct describes one tutorial topic that will be printed in logs.
// Each topic has a title, a short Rust example, and an explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticVariableTopic {
    // Human-readable topic heading.
    pub title: &'static str,
    // Minimal code snippet for the topic.
    pub rust_example: &'static str,
    // Beginner-friendly explanation shown in firmware logs/docs.
    pub explanation: &'static str,
}

// This struct stores computed results from executable demo functions.
// It helps keep the tutorial testable and easy to log from main.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticVariableDemoResults {
    // Result of local variable demo.
    pub local_counter: u32,
    // Current value of global static counter.
    pub global_boot_count: u32,
    // Indicates naming convention demo succeeded.
    pub snake_case_ok: bool,
    // Value returned from UPPER_CASE constant demo.
    pub constant_limit: u16,
    // Type name string used to illustrate CamelCase type naming.
    pub type_name: &'static str,
}

// Global static counter used to demonstrate shared global state safely.
// AtomicU32 is used instead of static mut to avoid data race issues.
static GLOBAL_BOOT_COUNT: AtomicU32 = AtomicU32::new(0);

// UPPER_CASE constant naming example.
const MAX_SENSOR_LIMIT: u16 = 4095;

// CamelCase type naming example.
struct DeviceStatus;

// Fixed tutorial topic list printed at startup.
const TOPICS: [StaticVariableTopic; 4] = [
    StaticVariableTopic {
        title: "Local Variable",
        rust_example: "let local_counter: u32 = 1;",
        explanation: "Local variables live inside function scope and are dropped when the function returns.",
    },
    StaticVariableTopic {
        title: "Global Static Variable",
        rust_example: "static GLOBAL_BOOT_COUNT: AtomicU32 = AtomicU32::new(0);",
        explanation: "Global static data lives for entire program lifetime; atomics are used for safe shared updates.",
    },
    StaticVariableTopic {
        title: "Naming Convention",
        rust_example: "snake_case for variables/functions, UPPER_CASE for constants, CamelCase for types",
        explanation: "Rust style conventions improve readability and consistency across modules.",
    },
    StaticVariableTopic {
        title: "Static Lifetime Concept",
        rust_example: "&'static str values are valid for program lifetime",
        explanation: "String literals and static items commonly use `'static` lifetime in embedded apps.",
    },
];

// Returns all static-variable tutorial topics.
pub fn staticvariable_topics() -> &'static [StaticVariableTopic] {
    &TOPICS
}

// Demonstrates a local variable created inside function scope.
// Local variable lives only during this function call.
pub fn demo_local_variable() -> u32 {
    let local_counter: u32 = 1;
    local_counter + 1
}

// Demonstrates a global static variable update.
// fetch_add increments atomically and returns old value, so +1 gives new value.
pub fn demo_global_static_increment() -> u32 {
    GLOBAL_BOOT_COUNT.fetch_add(1, Ordering::SeqCst) + 1
}

// Demonstrates snake_case naming for local variables.
pub fn demo_naming_convention() -> bool {
    let sensor_value_mv: u16 = 3300;
    sensor_value_mv > 0
}

// Demonstrates reading a UPPER_CASE constant.
pub fn demo_constant_naming() -> u16 {
    MAX_SENSOR_LIMIT
}

// Demonstrates CamelCase type naming by returning the type's full name.
pub fn demo_type_naming() -> &'static str {
    core::any::type_name::<DeviceStatus>()
}

// Runs all demos and aggregates their outputs into one result object.
// main.rs uses this object to print structured tutorial logs.
pub fn run_staticvariable_demo() -> StaticVariableDemoResults {
    StaticVariableDemoResults {
        local_counter: demo_local_variable(),
        global_boot_count: demo_global_static_increment(),
        snake_case_ok: demo_naming_convention(),
        constant_limit: demo_constant_naming(),
        type_name: demo_type_naming(),
    }
}

#[cfg(test)]
mod tests {
    // Bring demo APIs into test scope.
    use super::{
        demo_constant_naming, demo_global_static_increment, demo_local_variable,
        demo_naming_convention, demo_type_naming, run_staticvariable_demo, staticvariable_topics,
    };

    // Verifies tutorial topic list length stays as expected.
    #[test]
    fn has_expected_topic_count() {
        assert_eq!(staticvariable_topics().len(), 4);
    }

    // Verifies local variable demo returns deterministic value.
    #[test]
    fn local_variable_demo_is_correct() {
        assert_eq!(demo_local_variable(), 2);
    }

    // Verifies global static atomic counter increments safely and monotonically.
    #[test]
    fn global_static_counter_increments() {
        let first = demo_global_static_increment();
        let second = demo_global_static_increment();
        assert_eq!(second, first + 1);
    }

    // Verifies naming convention demo logic returns true.
    #[test]
    fn naming_demo_is_true() {
        assert!(demo_naming_convention());
    }

    // Verifies constant demo exposes expected limit value.
    #[test]
    fn constant_naming_demo_is_correct() {
        assert_eq!(demo_constant_naming(), 4095);
    }

    // Verifies CamelCase type name appears in reflected type path string.
    #[test]
    fn type_name_contains_device_status() {
        assert!(demo_type_naming().contains("DeviceStatus"));
    }

    // Verifies full aggregate demo output contains expected values.
    #[test]
    fn aggregate_demo_returns_valid_data() {
        let summary = run_staticvariable_demo();
        assert_eq!(summary.local_counter, 2);
        assert!(summary.global_boot_count >= 1);
        assert!(summary.snake_case_ok);
        assert_eq!(summary.constant_limit, 4095);
        assert!(summary.type_name.contains("DeviceStatus"));
    }
}

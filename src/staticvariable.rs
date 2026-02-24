use core::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticVariableTopic {
    pub title: &'static str,
    pub rust_example: &'static str,
    pub explanation: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticVariableDemoResults {
    pub local_counter: u32,
    pub global_boot_count: u32,
    pub snake_case_ok: bool,
    pub constant_limit: u16,
    pub type_name: &'static str,
}

static GLOBAL_BOOT_COUNT: AtomicU32 = AtomicU32::new(0);
const MAX_SENSOR_LIMIT: u16 = 4095;

struct DeviceStatus;

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

pub fn staticvariable_topics() -> &'static [StaticVariableTopic] {
    &TOPICS
}

pub fn demo_local_variable() -> u32 {
    let local_counter: u32 = 1;
    local_counter + 1
}

pub fn demo_global_static_increment() -> u32 {
    GLOBAL_BOOT_COUNT.fetch_add(1, Ordering::SeqCst) + 1
}

pub fn demo_naming_convention() -> bool {
    let sensor_value_mv: u16 = 3300;
    sensor_value_mv > 0
}

pub fn demo_constant_naming() -> u16 {
    MAX_SENSOR_LIMIT
}

pub fn demo_type_naming() -> &'static str {
    core::any::type_name::<DeviceStatus>()
}

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
    use super::{
        demo_constant_naming, demo_global_static_increment, demo_local_variable,
        demo_naming_convention, demo_type_naming, run_staticvariable_demo, staticvariable_topics,
    };

    #[test]
    fn has_expected_topic_count() {
        assert_eq!(staticvariable_topics().len(), 4);
    }

    #[test]
    fn local_variable_demo_is_correct() {
        assert_eq!(demo_local_variable(), 2);
    }

    #[test]
    fn global_static_counter_increments() {
        let first = demo_global_static_increment();
        let second = demo_global_static_increment();
        assert_eq!(second, first + 1);
    }

    #[test]
    fn naming_demo_is_true() {
        assert!(demo_naming_convention());
    }

    #[test]
    fn constant_naming_demo_is_correct() {
        assert_eq!(demo_constant_naming(), 4095);
    }

    #[test]
    fn type_name_contains_device_status() {
        assert!(demo_type_naming().contains("DeviceStatus"));
    }

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

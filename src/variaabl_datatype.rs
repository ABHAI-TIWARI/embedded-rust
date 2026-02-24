// Variable and data type tutorial module with concrete executable examples.
// Designed to be modular and testable, similar to memory_safety.rs.

// `derive(...)` here gives debug print support, copy/clone behavior,
// and equality checks for assertions in unit tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableDatatypeFeature {
    // Topic heading for tutorial logs.
    pub topic: &'static str,
    // Minimal Rust snippet for this concept.
    pub rust_example: &'static str,
    // C/C++ comparison note for context.
    pub c_cpp_note: &'static str,
}

// Same derive set for aggregate result struct so tests can compare/inspect easily.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableDatatypeDemoResults {
    // Result from immutability demo.
    pub immutable_value: i32,
    // Result from mutability demo.
    pub mutable_counter: u32,
    // Result from typed numeric variable demo.
    pub voltage_mv: u16,
    // Result from type inference demo.
    pub inferred_duty_cycle: u8,
    // Indicates whether Option had a value.
    pub option_has_value: bool,
    // Indicates overflow handling branch.
    pub checked_add_overflowed: bool,
    // Tuple-grouped values demo result.
    pub tuple_group: (u16, u16, bool),
    // Result from underscore-unused variable demo.
    pub active_channel: u8,
}

// Static list of tutorial topics.
const FEATURES: [VariableDatatypeFeature; 8] = [
    VariableDatatypeFeature {
        topic: "Immutability by Default",
        rust_example: "let x: i32 = 10; // cannot be reassigned without mut",
        c_cpp_note: "C/C++ variables are mutable by default, so accidental modification is easier.",
    },
    VariableDatatypeFeature {
        topic: "Explicit Mutability",
        rust_example: "let mut counter: u32 = 0; counter += 1;",
        c_cpp_note: "Rust makes mutability explicit in the declaration.",
    },
    VariableDatatypeFeature {
        topic: "Strong Static Typing",
        rust_example: "let voltage_mv: u16 = 3300;",
        c_cpp_note: "Rust enforces strict typing and avoids implicit unsafe behavior.",
    },
    VariableDatatypeFeature {
        topic: "Type Inference with Safety",
        rust_example: "let duty_cycle = 128u8; // type inferred safely",
        c_cpp_note: "Inference exists in C++, but Rust keeps strict compile-time checks.",
    },
    VariableDatatypeFeature {
        topic: "Option Instead of Null",
        rust_example: "let sensor_value: Option<u16> = None;",
        c_cpp_note: "C/C++ often uses null/sentinel values, which are easier to misuse.",
    },
    VariableDatatypeFeature {
        topic: "Checked Integer Handling",
        rust_example: "let total = a.checked_add(b); // Option<T>",
        c_cpp_note: "C/C++ overflow may be silent or undefined depending on type/rules.",
    },
    VariableDatatypeFeature {
        topic: "Tuple Initialization for Grouped Variables",
        rust_example: "let sensor_group: (u16, u16, bool) = (3300, 120, true);",
        c_cpp_note: "In C/C++, grouped values often require custom struct setup; tuples give quick typed grouping.",
    },
    VariableDatatypeFeature {
        topic: "Underscore Prefix for Unused Variables",
        rust_example: "let _raw_adc: u16 = 512; // no unused-variable warning",
        c_cpp_note: "Rust warns on unused bindings by default; prefix `_` marks intentional non-use.",
    },
];

// Returns all variable/data-type tutorial topics.
pub fn variable_datatype_features() -> &'static [VariableDatatypeFeature] {
    &FEATURES
}

// Returns topic count.
pub fn topic_count() -> usize {
    FEATURES.len()
}

// Finds one topic by exact name.
pub fn topic_by_name(name: &str) -> Option<&'static VariableDatatypeFeature> {
    FEATURES.iter().find(|item| item.topic == name)
}

// Demonstrates immutability by default.
pub fn demo_immutability() -> i32 {
    let value: i32 = 10;
    value
}

// Demonstrates explicit mutability via `mut`.
pub fn demo_mutability() -> u32 {
    let mut counter: u32 = 0;
    counter += 1;
    counter += 1;
    counter
}

// Demonstrates explicit numeric typing.
pub fn demo_strong_typing() -> u16 {
    let voltage_mv: u16 = 3300;
    voltage_mv
}

// Demonstrates type inference with suffix.
pub fn demo_type_inference() -> u8 {
    let duty_cycle = 128u8;
    duty_cycle
}

// Demonstrates Option instead of null-like value.
pub fn demo_option_instead_of_null() -> Option<u16> {
    let sensor_value: Option<u16> = None;
    sensor_value
}

// Demonstrates checked addition overflow handling.
pub fn demo_checked_add_overflow() -> Option<u8> {
    let left: u8 = 250;
    let right: u8 = 10;
    left.checked_add(right)
}

// Demonstrates tuple initialization for grouped values.
pub fn demo_tuple_group_initialization() -> (u16, u16, bool) {
    let sensor_group: (u16, u16, bool) = (3300, 120, true);
    sensor_group
}

// Demonstrates underscore prefix for intentionally unused variable.
pub fn demo_underscore_unused_variable() -> u8 {
    let _raw_adc: u16 = 512;
    let active_channel: u8 = 2;
    active_channel
}

// Runs all demos and aggregates outputs.
pub fn run_variable_datatype_demo() -> VariableDatatypeDemoResults {
    VariableDatatypeDemoResults {
        immutable_value: demo_immutability(),
        mutable_counter: demo_mutability(),
        voltage_mv: demo_strong_typing(),
        inferred_duty_cycle: demo_type_inference(),
        option_has_value: demo_option_instead_of_null().is_some(),
        checked_add_overflowed: demo_checked_add_overflow().is_none(),
        tuple_group: demo_tuple_group_initialization(),
        active_channel: demo_underscore_unused_variable(),
    }
}

#[cfg(test)]
mod tests {
    // Import functions under test.
    use super::{
        demo_checked_add_overflow, demo_immutability, demo_mutability,
        demo_option_instead_of_null, demo_strong_typing, demo_tuple_group_initialization,
        demo_type_inference, demo_underscore_unused_variable, run_variable_datatype_demo,
        topic_by_name, topic_count, variable_datatype_features,
    };

    // Verifies expected topic count.
    #[test]
    fn has_expected_topics() {
        assert_eq!(topic_count(), 8);
    }

    // Verifies known topic lookup succeeds.
    #[test]
    fn finds_existing_topic() {
        assert!(topic_by_name("Immutability by Default").is_some());
    }

    // Verifies unknown topic lookup fails safely.
    #[test]
    fn returns_none_for_missing_topic() {
        assert!(topic_by_name("Dynamic Typing").is_none());
    }

    // Verifies all topic records are fully populated.
    #[test]
    fn topic_fields_are_not_empty() {
        for topic in variable_datatype_features() {
            assert!(!topic.topic.is_empty());
            assert!(!topic.rust_example.is_empty());
            assert!(!topic.c_cpp_note.is_empty());
        }
    }

    // Verifies all executable demo functions return expected values.
    #[test]
    fn executable_examples_produce_expected_values() {
        assert_eq!(demo_immutability(), 10);
        assert_eq!(demo_mutability(), 2);
        assert_eq!(demo_strong_typing(), 3300);
        assert_eq!(demo_type_inference(), 128u8);
        assert_eq!(demo_option_instead_of_null(), None);
        assert!(demo_checked_add_overflow().is_none());
        assert_eq!(demo_tuple_group_initialization(), (3300, 120, true));
        assert_eq!(demo_underscore_unused_variable(), 2);
    }

    // Verifies aggregate summary has consistent values.
    #[test]
    fn aggregate_demo_summary_is_correct() {
        let summary = run_variable_datatype_demo();
        assert_eq!(summary.immutable_value, 10);
        assert_eq!(summary.mutable_counter, 2);
        assert_eq!(summary.voltage_mv, 3300);
        assert_eq!(summary.inferred_duty_cycle, 128);
        assert!(!summary.option_has_value);
        assert!(summary.checked_add_overflowed);
        assert_eq!(summary.tuple_group, (3300, 120, true));
        assert_eq!(summary.active_channel, 2);
    }
}

// Variable and data type tutorial module with concrete executable examples.
// Designed to be modular and testable, similar to memory_safety.rs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableDatatypeFeature {
    pub topic: &'static str,
    pub rust_example: &'static str,
    pub c_cpp_note: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VariableDatatypeDemoResults {
    pub immutable_value: i32,
    pub mutable_counter: u32,
    pub voltage_mv: u16,
    pub inferred_duty_cycle: u8,
    pub option_has_value: bool,
    pub checked_add_overflowed: bool,
    pub tuple_group: (u16, u16, bool),
    pub active_channel: u8,
}

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

pub fn variable_datatype_features() -> &'static [VariableDatatypeFeature] {
    &FEATURES
}

pub fn topic_count() -> usize {
    FEATURES.len()
}

pub fn topic_by_name(name: &str) -> Option<&'static VariableDatatypeFeature> {
    FEATURES.iter().find(|item| item.topic == name)
}

pub fn demo_immutability() -> i32 {
    let value: i32 = 10;
    value
}

pub fn demo_mutability() -> u32 {
    let mut counter: u32 = 0;
    counter += 1;
    counter += 1;
    counter
}

pub fn demo_strong_typing() -> u16 {
    let voltage_mv: u16 = 3300;
    voltage_mv
}

pub fn demo_type_inference() -> u8 {
    let duty_cycle = 128u8;
    duty_cycle
}

pub fn demo_option_instead_of_null() -> Option<u16> {
    let sensor_value: Option<u16> = None;
    sensor_value
}

pub fn demo_checked_add_overflow() -> Option<u8> {
    let left: u8 = 250;
    let right: u8 = 10;
    left.checked_add(right)
}

pub fn demo_tuple_group_initialization() -> (u16, u16, bool) {
    let sensor_group: (u16, u16, bool) = (3300, 120, true);
    sensor_group
}

pub fn demo_underscore_unused_variable() -> u8 {
    let _raw_adc: u16 = 512;
    let active_channel: u8 = 2;
    active_channel
}

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
    use super::{
        demo_checked_add_overflow, demo_immutability, demo_mutability,
        demo_option_instead_of_null, demo_strong_typing, demo_tuple_group_initialization,
        demo_type_inference, demo_underscore_unused_variable, run_variable_datatype_demo,
        topic_by_name, topic_count, variable_datatype_features,
    };

    #[test]
    fn has_expected_topics() {
        assert_eq!(topic_count(), 8);
    }

    #[test]
    fn finds_existing_topic() {
        assert!(topic_by_name("Immutability by Default").is_some());
    }

    #[test]
    fn returns_none_for_missing_topic() {
        assert!(topic_by_name("Dynamic Typing").is_none());
    }

    #[test]
    fn topic_fields_are_not_empty() {
        for topic in variable_datatype_features() {
            assert!(!topic.topic.is_empty());
            assert!(!topic.rust_example.is_empty());
            assert!(!topic.c_cpp_note.is_empty());
        }
    }

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

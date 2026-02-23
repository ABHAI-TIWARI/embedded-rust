// This module is intentionally data-driven so it is easy to test and reuse.
// It explains Rust memory safety features by contrasting them with common
// C/C++ failure modes.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyFeature {
    pub title: &'static str,
    pub rust_guarantee: &'static str,
    pub c_cpp_risk: &'static str,
}

const FEATURES: [SafetyFeature; 5] = [
    SafetyFeature {
        title: "Ownership",
        rust_guarantee: "Each value has one owner; moving transfers ownership and prevents double free.",
        c_cpp_risk: "Manual ownership conventions can be violated, leading to double free or leaks.",
    },
    SafetyFeature {
        title: "Borrowing Rules",
        rust_guarantee: "At any time: many immutable references OR one mutable reference.",
        c_cpp_risk: "Aliasing mutable pointers/references can cause data races and undefined behavior.",
    },
    SafetyFeature {
        title: "Lifetimes",
        rust_guarantee: "References cannot outlive the data they point to.",
        c_cpp_risk: "Dangling pointers can occur when returning/storing addresses to freed stack or heap data.",
    },
    SafetyFeature {
        title: "Bounds-Checked Access",
        rust_guarantee: "Safe indexing and slicing validate bounds before access.",
        c_cpp_risk: "Out-of-bounds array access may silently corrupt memory.",
    },
    SafetyFeature {
        title: "No Null References",
        rust_guarantee: "Use Option<T> to represent absence explicitly and force handling.",
        c_cpp_risk: "Raw/null pointer dereference can crash or cause undefined behavior.",
    },
];

pub fn memory_safety_features() -> &'static [SafetyFeature] {
    &FEATURES
}

pub fn lesson_count() -> usize {
    FEATURES.len()
}

pub fn feature_by_title(title: &str) -> Option<&'static SafetyFeature> {
    FEATURES.iter().find(|feature| feature.title == title)
}

#[cfg(test)]
mod tests {
    use super::{feature_by_title, lesson_count, memory_safety_features};

    #[test]
    fn has_expected_number_of_lessons() {
        assert_eq!(lesson_count(), 5);
    }

    #[test]
    fn ownership_lesson_exists() {
        let feature = feature_by_title("Ownership");
        assert!(feature.is_some());
    }

    #[test]
    fn unknown_lesson_returns_none() {
        let feature = feature_by_title("Garbage Collector");
        assert!(feature.is_none());
    }

    #[test]
    fn every_lesson_has_complete_text() {
        for feature in memory_safety_features() {
            assert!(!feature.title.is_empty());
            assert!(!feature.rust_guarantee.is_empty());
            assert!(!feature.c_cpp_risk.is_empty());
        }
    }
}

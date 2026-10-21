//! Regression tests for build-time live test selection.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod live_test_selection {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/build/live_test_selection.rs"
    ));
}

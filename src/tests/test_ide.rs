use crate::tests::{test_ide_contains, TestResult};

#[test]
fn parser_recovers() -> TestResult {
    test_ide_contains(
        "3 + \"bob\"\nlet x = \"fred\"\n",
        &["--ide-check"],
        "\"typename\":\"string\"",
    )
}

use conformance_core::result::{Status as CoreStatus, TestResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Pass,
    Fail,
    Error,
    Skipped,
}

impl From<&CoreStatus> for Status {
    fn from(s: &CoreStatus) -> Self {
        match s {
            CoreStatus::Pass => Status::Pass,
            CoreStatus::Fail => Status::Fail,
            CoreStatus::Error => Status::Error,
            CoreStatus::Skipped => Status::Skipped,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScenarioResult {
    pub test_id: String,
    pub description: String,
    pub status: Status,
    pub expected_behavior: String,
    pub observed_behavior: String,
}

impl From<&TestResult> for ScenarioResult {
    fn from(r: &TestResult) -> Self {
        ScenarioResult {
            test_id: r.test_id.to_string(),
            description: r.description.to_string(),
            status: Status::from(&r.status),
            expected_behavior: r.expected_behavior.to_string(),
            observed_behavior: r.observed_behavior.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Summary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub errors: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConformanceReport {
    pub profile: String,
    pub fixture: String,
    pub status: Status,
    pub summary: Summary,
    pub results: Vec<ScenarioResult>,
}

impl ConformanceReport {
    pub fn new(profile: &str, fixture: &str, results: &[TestResult]) -> Self {
        let mut passed = 0;
        let mut failed = 0;
        let mut errors = 0;
        let mut skipped = 0;

        let mut scenarios = Vec::with_capacity(results.len());

        for r in results {
            scenarios.push(ScenarioResult::from(r));
            match r.status {
                CoreStatus::Pass => passed += 1,
                CoreStatus::Fail => failed += 1,
                CoreStatus::Error => errors += 1,
                CoreStatus::Skipped => skipped += 1,
            }
        }

        let total = passed + failed + errors + skipped;

        // Define deterministic overall result
        let overall_status = if errors > 0 {
            Status::Error
        } else if failed > 0 {
            Status::Fail
        } else if total == 0 || total == skipped {
            Status::Skipped
        } else {
            Status::Pass
        };

        ConformanceReport {
            profile: profile.to_string(),
            fixture: fixture.to_string(),
            summary: Summary {
                total,
                passed,
                failed,
                errors,
                skipped,
            },
            status: overall_status,
            results: scenarios,
        }
    }

    pub fn to_human_readable(&self) -> String {
        let mut out = String::new();
        out.push_str("Stellar Contract Conformance Report\n\n");
        out.push_str(&format!("Profile: {}\n", self.profile));
        out.push_str(&format!("Fixture: {}\n\n", self.fixture));

        for s in &self.results {
            let status_str = match s.status {
                Status::Pass => "PASS",
                Status::Fail => "FAIL",
                Status::Error => "ERROR",
                Status::Skipped => "SKIP",
            };
            out.push_str(&format!(
                "{:5}  {:20}  {}\n",
                status_str, s.test_id, s.description
            ));
            if s.status == Status::Fail || s.status == Status::Error {
                out.push_str(&format!("         Requirement: {}\n", s.expected_behavior));
                out.push_str(&format!("         Message:     {}\n", s.observed_behavior));
            }
        }

        out.push_str("\nSummary:\n");
        out.push_str(&format!("  Total: {}\n", self.summary.total));
        out.push_str(&format!("  Passed: {}\n", self.summary.passed));
        out.push_str(&format!("  Failed: {}\n", self.summary.failed));
        out.push_str(&format!("  Errors: {}\n", self.summary.errors));
        out.push_str(&format!("  Skipped: {}\n\n", self.summary.skipped));

        let overall_str = match self.status {
            Status::Pass => "PASS",
            Status::Fail => "FAIL",
            Status::Error => "ERROR",
            Status::Skipped => "SKIPPED",
        };
        out.push_str(&format!("Overall: {}\n", overall_str));

        out
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_result(status: CoreStatus) -> TestResult {
        TestResult {
            test_id: "TEST-001",
            description: "Dummy test",
            status,
            expected_behavior: "Expected",
            observed_behavior: "Observed",
        }
    }

    #[test]
    fn test_overall_status_all_pass() {
        let results = vec![
            dummy_result(CoreStatus::Pass),
            dummy_result(CoreStatus::Pass),
        ];
        let report = ConformanceReport::new("SEP-41", "valid", &results);
        assert_eq!(report.status, Status::Pass);
        assert_eq!(report.summary.passed, 2);
    }

    #[test]
    fn test_overall_status_any_fail_no_error() {
        let results = vec![
            dummy_result(CoreStatus::Pass),
            dummy_result(CoreStatus::Fail),
        ];
        let report = ConformanceReport::new("SEP-41", "valid", &results);
        assert_eq!(report.status, Status::Fail);
    }

    #[test]
    fn test_overall_status_any_error() {
        let results = vec![
            dummy_result(CoreStatus::Pass),
            dummy_result(CoreStatus::Fail),
            dummy_result(CoreStatus::Error),
        ];
        let report = ConformanceReport::new("SEP-41", "valid", &results);
        assert_eq!(report.status, Status::Error);
    }

    #[test]
    fn test_overall_status_all_skipped() {
        let results = vec![dummy_result(CoreStatus::Skipped)];
        let report = ConformanceReport::new("SEP-41", "valid", &results);
        assert_eq!(report.status, Status::Skipped);

        let empty: Vec<TestResult> = vec![];
        let report_empty = ConformanceReport::new("SEP-41", "valid", &empty);
        assert_eq!(report_empty.status, Status::Skipped);
    }

    #[test]
    fn test_deterministic_ordering_and_serialization() {
        let results = vec![
            TestResult {
                test_id: "ID-1",
                description: "Test 1",
                status: CoreStatus::Pass,
                expected_behavior: "A",
                observed_behavior: "B",
            },
            TestResult {
                test_id: "ID-2",
                description: "Test 2",
                status: CoreStatus::Fail,
                expected_behavior: "C",
                observed_behavior: "D",
            },
        ];

        let report = ConformanceReport::new("PROFILE", "FIXTURE", &results);

        // Assert deterministic ordering
        assert_eq!(report.results[0].test_id, "ID-1");
        assert_eq!(report.results[1].test_id, "ID-2");

        let json = report.to_json().unwrap();

        // Validate round-trip structure
        let deserialized: ConformanceReport = serde_json::from_str(&json).unwrap();

        assert_eq!(report, deserialized);
    }

    #[test]
    fn test_negative_fixture_compatibility() {
        // In a negative fixture, we EXPECT a test to fail.
        // But the report should STILL show it as a FAIL natively.
        // It is the testing harness's job to check `assert_eq!(report.status, Status::Fail)`.
        let results = vec![dummy_result(CoreStatus::Fail)];
        let report = ConformanceReport::new("SEP-41", "invalid-fixture", &results);

        // Ensure it doesn't get converted to PASS just because we "expected" it.
        // Report status MUST remain FAIL.
        assert_eq!(report.status, Status::Fail);
        assert_eq!(report.results[0].status, Status::Fail);
    }
}

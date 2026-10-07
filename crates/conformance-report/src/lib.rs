use conformance_core::result::{TestResult, Status};

pub struct Reporter;

impl Reporter {
    pub fn print_human_readable(results: &[TestResult]) {
        let mut passed = 0;
        let mut failed = 0;
        let mut errors = 0;

        for r in results {
            let symbol = match r.status {
                Status::Pass => { passed += 1; "PASS " }
                Status::Fail => { failed += 1; "FAIL " }
                Status::Error => { errors += 1; "ERROR" }
                Status::Skipped => "SKIP "
            };
            println!("{} {} {}", symbol, r.test_id, r.description);
        }

        println!("--------------------------------");
        println!("{} passed, {} failed, {} errors", passed, failed, errors);
        if failed > 0 || errors > 0 {
            println!("STATUS: NON-CONFORMANT");
        } else {
            println!("STATUS: CONFORMANT");
        }
    }
}

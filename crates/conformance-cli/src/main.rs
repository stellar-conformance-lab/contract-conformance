use clap::{Parser, Subcommand};
use conformance_core::engine::ConformanceEngine;
use conformance_report::{ConformanceReport, Status};
use conformance_sep41::*;
use fixture_valid::ValidSep41Fixture;
use std::process::exit;

#[derive(Parser)]
#[command(name = "stellar-conform", version)]
#[command(about = "Stellar Contract Conformance CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run conformance tests
    Test {
        /// The conformance profile to test
        #[arg(long)]
        profile: String,
    },
}

fn run_sep41() -> Vec<conformance_core::result::TestResult> {
    vec![
        ConformanceEngine::run_isolated_scenario(&MetaNameScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(&MetaSymbolScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(&MetaDecimalsScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(&BalInitialScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(&BalZeroScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(&TransferSuccessScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(&TransferEventScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(
            &TransferInsufficientBalanceScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &TransferAuthorizationScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &TransferNegativeAmountScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(&AllowanceQueryScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(&AllowanceApproveScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(
            &AllowanceTransferFromScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &AllowanceExpirationScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &AllowanceInsufficientScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &AllowanceUnauthorizedApproveScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &AllowanceUnauthorizedTransferFromScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(&AllowanceEventScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(
            &AllowanceZeroRevocationScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &AllowanceOverwriteScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(&BurnSuccessScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(
            &BurnAuthorizationScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(&BurnFromSuccessScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(
            &BurnFromAuthorizationScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &BurnInsufficientBalanceScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(
            &BurnFromInsufficientAllowanceScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(&BurnEventScenario, ValidSep41Fixture::new),
        ConformanceEngine::run_isolated_scenario(
            &AllowanceTransferFromEventScenario,
            ValidSep41Fixture::new,
        ),
        ConformanceEngine::run_isolated_scenario(&BurnFromEventScenario, ValidSep41Fixture::new),
    ]
}

pub fn execute_cli(cli: Cli) -> Result<i32, String> {
    match &cli.command {
        Commands::Test { profile } => {
            if profile.to_lowercase() != "sep-41" {
                return Err(format!("Error: unsupported profile '{}'", profile));
            }

            let results = run_sep41();
            let report = ConformanceReport::new("SEP-41", "valid", &results);

            println!("Profile: {}", report.profile);
            println!(
                "Status: {}",
                match report.status {
                    Status::Pass => "PASS",
                    Status::Fail => "FAIL",
                    Status::Error => "ERROR",
                    Status::Skipped => "SKIPPED",
                }
            );
            println!("Total: {}", report.summary.total);
            println!("Passed: {}", report.summary.passed);
            println!("Failed: {}", report.summary.failed);
            println!("Errors: {}", report.summary.errors);
            println!("Skipped: {}", report.summary.skipped);

            match report.status {
                Status::Pass => Ok(0),
                _ => Ok(1), // Use non-zero exit code for test failures
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match execute_cli(cli) {
        Ok(code) => exit(code),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_valid_profile() {
        let cli = Cli::try_parse_from(["stellar-conform", "test", "--profile", "sep-41"]).unwrap();
        let code = execute_cli(cli).expect("Should succeed");
        assert_eq!(code, 0, "All valid fixture scenarios should pass");
    }

    #[test]
    fn test_unsupported_profile() {
        let cli = Cli::try_parse_from(["stellar-conform", "test", "--profile", "unknown"]).unwrap();
        let err = execute_cli(cli).unwrap_err();
        assert!(err.contains("unsupported profile"));
    }

    #[test]
    fn test_missing_profile_arg() {
        let err = Cli::try_parse_from(["stellar-conform", "test"]).unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    }
}

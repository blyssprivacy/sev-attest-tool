//! Create and verify AMD SEV-SNP attestation reports.
//!
//! Builds off of the the "SEV-SNP Platform Attestation Using VirTEE/SEV" whitepaper.
use clap::*;

use sev_attest_tool::generate_attestation::*;
use sev_attest_tool::verify_attestation::*;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    GenerateAttestation {
        /// Data to attach to the attestation report.
        /// Must be a 64 byte hex string.
        data_to_attach: Option<String>,
    },
    VerifyAttestation {
        /// Path to the attestation report to verify.
        /// The report must be a JSON file.
        path: String,

        /// Flag that modifies the attestation report, causing verification to fail.
        #[clap(short, long)]
        fail_on_purpose: bool,

        /// Path to the "Versioned Chip Endorsement Key" (VCEK) to use for verification.
        /// If not provided, the VCEK will be requested from the AMD Key Distribution Service (KDS).
        vcek_path: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::GenerateAttestation { data_to_attach } => {
            let output = generate_attestation_report(data_to_attach);
            println!("{}", output);
        }
        Commands::VerifyAttestation {
            path,
            fail_on_purpose,
            vcek_path,
        } => {
            println!("Verifying attestation report...");
            verify_attestation_report_cli(&path, vcek_path.as_deref(), fail_on_purpose);
            println!("RESULT: PASS\nVerification successful!");
        }
    }
}

use anyhow::Result;
use clap::Subcommand;
use rumble_canvas_handoff::wrench_integration::{
    check_package_completeness, summarize_report, WrenchError,
};
use rumble_canvas_store::JsonFileStore;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum WrenchCommand {
    /// Run completeness checks on a package
    Check {
        #[arg(long)]
        store: PathBuf,
    },
}

pub fn handle_wrench(cmd: WrenchCommand) -> Result<()> {
    match cmd {
        WrenchCommand::Check { store } => {
            let file = JsonFileStore::new(&store);
            let store_data = file.load()?;

            if let Some(package) = store_data.packages.last() {
                eprintln!("Running wrench completeness checks...");
                match check_package_completeness(&store_data.workspace, package) {
                    Ok(report) => {
                        let (passed, messages) = summarize_report(&report);
                        for msg in messages {
                            eprintln!("{}", msg);
                        }
                        eprintln!(
                            "summary: {} error(s), {} warning(s), {} info(s)",
                            report.summary.errors, report.summary.warnings, report.summary.infos
                        );

                        if passed {
                            println!("✓ All wrench checks passed");
                            Ok(())
                        } else {
                            Err(anyhow::anyhow!("Wrench checks failed"))
                        }
                    }
                    // wrench-inspect not installed is a soft error for MVP
                    Err(WrenchError::NotFound) => {
                        eprintln!("warning: wrench-inspect not found in PATH; skipping checks");
                        println!("✓ Wrench checks skipped (wrench-inspect not installed)");
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            } else {
                Err(anyhow::anyhow!(
                    "No packages in store; run `package build` first"
                ))
            }
        }
    }
}

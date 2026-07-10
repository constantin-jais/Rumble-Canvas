use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use rumble_canvas_domain::sample_workspace;
use rumble_canvas_handoff::build_handoff;
use rumble_canvas_package::build_package;
use rumble_canvas_store::{handoff_id, CanvasStore, JsonFileStore, StoredReport};
use serde_json::Value;

mod commands;

#[derive(Debug, Parser)]
#[command(name = "rumble-canvas")]
#[command(
    about = "Rumble Canvas local package/handoff producer. Planning-only; never executes implementation work."
)]
struct Cli {
    #[command(subcommand)]
    command: TopCommand,
}

#[derive(Debug, Subcommand)]
enum TopCommand {
    Workspace {
        #[command(subcommand)]
        action: WorkspaceAction,
    },
    Package {
        #[command(subcommand)]
        action: PackageAction,
    },
    Handoff {
        #[command(subcommand)]
        action: HandoffAction,
    },
    Wrench {
        #[command(subcommand)]
        cmd: commands::wrench::WrenchCommand,
    },
}

#[derive(Debug, Subcommand)]
enum WorkspaceAction {
    Sample {
        #[arg(long)]
        store: PathBuf,
    },
    Show {
        #[arg(long)]
        store: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum PackageAction {
    Sample {
        #[arg(long)]
        out: PathBuf,
    },
    Build {
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum HandoffAction {
    Sample {
        #[arg(long)]
        out: PathBuf,
    },
    Build {
        #[arg(long)]
        store: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Validate {
        payload: Option<PathBuf>,
        #[arg(long)]
        store: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    Plan {
        payload: Option<PathBuf>,
        #[arg(long)]
        store: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        TopCommand::Workspace { action } => match action {
            WorkspaceAction::Sample { store } => write_sample_store(&store),
            WorkspaceAction::Show { store } => show_store(&store),
        },
        TopCommand::Package { action } => match action {
            PackageAction::Sample { out } => write_sample_package(&out),
            PackageAction::Build { store, out } => build_package_from_store(&store, out.as_deref()),
        },
        TopCommand::Handoff { action } => match action {
            HandoffAction::Sample { out } => write_sample_handoff(&out),
            HandoffAction::Build { store, out } => build_handoff_from_store(&store, out.as_deref()),
            HandoffAction::Validate {
                payload,
                store,
                json,
            } => run_and_maybe_store(&["handoff", "validate"], payload, store, json),
            HandoffAction::Plan {
                payload,
                store,
                json,
            } => run_and_maybe_store(&["handoff", "plan"], payload, store, json),
        },
        TopCommand::Wrench { cmd } => commands::handle_wrench(cmd),
    }
}

fn write_sample_store(path: &Path) -> Result<()> {
    let file = JsonFileStore::new(path);
    let store = CanvasStore::new(sample_workspace());
    file.save(&store)?;
    println!("wrote workspace store {}", path.display());
    Ok(())
}

fn show_store(path: &Path) -> Result<()> {
    let store = JsonFileStore::new(path).load()?;
    println!(
        "workspace: {} ({})",
        store.workspace.name, store.workspace.id
    );
    println!("status: {}", store.workspace.status);
    println!("packages: {}", store.packages.len());
    println!("handoffs: {}", store.handoffs.len());
    println!("validation reports: {}", store.validation_reports.len());
    println!("dry-run plans: {}", store.dry_run_plans.len());
    Ok(())
}

fn build_package_from_store(path: &Path, out: Option<&Path>) -> Result<()> {
    let file = JsonFileStore::new(path);
    let mut store = file.load()?;
    let package = build_package(&store.workspace)?;
    if let Some(out) = out {
        write_json(out, &package)?;
    }
    let id = package.package_id.clone();
    let hash = package.package_hash.clone();
    store.upsert_package(package);
    file.save(&store)?;
    println!("built package {id} {hash}");
    Ok(())
}

fn build_handoff_from_store(path: &Path, out: Option<&Path>) -> Result<()> {
    let file = JsonFileStore::new(path);
    let mut store = file.load()?;
    let package = if let Some(package) = store.packages.last().cloned() {
        package
    } else {
        let package = build_package(&store.workspace)?;
        store.upsert_package(package.clone());
        package
    };
    let handoff = build_handoff(&store.workspace, &package)?;
    if let Some(out) = out {
        write_json(out, &handoff)?;
    }
    let id = handoff_id(&handoff).unwrap_or_else(|| "<missing>".to_string());
    store.upsert_handoff(handoff);
    file.save(&store)?;
    println!("built handoff {id}");
    Ok(())
}

fn write_sample_package(out: &Path) -> Result<()> {
    let workspace = sample_workspace();
    let package = build_package(&workspace)?;
    write_json(out, &package)?;
    println!("wrote package {}", out.display());
    Ok(())
}

fn write_sample_handoff(out: &Path) -> Result<()> {
    let workspace = sample_workspace();
    let package = build_package(&workspace)?;
    let handoff = build_handoff(&workspace, &package)?;
    write_json(out, &handoff)?;
    println!("wrote handoff {}", out.display());
    Ok(())
}

fn run_and_maybe_store(
    prefix: &[&str],
    payload: Option<PathBuf>,
    store: Option<PathBuf>,
    json: bool,
) -> Result<()> {
    let effective_json = json || store.is_some();
    let temp_dir;
    let payload_path = match (payload, store.as_deref()) {
        (Some(payload), _) => payload,
        (None, Some(store_path)) => {
            let store_data = JsonFileStore::new(store_path).load()?;
            let handoff = store_data
                .latest_handoff()
                .context("store has no handoff; run `handoff build` first")?;
            temp_dir = std::env::temp_dir();
            let path = temp_dir.join("rumble-canvas-latest-handoff.json");
            write_json(&path, handoff)?;
            path
        }
        (None, None) => bail!("provide a payload path or --store"),
    };

    let output = run_cosmatic_capture(prefix, &payload_path, effective_json)?;
    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
    if !output.status.success() {
        bail!(
            "cosmatic {:?} failed with status {}",
            cosmatic_args(prefix, &payload_path, json),
            output.status
        );
    }

    if let Some(store_path) = store {
        let file = JsonFileStore::new(&store_path);
        let mut store_data = file.load()?;
        let payload_json: Value = serde_json::from_slice(&output.stdout)
            .context("cosmatic --json stdout must be JSON to store reports")?;
        let report = StoredReport {
            id: format!(
                "report:{}:{}",
                prefix[1],
                store_data.validation_reports.len() + store_data.dry_run_plans.len() + 1
            ),
            handoff_id: store_data
                .latest_handoff()
                .and_then(handoff_id)
                .unwrap_or_else(|| "<external-payload>".to_string()),
            created_at: rumble_canvas_domain::SAMPLE_TS.to_string(),
            source_command: cosmatic_args(prefix, &payload_path, effective_json),
            payload: payload_json,
        };
        if prefix == ["handoff", "validate"] {
            store_data.push_validation_report(report);
        } else {
            store_data.push_dry_run_plan(report);
        }
        file.save(&store_data)?;
    }
    Ok(())
}

fn write_json<T: serde::Serialize>(out: &Path, value: &T) -> Result<()> {
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(value)?;
    std::fs::write(out, format!("{json}\n")).with_context(|| format!("write {}", out.display()))
}

fn run_cosmatic_capture(
    prefix: &[&str],
    payload: &Path,
    json: bool,
) -> Result<std::process::Output> {
    Command::new("cosmatic")
        .args(cosmatic_args(prefix, payload, json))
        .output()
        .context("failed to spawn `cosmatic`; install or put cosmatic on PATH")
}

fn cosmatic_args(prefix: &[&str], payload: &Path, json: bool) -> Vec<String> {
    let mut args: Vec<String> = prefix.iter().map(|s| (*s).to_string()).collect();
    args.push(payload.display().to_string());
    if prefix == ["handoff", "plan"] {
        args.push("--dry-run".to_string());
    }
    if json {
        args.push("--json".to_string());
    }
    args
}

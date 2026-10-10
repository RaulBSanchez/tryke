use anyhow::Result;

use super::load_project;
use crate::ExitStatus;
use crate::cli::{CleanArgs, GlobalArgs};

pub(crate) fn run_clean_command(args: CleanArgs, global: &GlobalArgs) -> Result<ExitStatus> {
    let project = load_project(args.root.as_deref(), global, args.project_options(global))?;
    let report = tryke_discovery::clean_project_cache(&project)?;

    if report.removed_entries == 0 {
        println!(
            "No tryke discovery cache found at {}",
            report.cache_dir.display()
        );
    } else {
        println!(
            "Cleaned tryke discovery cache at {}",
            report.cache_dir.display()
        );
    }
    Ok(ExitStatus::Success)
}

use std::env;

use anyhow::Result;
use tryke_config::{Project, ProjectMetadata};

use crate::ExitStatus;
use crate::cli::{CleanArgs, GlobalArgs};

pub(crate) fn run_clean_command(args: CleanArgs, global: &GlobalArgs) -> Result<ExitStatus> {
    let cwd = env::current_dir()?;
    let mut metadata = ProjectMetadata::new(args.root.as_deref().unwrap_or(&cwd));
    if let Some(config_file) = &global.config_file {
        metadata.apply_configuration_file_from_path(config_file);
    } else {
        metadata.apply_configuration_file();
    }
    metadata.apply_cli_args(args.project_options(global));
    let project = Project::from_metadata(metadata);
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

mod clean;
mod graph;
mod server;
mod test;

use std::{env, path::Path};

use anyhow::Result;
use tryke_config::{Project, ProjectMetadata, TrykeOptions};

use crate::cli::GlobalArgs;

pub(crate) use clean::run_clean_command;
pub(crate) use graph::run_graph_command;
pub(crate) use server::run_server_command;
pub(crate) use test::run_test_command;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandOrigin {
    Bare,
    Explicit,
}

fn load_project(
    root: Option<&Path>,
    global: &GlobalArgs,
    options: TrykeOptions,
) -> Result<Project> {
    let cwd = env::current_dir()?;
    let mut metadata = ProjectMetadata::new(root.unwrap_or(&cwd));
    if let Some(config_file) = &global.config_file {
        metadata.apply_configuration_file_from_path(config_file);
    } else {
        metadata.apply_configuration_file();
    }
    metadata.apply_cli_args(options);
    Ok(Project::from_metadata(metadata))
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use tryke_testing::TestProject;

    use super::*;
    use crate::cli::Cli;

    #[test]
    fn discovered_config_respects_cli_includes_and_cache_dir() -> Result<()> {
        let fixture = TestProject::with_files([(
            "pyproject.toml",
            "[tool.tryke]\nexclude = [\"generated\", \"vendor\"]\ncache_dir = \".file-cache\"\n",
        )])?;
        let global = Cli::try_parse_from(["tryke"])?.global;

        let project = load_project(
            Some(fixture.root()),
            &global,
            TrykeOptions {
                include: Some(vec!["generated".into()]),
                cache_dir: Some(".cli-cache".into()),
                ..TrykeOptions::default()
            },
        )?;

        assert_eq!(project.discovery().exclude, vec!["vendor"]);
        assert_eq!(
            project.cache_dir(),
            Some(fixture.root().join(".cli-cache").as_path())
        );
        Ok(())
    }

    #[test]
    fn explicit_config_replaces_discovery_and_cli_excludes_win() -> Result<()> {
        let fixture = TestProject::with_files([
            (
                "pyproject.toml",
                "[tool.tryke]\nexclude = [\"auto\"]\ncache_dir = \".auto-cache\"\n",
            ),
            (
                "ci/tryke.toml",
                "exclude = [\"explicit\"]\ncache_dir = \".explicit-cache\"\n",
            ),
        ])?;
        let mut global = Cli::try_parse_from(["tryke"])?.global;
        global.config_file = Some(fixture.root().join("ci/tryke.toml"));

        let project = load_project(
            Some(fixture.root()),
            &global,
            TrykeOptions {
                exclude: Some(vec!["cli".into()]),
                ..TrykeOptions::default()
            },
        )?;

        assert_eq!(project.discovery().exclude, vec!["cli"]);
        assert_eq!(
            project.cache_dir(),
            Some(fixture.root().join("ci/.explicit-cache").as_path())
        );
        Ok(())
    }
}

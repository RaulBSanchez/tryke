use std::env;

use anyhow::Result;
use tryke_discovery::Discoverer;
use tryke_runner::{WorkerPool, WorkerPoolOptions};

use super::load_project;
use crate::ExitStatus;
use crate::cli::{GlobalArgs, ServerArgs};

pub(crate) fn run_server_command(args: ServerArgs, global: &GlobalArgs) -> Result<ExitStatus> {
    let cli_filter = global.verbose.log_level_filter();
    let tryke_log = env::var("TRYKE_LOG").ok();
    let log_level = tryke_config::worker_log_level(tryke_log.as_deref(), cli_filter);
    let project = load_project(args.root.as_deref(), global, args.project_options(global))?;
    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async move {
        let worker_pool = WorkerPool::spawn(
            &project,
            WorkerPoolOptions {
                size: args.workers,
                python_path: None,
                log_level,
                warm: false,
            },
        )
        .await;

        let discoverer = Discoverer::new(&project);

        tryke_server::Server::new(worker_pool, discoverer)
            .serve()
            .await
    })?;
    Ok(ExitStatus::Success)
}

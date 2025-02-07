use anyhow::{Context, Result};
use options::Format;
use regex::RegexSet;
use std::collections::HashSet;
use std::{fs, io::BufRead};
use structopt::StructOpt;

mod options;

use crate::options::{Config, FreqOptions};

use freq::{ClientBuilder, Input, Stats};

/// A C-like enum that can be cast to `i32` and used as process exit code.
enum ExitCode {
    Success = 0,
    // NOTE: exit code 1 is used for any `Result::Err` bubbled up to `main()`
    // using the `?` operator. For now, 1 acts as a catch-all for all errors
    // (including config errors), until we find a way to structure the error
    // code handling better.
    #[allow(unused)]
    UnexpectedFailure = 1,
}

fn main() -> Result<()> {
    // std::process::exit doesn't guarantee that all destructors will be ran,
    // therefore we wrap "main" code in another function to guarantee that.
    // See: https://doc.rust-lang.org/stable/std/process/fn.exit.html
    // Also see: https://www.youtube.com/watch?v=zQC8T71Y8e4
    let exit_code = run_main()?;
    std::process::exit(exit_code);
}

fn run_main() -> Result<i32> {
    let mut opts = FreqOptions::from_args();

    // Load a potentially existing config file and merge it into the config from the CLI
    if let Some(c) = Config::load_from_file(&opts.config_file)? {
        opts.config.merge(c)
    }
    let inputs = opts.inputs();
    let cfg = opts.config;

    let runtime = match cfg.threads {
        Some(threads) => {
            // We define our own runtime instead of the `tokio::main` attribute
            // since we want to make the number of threads configurable
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(threads)
                .enable_all()
                .build()?
        }
        None => tokio::runtime::Runtime::new()?,
    };

    runtime.block_on(run(cfg, inputs))
}

fn fmt(stats: &Stats, format: &Format) -> Result<String> {
    Ok(match format {
        Format::String => stats.to_string(),
        Format::Json => serde_json::to_string_pretty(&stats)?,
    })
}

async fn run(cfg: Config, _inputs: Vec<Input>) -> Result<i32> {
    let exclude = RegexSet::new(cfg.exclude)?;

    let stopwords = if cfg.exclude_stopwords {
        stop_words::get(stop_words::LANGUAGE::English)
            .into_iter()
            .collect::<HashSet<String>>()
    } else {
        HashSet::default()
    };

    let mut client = ClientBuilder::default()
        .excludes(exclude)
        .stopwords(stopwords)
        .build()?;

    // TODO: Add support for file input
    // let files = collector::collect_files(&inputs, max_concurrency).await?;

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            client.update(line);
        } else {
            break;
        }
    }

    // TODO: Use tokio channel for analyzing multiple files concurrently
    // let (send_req, recv_req) = mpsc::channel(max_concurrency);
    // let (send_resp, mut recv_resp) = mpsc::channel(max_concurrency);
    // let sr = send_req.clone();
    // tokio::spawn(async move {
    //     for link in links {
    //         sr.send(link).await.unwrap();
    //     }
    // });
    // tokio::spawn(async move {
    //     // Start receiving requests
    //     let clients: Vec<_> = (0..max_concurrency).map(|_| client.clone()).collect();
    //     let mut clients = ClientPool::new(send_resp, recv_req, clients);
    //     clients.listen().await;
    // });

    let stats_formatted = fmt(&client.stats, &cfg.format)?;
    if let Some(output) = &cfg.output {
        fs::write(output, stats_formatted).context("Cannot write status output to file")?;
    } else {
        println!("\n{}", stats_formatted);
    }

    Ok(ExitCode::Success as i32)
}

use anyhow::{anyhow, Error, Result};
use freq::Input;
use serde::Deserialize;
use std::str::FromStr;
use std::{fs, io::ErrorKind, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, Deserialize)]
pub enum Format {
    String,
    Json,
}

impl FromStr for Format {
    type Err = Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format {
            "string" => Ok(Format::String),
            "json" => Ok(Format::Json),
            _ => Err(anyhow!("Could not parse format {}", format)),
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::String
    }
}

// Macro for merging configuration values
macro_rules! fold_in {
    ( $cli:ident , $toml:ident ; $( $key:ident : $default:expr; )* ) => {
        $(
            if $cli.$key == $default && $toml.$key != $default {
                $cli.$key = $toml.$key;
            }
        )*
    };
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "freq",
    about = "A word frequency counter.\n\nProject home page: https://github.com/mre/freq"
)]
pub(crate) struct FreqOptions {
    /// The input files
    /// These can be: files (e.g. `README.md`), glob patterns (e.g. `"~/git/*/README.md"`),
    /// or standard input (`-`).
    /// Prefix with `--` to separate inputs from options that allow multiple arguments.
    #[structopt(name = "inputs", default_value = ".")]
    raw_inputs: Vec<String>,

    /// Configuration file to use
    #[structopt(short, long = "config", default_value = "./freq.toml")]
    pub config_file: String,

    #[structopt(flatten)]
    pub config: Config,
}

impl FreqOptions {
    // This depends on config, which is why a method is required (we could
    // accept a `Vec<Input>` in `FreqOptions` and do the conversion there,
    // but we'd get no access to `glob_ignore_case`.
    /// Get parsed inputs from options.
    pub(crate) fn inputs(&self) -> Vec<Input> {
        self.raw_inputs
            .iter()
            .map(|s| Input::new(s, self.config.glob_ignore_case))
            .collect()
    }
}

#[derive(Debug, Deserialize, StructOpt)]
pub struct Config {
    /// Verbose program output
    #[structopt(short, long)]
    #[serde(default)]
    pub verbose: bool,

    /// Do not show interactive histogram output
    /// This is recommended for non-interactive shells (e.g. for continuos
    /// integration)
    #[structopt(long)]
    #[serde(default)]
    pub non_interactive: bool,

    /// Number of threads to utilize.
    /// Defaults to number of virtual cores available on the system
    #[structopt(short = "T", long)]
    #[serde(default)]
    pub threads: Option<usize>,

    /// Exclude words from analysis (supports regex)
    #[structopt(short, long)]
    #[serde(default)]
    pub exclude: Vec<String>,

    /// Skip missing input files (default is to error if they don't exist)
    #[structopt(long)]
    #[serde(default)]
    pub skip_missing: bool,

    /// Ignore case when expanding filesystem path glob inputs
    #[structopt(long)]
    #[serde(default)]
    pub glob_ignore_case: bool,

    /// Output file of status report
    #[structopt(short, long, parse(from_os_str))]
    #[serde(default)]
    pub output: Option<PathBuf>,

    /// Output file format of status report (json, string)
    #[structopt(short, long, default_value = "string")]
    #[serde(default)]
    pub format: Format,

    /// Exclude stopwords from analysis (using iso and nltk stopwords)
    #[structopt(long)]
    #[serde(default)]
    pub exclude_stopwords: bool,
}

impl Config {
    /// Load configuration from a file
    pub(crate) fn load_from_file(path: &str) -> Result<Option<Config>> {
        // Read configuration file
        let result = fs::read(path);

        // Ignore a file not found error
        let contents = match result {
            Ok(c) => c,
            Err(e) => {
                return match e.kind() {
                    ErrorKind::NotFound => Ok(None),
                    _ => Err(Error::from(e)),
                }
            }
        };

        Ok(Some(toml::from_slice(&contents)?))
    }

    /// Merge the configuration from TOML into the CLI configuration
    pub(crate) fn merge(&mut self, toml: Config) {
        fold_in! {
            // Destination and source configs
            self, toml;

            // Keys with defaults to assign
            verbose: false;
            non_interactive: false;
            threads: None;
            exclude: Vec::<String>::new();
            skip_missing: false;
            glob_ignore_case: false;
            output: None;
        }
    }
}

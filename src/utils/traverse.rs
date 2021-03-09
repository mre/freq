use anyhow::{anyhow, Context, Result};
use glob::glob_with;
use serde::Serialize;
use shellexpand::tilde;
use std::path::PathBuf;
use std::{collections::HashSet, fmt::Display};
use std::{io::Read, path::Path};
use tokio::io::{stdin, AsyncReadExt};

const STDIN: &str = "-";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Input {
    FsGlob { pattern: String, ignore_case: bool },
    FsPath(PathBuf),
    Stdin,
    String(String),
}

impl Serialize for Input {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::FsGlob {
                pattern,
                ignore_case: _,
            } => {
                write!(f, "{}", pattern)
            }
            Input::FsPath(path) => {
                write!(f, "{}", path.to_str().unwrap_or_default())
            }
            Input::Stdin => {
                write!(f, "stdin")
            }
            Input::String(_) => {
                write!(f, "raw input string")
            }
        }
    }
}

impl Input {
    pub fn new(value: &str, glob_ignore_case: bool) -> Self {
        if value == STDIN {
            Self::Stdin
        } else {
            let is_glob = glob::Pattern::escape(value) != value;

            if is_glob {
                Self::FsGlob {
                    pattern: value.to_owned(),
                    ignore_case: glob_ignore_case,
                }
            } else {
                Self::FsPath(value.into())
            }
        }
    }

    // pub async fn get_contents(
    //     &self,
    //     file_type_hint: Option<FileType>,
    //     skip_missing: bool,
    // ) -> Result<Vec<InputContent>> {
    //     use Input::*;

    //     match self {
    //         // TODO: should skip_missing also affect URLs?
    //         RemoteUrl(url) => Ok(vec![Self::url_contents(url).await?]),
    //         FsGlob {
    //             pattern,
    //             ignore_case,
    //         } => Ok(Self::glob_contents(pattern, *ignore_case).await?),
    //         FsPath(path) => {
    //             let content = Self::path_content(&path).await.with_context(|| {
    //                 format!(
    //                     "Failed to read file: `{}`",
    //                     path.to_str().unwrap_or("<MALFORMED PATH>")
    //                 )
    //             });
    //             match content {
    //                 Ok(input_content) => Ok(vec![input_content]),
    //                 Err(_) if skip_missing => Ok(vec![]),
    //                 Err(arg) => Err(anyhow!(arg)),
    //             }
    //         }
    //         Stdin => Ok(vec![Self::stdin_content(file_type_hint).await?]),
    //         String(s) => Ok(vec![Self::string_content(s, file_type_hint)]),
    //     }
    // }
}

// TODO: Make that a stream of files?
// async fn glob_contents(path_glob: &str, ignore_case: bool) -> Result<Vec<Input>> {
//     let mut contents = vec![];
//     let glob_expanded = tilde(&path_glob);
//     let mut match_opts = glob::MatchOptions::new();

//     match_opts.case_sensitive = !ignore_case;

//     for entry in glob_with(&glob_expanded, match_opts)? {
//         match entry {
//             Ok(path) => {
//                 contents.push(content);
//             }
//             Err(e) => println!("{:?}", e),
//         }
//     }

//     Ok(contents)
// }

/// Return readers for all matches from a slice of inputs
pub async fn files_stream<T: Read>(inputs: &[Input]) -> Result<HashSet<T>> {
    todo!();

    // // extract input contents
    // for input in inputs.iter().cloned() {
    //     let sender = contents_tx.clone();

    //     tokio::spawn(async move {
    //         let contents = input.get_contents(None, skip_missing_inputs).await;
    //         sender.send(contents).await
    //     });
    // }

    // // receiver will get None once all tasks are done
    // drop(contents_tx);

    // // extract links from input contents
    // let mut extract_link_handles = vec![];

    // while let Some(result) = contents_rx.recv().await {
    //     for input_content in result? {
    //         let base_url = base_url.clone();
    //         let handle =
    //             tokio::task::spawn_blocking(move || extract_links(&input_content, base_url));
    //         extract_link_handles.push(handle);
    //     }
    // }

    // // Note: we could dispatch links to be checked as soon as we get them,
    // //       instead of building a HashSet with all links.
    // //       This optimization would speed up cases where there's
    // //       a lot of inputs and/or the inputs are large (e.g. big files).
    // let mut collected_links: HashSet<Request> = HashSet::new();

    // for handle in extract_link_handles {
    //     let links = handle.await?;
    //     collected_links.extend(links);
    // }

    // Ok(collected_links)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_collect_links() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("f");
        let file_glob_1_path = dir.path().join("glob-1");
        let file_glob_2_path = dir.path().join("glob-2");

        let mut file = File::create(&file_path)?;
        let mut file_glob_1 = File::create(file_glob_1_path)?;
        let mut file_glob_2 = File::create(file_glob_2_path)?;

        let inputs = vec![
            // Input::String(TEST_STRING.to_string()),
            Input::FsPath(file_path),
            Input::FsGlob {
                pattern: dir.path().join("glob*").to_str().unwrap().to_string(),
                ignore_case: true,
            },
        ];

        // TODO
        // let found_files: HashSet<Box<Read>> = files_stream(&inputs).await?.collect();
        // // let expected_files = vec![file_path, file_glob_1_path, file_glob_2_path, file, file_glob_1, file_glob_2, expected_files];
        // assert_eq!(found_files, expected_files);

        Ok(())
    }
}

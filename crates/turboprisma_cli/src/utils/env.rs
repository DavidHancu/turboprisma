use std::path::PathBuf;

use relative_path::RelativePathBuf;

pub fn obtain_env_path(
    cwd: &PathBuf,
    schema_argument: &Option<String>,
) -> Result<Option<RelativePathBuf>, ()> {
    match schema_argument {
        Some(value) => {
            let base = RelativePathBuf::new().join_normalized(value).to_path(cwd);
            let checked = match base.parent() {
                Some(value) => Ok(value),
                None => Err(()),
            };

            match checked {
                Ok(value) => {
                    let checked = RelativePathBuf::new()
                        .join_normalized("./.env");
                    match checked.to_path(value).exists() {
                        true => Ok(Some(checked)),
                        false => {
                            let checked = RelativePathBuf::new().join_normalized("./.env");
                            match checked.to_path(cwd).exists() {
                                true => Ok(Some(checked)),
                                false => {
                                    let checked =
                                        RelativePathBuf::new().join_normalized("./prisma/.env");
                                    match checked.to_path(cwd).exists() {
                                        true => Ok(Some(checked)),
                                        false => Ok(None),
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => Err(()),
            }
        }
        None => {
            let checked = RelativePathBuf::new().join_normalized("./.env");
            match checked.to_path(cwd).exists() {
                true => Ok(Some(checked)),
                false => {
                    let checked = RelativePathBuf::new().join_normalized("./prisma/.env");
                    match checked.to_path(cwd).exists() {
                        true => Ok(Some(checked)),
                        false => Ok(None),
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn obtain_env_paths(
    cwd: &PathBuf,
    schema_argument: &Option<Vec<String>>,
) -> Vec<Result<std::option::Option<RelativePathBuf>, ()>> {
    match schema_argument {
        Some(values) => values
            .iter()
            .map(|path| obtain_env_path(cwd, &Some(path.to_string())))
            .collect(),
        None => vec![obtain_env_path(cwd, &None)],
    }
}

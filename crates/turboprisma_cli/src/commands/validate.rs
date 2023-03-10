use colored::Colorize;
use serde::Deserialize;
use std::{collections::HashMap, env, path::PathBuf, process::exit};
use tiny_gradient::*;

use crate::{
    env_parser,
    utils::{env::obtain_env_path, schema::obtain_schema_paths_with_argument},
};

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

#[derive(Deserialize)]
struct ValidateError {
    #[allow(dead_code)]
    error_code: String,
    message: String,
}

pub fn run(help: bool, options: HashMap<String, String>) {
    match help {
        true => {
            println!(
                "\n{} \
            \n{}{}{} \
            \n\n{} \
            \n\n  {} {} \
            \n\n{} \
            \n\n  {}     {} \
            \n  {}     {} \
            \n\n{} \
            \n\n  {} \
            \n  {} {} \
            \n\n  {} \
            \n  {} {} \
            \n\n  {} \
            \n  {} {} \
            \n",
                "Validate a Turboprisma schema.".dimmed(),
                "Read more about this command at ".dimmed(),
                "https://turboprisma.js.org/docs/general-commands/validate"
                    .gradient(Gradient::Passion),
                ".".dimmed(),
                "Usage".gradient(Gradient::Passion),
                "$".dimmed(),
                "turboprisma validate [options]",
                "Options".gradient(Gradient::Passion),
                "-h, --help".dimmed(),
                "Display this help message",
                "  --schema".dimmed(),
                "Custom path to your Turboprisma schema",
                "Examples".gradient(Gradient::Passion),
                "With an existing Turboprisma schema".dimmed(),
                "$".dimmed(),
                "turboprisma validate",
                "Or specify a Turboprisma schema path".dimmed(),
                "$".dimmed(),
                "turboprisma validate --schema \"./schema.prisma\"",
                "Or specify multiple Turboprisma schemas".dimmed(),
                "$".dimmed(),
                "turboprisma validate --schema \"./prisma/schema.prisma,./prisma/another.prisma\"",
            )
        }

        false => {
            let cwd = match get_current_working_dir() {
                Ok(value) => value,
                Err(_) => {
                    println!(
                        "{} {}",
                        " ERROR ".on_red().white(),
                        "Couldn't get access to the current working directory.".red()
                    );
                    exit(1);
                }
            };

            let schema_arg: Option<Vec<&str>> = match options.get("--schema") {
                Some(value) => Some(value.split(",").collect()),
                None => None,
            };

            let paths = obtain_schema_paths_with_argument(&schema_arg);

            for path in paths {
                if !path.schema_path.to_path(&cwd).exists() {
                    println!(
                        "{} {}{}{}",
                        " ERROR ".on_red().white(),
                        "Schema file not found. (at ".red(),
                        path.schema_path.normalize().to_string().red().bold(),
                        ").".red()
                    );
                    exit(1);
                }

                println!(
                    "{}{}{}",
                    "Loaded schema from ".dimmed(),
                    path.schema_path.normalize().to_string().dimmed(),
                    ".".dimmed()
                );

                let schema = match std::fs::read_to_string(path.schema_path.to_path(&cwd)) {
                    Ok(value) => value,
                    Err(_) => {
                        println!(
                            "{} {}",
                            " ERROR ".on_red().white(),
                            "Encountered error while reading schema file.".red()
                        );
                        exit(1);
                    }
                };

                if let Err(error) = turboprisma_fmt::validate_schema(&schema) {
                    let data: ValidateError = match serde_json::from_str(&error) {
                        Ok(params) => params,
                        Err(serde_err) => {
                            panic!("Failed to deserialize ValidateError: {serde_err}");
                        }
                    };
                    println!("{}", data.message);
                    exit(1);
                }

                let env_path = obtain_env_path(&cwd, &path.schema_argument);

                match env_path {
                    Ok(env_file) => {
                        match env_file {
                            Some(value) => {
                                let env_content = std::fs::read_to_string(value.to_path(&cwd));
                                match env_content {
                                    Ok(content) => {
                                        let env_parsed = env_parser::parse_env(&content);
                                        match env_parsed {
                                            Ok(parsed) => {
                                                let config = json!({
                                                    "prismaSchema": &schema,
                                                    "datasourceOverrides": {},
                                                    "ignoreEnvVarErrors": false,
                                                    "env": parsed,
                                                }).to_string();

                                                
                                                if let Err(error) = turboprisma_fmt::get_config(config) {
                                                    let data: ValidateError = match serde_json::from_str(&error) {
                                                        Ok(params) => params,
                                                        Err(serde_err) => {
                                                            panic!("Failed to deserialize ValidateError: {serde_err}");
                                                        }
                                                    };
                                                    println!("{}", data.message);
                                                    exit(1);
                                                }
                                                
                                                println!("{} {}{}{}\n", " SUCCESS ".on_green().white(), "Schema ".green(), path.schema_path.normalize().to_string().green().bold(), " is valid.".green());
                                            }
                                            Err(_) => {
                                                println!(
                                                    "{} {}",
                                                    " ERROR ".on_red().white(),
                                                    "Invalid .env file.".red()
                                                );
                                                exit(1);
                                            }
                                        };
                                    }
                                    Err(_) => {
                                        println!(
                                            "{} {}",
                                            " ERROR ".on_red().white(),
                                            "Encountered error while attempting to read .env file."
                                                .red()
                                        );
                                        exit(1);
                                    }
                                };
                            }
                            None => {
                                println!(
                                    "{} {}",
                                    " ERROR ".on_red().white(),
                                    "Couldn't find an .env file to use.".red()
                                );
                                exit(1);
                            }
                        };
                    }
                    Err(_) => {
                        println!(
                            "{} {}",
                            " ERROR ".on_red().white(),
                            "Invalid schema path provided.".red()
                        );
                        exit(1);
                    }
                };
            }
        }
    }
}

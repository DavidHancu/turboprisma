use colored::Colorize;
use serde::Deserialize;
use std::{collections::HashMap, env, path::PathBuf, process::exit, time::Instant};
use tiny_gradient::*;

use crate::utils::schema::obtain_schema_paths;

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
                "Format a Turboprisma schema.".dimmed(),
                "Read more about this command at ".dimmed(), "https://turboprisma.js.org/docs/general-commands/format".gradient(Gradient::Passion), ".".dimmed(),

                "Usage".gradient(Gradient::Passion),
                "$".dimmed(), "turboprisma format [options]",

                "Options".gradient(Gradient::Passion),
                "-h, --help".dimmed(), "Display this help message",
                "  --schema".dimmed(), "Custom path to your Turboprisma schema",

                "Examples".gradient(Gradient::Passion),
                "With an existing Turboprisma schema".dimmed(),
                "$".dimmed(), "turboprisma format",
                "Or specify a Turboprisma schema path".dimmed(),
                "$".dimmed(), "turboprisma format --schema \"./schema.prisma\"",
                "Or specify multiple Turboprisma schemas".dimmed(),
                "$".dimmed(), "turboprisma format --schema \"./prisma/schema.prisma,./prisma/another.prisma\"",
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
                Some(value) => {
                    Some(value.split(",").collect())
                },
                None => None,
            };

            let paths = obtain_schema_paths(&schema_arg);

            for path in paths {
                if !path.to_path(&cwd).exists() {
                    println!("{} {}{}{}", " ERROR ".on_red().white(), "Schema file not found. (at ".red(), path.normalize().to_string().red().bold(), ").".red());
                    exit(1);
                }

                println!("{}{}{}", "Loaded schema from ".dimmed(), path.normalize().to_string().dimmed(), ".".dimmed());
                let before = Instant::now();

                let formatted = turboprisma_fmt::format(&match std::fs::read_to_string(path.to_path(&cwd)) {
                    Ok(value) => value,
                    Err(_) => {
                        println!("{} {}", " ERROR ".on_red().white(), "Encountered error while reading schema file.".red());
                        exit(1);
                    },
                }, "{\"textDocument\":{\"uri\":\"file:/dev/null\"},\"options\":{\"tabSize\":2,\"insertSpaces\":true}}");

                if let Err(error) = turboprisma_fmt::validate_schema(&formatted) {
                    let data: ValidateError = match serde_json::from_str(&error) {
                        Ok(params) => params,
                        Err(serde_err) => {
                            panic!("Failed to deserialize ValidateError: {serde_err}");
                        }
                    };
                    println!("{}", data.message);
                    exit(1);
                }

                match std::fs::write(path.to_path(&cwd), formatted) {
                    Ok(_) => {
                        println!("{} {}{}{}{}{}{}\n", " SUCCESS ".on_green().white(), "Successfully formatted ".green(), path.normalize().to_string().green().bold(), " in ".green(), before.elapsed().as_millis().to_string().green().bold(), "ms".green().bold(), ".".green());
                    },
                    Err(_) => {
                        println!("{} {}", " ERROR ".on_red().white(), "Encountered error while writing formatted schema file.".red());
                        exit(1);
                    },
                };
            }
        }
    }
}

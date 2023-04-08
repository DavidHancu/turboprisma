use colored::Colorize;
use std::{env, path::PathBuf};
use tiny_gradient::*;
use crate::utils::install::{get_package_manager, PackageManager};

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn run(help: bool, flags: Vec<String>) {
    match help {
        true => {
            println!(
                "\n{} \
            \n{}{}{} \
            \n\n{} \
            \n\n  {} {} \
            \n  {} {} \
            \n\n{} \
            \n\n  {}     {} \
            \n  {}     {} \
            \n",
                "Print current version of different Turboprisma components.".dimmed(),
                "Read more about this command at ".dimmed(), "https://turboprisma.js.org/docs/general-commands/version".gradient(Gradient::Passion), ".".dimmed(),
                "Usage".gradient(Gradient::Passion),
                "$".dimmed(),
                "turboprisma -v [options]",
                "$".dimmed(),
                "turboprisma version [options]",
                "Options".gradient(Gradient::Passion),
                "-h, --help".dimmed(),
                "Display this help message",
                "    --json".dimmed(),
                "Output JSON"
            )
        }
        false => match flags.contains(&"--json".to_string()) {
            true => {
                let cwd = match get_current_working_dir() {
                    Ok(value) => value,
                    Err(_) => {
                        println!(
                            "{} {}",
                            " ERROR ".on_red().white(),
                            "Couldn't get access to the current working directory.".red()
                        );
                        std::process::exit(1);
                    }
                };
                let obj = json!(
                    { 
                        "platform": { 
                            "os": env::consts::OS,
                            "arch": env::consts::ARCH,
                            "manager": match get_package_manager(&cwd) {
                                PackageManager::Npm => "npm",
                                PackageManager::Yarn => "yarn",
                                PackageManager::Pnpm => "pnpm",
                            }
                        }, 
                        "turboprisma": {
                            "cli": env!("CARGO_PKG_VERSION")
                        } 
                    });
                println!("{}", serde_json::to_string_pretty(&obj).unwrap());
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
                        std::process::exit(1);
                    }
                };
                println!(
                    "\n{} \
                    \n\n  {}  {} \
                    \n  {}  {} \
                    \n  {}  {} \
                    \n\n{} \
                    \n\n  {}  {} \
                    \n",
                    "Platform".gradient(Gradient::Passion),
                    "Operating System".dimmed(), env::consts::OS,
                    "    Architecture".dimmed(), env::consts::ARCH,
                    " Package Manager".dimmed(), match get_package_manager(&cwd) {
                        PackageManager::Npm => "npm",
                        PackageManager::Yarn => "yarn",
                        PackageManager::Pnpm => "pnpm",
                    },
                    "Turboprisma".gradient(Gradient::Passion),
                    "             CLI".dimmed(), env!("CARGO_PKG_VERSION")
                )
            }
        },
    }
}

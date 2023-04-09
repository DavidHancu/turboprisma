use std::env::{current_dir, args};
use std::process::{exit, Command};
use colored::Colorize;
use tiny_gradient::*;

use dunce::canonicalize as fs_canonicalize;
fn main() {
    let mut addendum = vec!["node_modules", "turboprisma"];
    #[cfg(windows)]
    addendum.push("turboprisma_cli.exe");
    #[cfg(not(windows))]
    addendum.push("turboprisma_cli");

    let cwd = match current_dir() {
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
    let turbo_internal_path = match fs_canonicalize(cwd) {
        Ok(value) => value,
        Err(_) => {
            println!(
                "{} {}",
                " ERROR ".on_red().white(),
                "Couldn't canonicalize the current working directory.".red()
            );
            exit(1);
        }
    };
    let mut turbo_path = turbo_internal_path.clone();

    loop {
        // append the addendum to the path
        let mut internal = turbo_path.clone();
        for item in addendum.iter() {
            internal.push(item);
        }

        if internal.exists() {
            let result = Command::new(internal)
                .current_dir(turbo_internal_path)
                .args(args().skip(1))
                .spawn();
            match result {
                Ok(mut child) => {
                    let ecode = child.wait();
                    match ecode {
                        Ok(ecode) => {
                            exit(ecode.code().unwrap_or(1));
                        }
                        Err(e) => {
                            println!(
                                "{} {}",
                                " ERROR ".on_red().white(),
                                format!("Couldn't execute the CLI: {}", e).red()
                            );
                            exit(1);
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "{} {}",
                        " ERROR ".on_red().white(),
                        format!("Couldn't execute the CLI: {}", e).red()
                    );
                    exit(1);
                }
            }
        }
        if !turbo_path.pop() {
            break;
        }
    }

    println!(
        "\n{} \
        \n\n  {} \
        \n  {} {}{} \
        \n",
        "Turboprisma Version Manager".gradient(Gradient::Passion),
        "The Turboprisma CLI needs to be installed in this project.".dimmed(),
        "Read more at".dimmed(), "https://turboprisma.js.org/go/tvm".gradient(Gradient::Passion), ".".dimmed(),
    );
    exit(1);
}
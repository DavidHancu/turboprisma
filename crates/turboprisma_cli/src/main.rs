mod args;
mod commands;
mod utils;
mod env_parser;

use args::{Args, ErrorType};
use colored::*;
use std::process::exit;
use tiny_gradient::*;

#[macro_use]
extern crate serde_json;

// https://github.com/vercel/turbo/blob/main/crates/turborepo-lib/src/shim.rs#L185
// --skip-infer
fn main() {
    let args = Args::try_parse();

    match args {
        Ok(mut parsed) => {
            if parsed.empty_commands() || (parsed.commands.first.is_none() && parsed.help) {
                println!(
                    "\n{} \
                    \n\n  {} \
                    \n  {} {}{} \
                    \n\n{} \
                    \n\n  {} {} \
                    \n\n{} \
                    \n\n  {}  {} \
                    \n  {}  {} \
                    \n  {}  {} \
                    \n  {}  {} \
                    \n\n{} \
                    \n\n  {} \
                    \n  {} {} \
                    \n\n  {} \
                    \n  {} {} \
                    \n\n  {} \
                    \n  {} {} \
                    \n",
                    "Turboprisma".gradient(Gradient::Passion),
                    "The agile runtime that allows you to use Prisma the way you want to.".dimmed(),
                    "Read more at".dimmed(), "https://turboprisma.js.org".gradient(Gradient::Passion), ".".dimmed(),

                    "Usage".gradient(Gradient::Passion),
                    "$".dimmed(), "turboprisma [command]".white(),

                    "Commands".gradient(Gradient::Passion),
                    " version".dimmed(), "Print useful information for debugging.".white(),
                    "    init".dimmed(), "Set up Turboprisma for your app.".white(),
                    "  format".dimmed(), "Format your Turboprisma schema.".white(),
                    "validate".dimmed(), "Validate your Turboprisma schema.".white(),

                    "Examples".gradient(Gradient::Passion),
                    "Setup a new Turboprisma project".dimmed(),
                    "$".dimmed(), "turboprisma init",
                    "Format your Turboprisma schema".dimmed(),
                    "$".dimmed(), "turboprisma format",
                    "Validate your Turboprisma schema".dimmed(),
                    "$".dimmed(), "turboprisma validate"
                );
                exit(0);
            }

            let first_command = parsed.commands.first.unwrap();

            match first_command.as_str() {
                "-v" | "version" => {
                    commands::version::run(parsed.help, parsed.flags);
                    exit(0);
                }
                
                "init" => {
                    commands::init::run(parsed.help, parsed.options);
                    exit(0);
                }

                "format" => {
                    commands::format::run(parsed.help, parsed.options);
                    exit(0);
                }

                "validate" => {
                    commands::validate::run(parsed.help, parsed.options);
                    exit(0);
                }

                _ => {
                    println!("{} {}{}{}", " ERROR ".on_red().white(), "Unknown command. (at ".red(), first_command.red().bold(), ").".red());
                    exit(1);
                }
            }
        }
        Err(error) => {
            match error.kind {
                ErrorType::InvalidFlag => {
                    println!("{} {}{}{}", " ERROR ".on_red().white(), "Invalid flag or option found in command. Flags must start with only one \"-\", and options must start with only two \"-\". (at ".red(), error.context.red().bold(), ").".red());
                }
            }
            exit(1);
        }
    }
}

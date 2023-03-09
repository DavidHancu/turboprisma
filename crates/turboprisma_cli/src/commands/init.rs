use colored::Colorize;
use std::{collections::HashMap, env, fs::create_dir, path::PathBuf, process::exit};
use tiny_gradient::*;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
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
            \n  {}     {} \
            \n  {}     {} \
            \n  {}     {} \
            \n\n{} \
            \n\n  {} \
            \n  {} {} \
            \n\n  {} \
            \n  {} {} \
            \n\n  {} \
            \n  {} {} \
            \n\n  {} \
            \n  {} {} \
            \n\n  {} \
            \n  {} {} \
            \n",
                "Set up Turboprisma for your app.".dimmed(),
                "Read more about this command at ".dimmed(), "https://turboprisma.js.org/docs/general-commands/init".gradient(Gradient::Passion), ".".dimmed(),

                "Usage".gradient(Gradient::Passion),
                "$".dimmed(), "turboprisma init [options]",

                "Options".gradient(Gradient::Passion),
                "           -h, --help".dimmed(), "Display this help message",
                "--datasource-provider".dimmed(), "Define the datasource provider",
                " --generator-provider".dimmed(), "Define the generator provider",
                "   --preview-features".dimmed(), "Define a list of preview features",
                "             --output".dimmed(), "Define the output for the generator",

                "Examples".gradient(Gradient::Passion),
                "Set up a new Turboprisma project with PostgreSQL (default)".dimmed(),
                "$".dimmed(), "turboprisma init",
                "Set up a new Turboprisma project and specify MySQL as the datasource provider to use".dimmed(),
                "$".dimmed(), "turboprisma init --datasource-provider mysql",
                "Set up a new Turboprisma project and use the prisma-client-go generator".dimmed(),
                "$".dimmed(), "turboprisma init --generator-provider prisma-client-go",
                "Set up a new Turboprisma project and enable clientExtensions and deno".dimmed(),
                "$".dimmed(), "turboprisma init --preview-features clientExtensions,deno",
                "Set up a new Turboprisma project and set the generator output to another folder".dimmed(),
                "$".dimmed(), "turboprisma init --output node_modules/another-location"
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

            let prisma_folder = &cwd.join("prisma");

            if prisma_folder.join("schema.prisma").exists() {
                println!("{} {}{}{}", " ERROR ".on_red().white(), "File ".red(), "prisma/schema.prisma".red().bold(), " already exists in your project.\nPlease try again in a project that is not yet using Turboprisma.".red());
                exit(1);
            }

            if prisma_folder.exists() {
                println!("{} {}{}{}", " ERROR ".on_red().white(), "A folder called ".red(), "prisma".red().bold(), " already exists in your project.\nPlease try again in a project that is not yet using Turboprisma.".red());
                exit(1);
            }

            let datasource_provider = match options.get("--datasource-provider") {
                Some(value) => {
                    if !vec![
                        "postgresql",
                        "mysql",
                        "sqlserver",
                        "sqlite",
                        "mongodb",
                        "cockroachdb",
                    ]
                    .contains(&value.to_lowercase().as_str())
                    {
                        println!("{} {}{}{}", " ERROR ".on_red().white(), "Invalid datasource provider. (at ".red(), value.to_lowercase().red().bold(), ")\nAvailable options are \"mysql\", \"postgresql\", \"sqlserver\", \"sqlite\", \"mongodb\" or \"cockroachdb\".".red());
                        exit(1);
                    }
                    value[..].as_ref()
                }
                None => "postgresql",
            };

            let preview_features = match options.get("--preview-features") {
                Some(value) => value.split(",").collect(),
                None => vec![],
            };

            let output = options.get("--output");

            let generator_provider = match options.get("--generator-provider") {
                Some(value) => value,
                None => "prisma-client-js",
            };

            match create_dir(prisma_folder) {
                Ok(_) => (),
                Err(_) => {
                    println!(
                        "{} {}",
                        " ERROR ".on_red().white(),
                        "Encountered error while creating folders.".red()
                    );
                    exit(1);
                }
            };

            let schema = format!("// This is your Turboprisma schema file,\n\
            // learn more about it in the docs: https://turboprisma.js.org/docs/schema-blocks/generator \n\
            generator client {{\n\
                {}provider = \"{}\"\n\
                {}\
                {}\
            }} \n\
            datasource db {{\n\
                {}provider = \"{}\"\n\
                {}url      = env(\"DATABASE_URL\")\n\
            }}", "  ", generator_provider, match output {
    Some(value) => format!("  output = \"{}\"\n", value),
    None => String::from(""),
}, match preview_features.len() {
    0 => String::from(""),
    _ => format!("  previewFeatures = [{}]\n", preview_features.iter().map(|v| format!("\"{}\"", v)).collect::<Vec<String>>().join(", ")),
}, "  ", datasource_provider, "  ");

            match std::fs::write(prisma_folder.join("schema.prisma"), schema) {
                Ok(_) => {
                    println!("{} {}{}{}", " SUCCESS ".on_green(), "Sucessfully written schema to ".green(), "prisma/schema.prisma".green().bold(), ".".green())
                },
                Err(_) => {
                    println!(
                        "{} {}",
                        " ERROR ".on_red().white(),
                        "Encountered error while writing schema.".red()
                    );
                    exit(1);
                }
            };
        }
    }
}

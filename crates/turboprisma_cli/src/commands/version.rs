use colored::Colorize;
use std::env;
use tiny_gradient::*;

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
                let obj = json!(
                    { 
                        "platform": { 
                            "os": env::consts::OS,
                            "arch": env::consts::ARCH
                        }, 
                        "turboprisma": {
                            "cli": env!("CARGO_PKG_VERSION")
                        } 
                    });
                println!("{}", serde_json::to_string_pretty(&obj).unwrap());
            }
            false => {
                println!(
                    "\n{} \
                    \n\n  {}  {} \
                    \n  {}  {} \
                    \n\n{} \
                    \n\n  {}  {} \
                    \n",
                    "Platform".gradient(Gradient::Passion),
                    "Operating System".dimmed(),
                    env::consts::OS,
                    "    Architecture".dimmed(),
                    env::consts::ARCH,
                    "Turboprisma".gradient(Gradient::Passion),
                    "             CLI".dimmed(),
                    env!("CARGO_PKG_VERSION")
                )
            }
        },
    }
}

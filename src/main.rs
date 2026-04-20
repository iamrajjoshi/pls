use anyhow::Result;
use clap::Parser;
use owo_colors::OwoColorize;
use pretty_please::cli::{Cli, CliAction};

fn main() {
    if let Err(error) = run() {
        eprintln!("{} {error:#}", "error:".red().bold());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.into_command_input() {
        CliAction::Init(shell) => {
            print!("{}", shell.init_script());
            Ok(())
        }
        CliAction::Completions(shell) => {
            shell.write_completions()?;
            Ok(())
        }
        CliAction::Exec(input) => pretty_please::exec::run(input),
    }
}

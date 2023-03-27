use std::io::Write;

use anyhow::{Ok, Result};
use clap_complete::{generate, Shell};

use super::{completions_arg::CompletionsArg, CliApp};

pub fn generate_completions(completions_arg: &CompletionsArg, buf: &mut dyn Write) -> Result<()> {
    let shell = choose_shell(&completions_arg);
    generate(shell, &mut CliApp::build_cli()?.command, CliApp::NAME, buf);
    Ok(())
}

fn choose_shell(completions_arg: &CompletionsArg) -> Shell {
    match completions_arg {
        CompletionsArg::Bash => Shell::Bash,
        CompletionsArg::Fish => Shell::Fish,
        CompletionsArg::Zsh => Shell::Zsh,
        CompletionsArg::PowerShell => Shell::PowerShell,
        CompletionsArg::Elvish => Shell::Elvish,
    }
}

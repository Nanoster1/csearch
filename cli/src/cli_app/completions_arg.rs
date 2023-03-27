use self::CompletionsArg::*;
use clap::{builder::PossibleValue, value_parser, Arg, ValueEnum};
use nameof::name_of;

#[derive(Clone)]
pub enum CompletionsArg {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}

impl CompletionsArg {
    pub const ID: &str = "Completions";
    pub const HELP: &str = "Generate completions for selected shell";
    pub const LONG: &str = "completions";
    pub const SHORT: char = 'c';
    pub const IS_REQUIRED: bool = false;

    pub fn describe_argument() -> Arg {
        Arg::new(CompletionsArg::ID)
            .help(CompletionsArg::HELP)
            .long(CompletionsArg::LONG)
            .short(CompletionsArg::SHORT)
            .required(CompletionsArg::IS_REQUIRED)
            .value_parser(value_parser!(CompletionsArg))
    }
}

impl ValueEnum for CompletionsArg {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            CompletionsArg::Bash,
            CompletionsArg::Zsh,
            CompletionsArg::PowerShell,
            CompletionsArg::Elvish,
            CompletionsArg::Fish,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            CompletionsArg::Bash => Some(PossibleValue::new(name_of!(Bash))),
            CompletionsArg::Zsh => Some(PossibleValue::new(name_of!(Zsh))),
            CompletionsArg::PowerShell => Some(PossibleValue::new(name_of!(PowerShell))),
            CompletionsArg::Elvish => Some(PossibleValue::new(name_of!(Elvish))),
            CompletionsArg::Fish => Some(PossibleValue::new(name_of!(Fish))),
        }
    }
}

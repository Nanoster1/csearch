use self::BrowserArg::*;
use app::models::Browser;
use clap::{builder::PossibleValue, value_parser, Arg, ValueEnum};
use nameof::name_of;

#[derive(Clone)]
pub enum BrowserArg {
    Default,
}

impl BrowserArg {
    pub const ID: &str = "Browser";
    pub const HELP: &str = "Select browser";
    pub const LONG: &str = "browser";
    pub const SHORT: char = 'b';
    pub const IS_REQUIRED: bool = false;
    pub const DEFAULT_VALUE: &str = name_of!(Default);

    pub fn describe_argument() -> Arg {
        Arg::new(Self::ID)
            .help(Self::HELP)
            .long(Self::LONG)
            .short(Self::SHORT)
            .required(Self::IS_REQUIRED)
            .value_parser(value_parser!(BrowserArg))
            .default_value(Self::DEFAULT_VALUE)
    }

    pub fn to_model(&self) -> Browser {
        match self {
            Default => Browser::Default,
        }
    }
}

impl ValueEnum for BrowserArg {
    fn value_variants<'a>() -> &'a [Self] {
        &[Default]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Default => PossibleValue::new(name_of!(Default)),
        })
    }
}

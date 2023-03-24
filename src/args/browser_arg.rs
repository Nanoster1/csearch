use crate::models::{Browser, Browser::Default};
use clap::{builder::PossibleValue, value_parser, Arg, ValueEnum};
use nameof::name_of;

pub struct BrowserArg;

impl BrowserArg {
    pub const ID: &str = "browser";
    pub const HELP: &str = "Select browser";
    pub const LONG: &str = "browser";
    pub const SHORT: char = 'b';
}

impl Into<Arg> for BrowserArg {
    fn into(self) -> Arg {
        Arg::new(Self::ID)
            .help(Self::HELP)
            .long(Self::LONG)
            .short(Self::SHORT)
            .required(false)
            .value_parser(value_parser!(Browser))
            .default_value(name_of!(Default))
    }
}

impl ValueEnum for Browser {
    fn value_variants<'a>() -> &'a [Self] {
        &[Default]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Default => PossibleValue::new(name_of!(Default)),
        })
    }
}

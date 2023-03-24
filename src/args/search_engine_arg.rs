use crate::models::{
    SearchEngine, SearchEngine::Bing, SearchEngine::DuckDuckGo, SearchEngine::Google,
    SearchEngine::Yandex,
};
use clap::{builder::PossibleValue, value_parser, Arg, ValueEnum};
use nameof::name_of;

pub struct SearchEngineArg;

impl SearchEngineArg {
    pub const ID: &str = "Search Engine";
    pub const HELP: &str = "Select search engine";
    pub const LONG: &str = "search";
    pub const SHORT: char = 's';
}

impl Into<Arg> for SearchEngineArg {
    fn into(self) -> Arg {
        Arg::new(Self::ID)
            .help(Self::HELP)
            .long(Self::LONG)
            .short(Self::SHORT)
            .required(false)
            .value_parser(value_parser!(SearchEngine))
            .default_value(name_of!(Yandex))
    }
}

impl ValueEnum for SearchEngine {
    fn value_variants<'a>() -> &'a [Self] {
        &[Yandex, Google, Bing, DuckDuckGo]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Yandex => PossibleValue::new(name_of!(Yandex)),
            Google => PossibleValue::new(name_of!(Google)),
            Bing => PossibleValue::new(name_of!(Bing)),
            DuckDuckGo => PossibleValue::new(name_of!(DuckDuckGo)),
        })
    }
}

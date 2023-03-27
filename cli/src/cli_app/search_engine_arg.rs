use self::SearchEngineArg::*;
use app::models::SearchEngine;
use clap::{builder::PossibleValue, value_parser, Arg, ValueEnum};
use nameof::name_of;

#[derive(Clone)]
pub enum SearchEngineArg {
    Yandex,
    Google,
    Bing,
    DuckDuckGo,
}

impl SearchEngineArg {
    pub const ID: &str = "Search Engine";
    pub const HELP: &str = "Select search engine";
    pub const LONG: &str = "search";
    pub const SHORT: char = 's';
    pub const IS_REQUIRED: bool = false;
    pub const DEFAULT_VALUE: &str = name_of!(Yandex);

    pub fn describe_argument() -> Arg {
        Arg::new(Self::ID)
            .help(Self::HELP)
            .long(Self::LONG)
            .short(Self::SHORT)
            .required(Self::IS_REQUIRED)
            .value_parser(value_parser!(SearchEngineArg))
            .default_value(Self::DEFAULT_VALUE)
    }

    pub fn to_model(&self) -> SearchEngine {
        match self {
            Yandex => SearchEngine::Yandex,
            Google => SearchEngine::Google,
            Bing => SearchEngine::Bing,
            DuckDuckGo => SearchEngine::DuckDuckGo,
        }
    }
}

impl ValueEnum for SearchEngineArg {
    fn value_variants<'a>() -> &'a [Self] {
        &[Yandex, Google, Bing, DuckDuckGo]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Yandex => PossibleValue::new(name_of!(Yandex)),
            Google => PossibleValue::new(name_of!(Google)),
            Bing => PossibleValue::new(name_of!(Bing)),
            DuckDuckGo => PossibleValue::new(name_of!(DuckDuckGo)),
        })
    }
}

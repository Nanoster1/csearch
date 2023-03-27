use app::models::Query;
use clap::{value_parser, Arg};
use std::str::FromStr;

#[derive(Clone)]
pub struct QueryArg(String);

impl QueryArg {
    pub const ID: &str = "Query";
    pub const HELP: &str = "Query for search in browser";
    pub const IS_REQUIRED: bool = false;

    pub fn describe_argument() -> Arg {
        Arg::new(Self::ID)
            .help(Self::HELP)
            .required(Self::IS_REQUIRED)
            .value_parser(value_parser!(QueryArg))
    }

    pub fn to_model(&self) -> Query {
        Query(self.0.clone())
    }
}

impl FromStr for QueryArg {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(String::from(s)))
    }
}

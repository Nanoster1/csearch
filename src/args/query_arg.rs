use crate::models::Query;
use clap::{value_parser, Arg};

pub struct QueryArg;

impl QueryArg {
    pub const ID: &str = "Query";
    pub const HELP: &str = "Query for search in browser";
}

impl Into<Arg> for QueryArg {
    fn into(self) -> Arg {
        Arg::new(Self::ID)
            .help(Self::HELP)
            .required(true)
            .value_parser(value_parser!(Query))
    }
}

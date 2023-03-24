mod args;
mod core_logic;
mod models;

use crate::args::BrowserArg;
use anyhow::{Ok, Result};
use args::{QueryArg, SearchEngineArg};
use clap::{command, crate_authors, crate_description, crate_name, crate_version};
use core_logic::{create_search_url, open_browser};
use models::{Browser, Query, SearchEngine};

fn main() -> Result<()> {
    let matches = command!(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(QueryArg)
        .arg(BrowserArg)
        .arg(SearchEngineArg)
        .get_matches();

    let query: &Query = matches.get_one(QueryArg::ID).unwrap();
    let browser: &Browser = matches.get_one(BrowserArg::ID).unwrap();
    let search_engine: &SearchEngine = matches.get_one(SearchEngineArg::ID).unwrap();

    let url = create_search_url(query, search_engine)?;
    open_browser(browser, &url)?;

    Ok(())
}

mod browser_arg;
mod query_arg;
mod search_engine_arg;

use self::{browser_arg::BrowserArg, query_arg::QueryArg, search_engine_arg::SearchEngineArg};
use anyhow::Result;
use app::{
    actions::{open_browser, OpenBrowserOptions},
    models::{Browser, Query, SearchEngine},
};
use clap::{command, crate_authors, crate_description, crate_version, Command};

pub struct CliApp(Command);

impl CliApp {
    const NAME: &str = "csearch";
    const AUTHOR: &str = crate_authors!();
    const ABOUT: &str = crate_description!();
    const VERSION: &str = crate_version!();

    pub fn build_cli() -> Result<Self> {
        let command = command!(Self::NAME)
            .author(Self::AUTHOR)
            .about(Self::ABOUT)
            .version(Self::VERSION)
            .arg(QueryArg::describe_argument())
            .arg(BrowserArg::describe_argument())
            .arg(SearchEngineArg::describe_argument());

        Ok(Self(command))
    }

    pub fn execute(self) -> Result<()> {
        let matches = self.0.get_matches();

        let query: Query = matches
            .get_one::<QueryArg>(QueryArg::ID)
            .unwrap()
            .to_model();
        let browser: Browser = matches
            .get_one::<BrowserArg>(BrowserArg::ID)
            .unwrap()
            .to_model();
        let search_engine: SearchEngine = matches
            .get_one::<SearchEngineArg>(SearchEngineArg::ID)
            .unwrap()
            .to_model();

        let options = OpenBrowserOptions {
            browser: &browser,
            query: &query,
            search_engine: &search_engine,
        };

        open_browser(options)?;

        Ok(())
    }
}

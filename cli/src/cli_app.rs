mod actions;
mod browser_arg;
mod completions_arg;
mod query_arg;
mod search_engine_arg;

use self::{
    actions::generate_completions, browser_arg::BrowserArg, completions_arg::CompletionsArg,
    query_arg::QueryArg, search_engine_arg::SearchEngineArg,
};
use anyhow::{Ok, Result};
use app::{
    actions::{open_browser, OpenBrowserOptions},
    models::{Browser, Query, SearchEngine},
};
use clap::{command, crate_authors, crate_description, crate_name, crate_version, Command};

pub struct CliApp {
    pub command: Command,
}

impl CliApp {
    const NAME: &str = crate_name!();
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
            .arg(SearchEngineArg::describe_argument())
            .arg(CompletionsArg::describe_argument());

        Ok(Self { command })
    }

    pub fn execute(self) -> Result<()> {
        let matches = self.command.get_matches();

        if matches.contains_id(CompletionsArg::ID) {
            let completions_arg = matches
                .get_one::<CompletionsArg>(CompletionsArg::ID)
                .unwrap();

            generate_completions(completions_arg, &mut std::io::stdout())?;

            return Ok(());
        }

        if matches.contains_id(QueryArg::ID) {
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

            return Ok(());
        }

        CliApp::build_cli()?.command.print_help()?;

        Ok(())
    }
}

use crate::models::{Browser as ModelBrowser, Query, SearchEngine};
use anyhow::Result;
use url::Url;
use urlencoding::encode;
use webbrowser::{open_browser as open_webbrowser, Browser as WebBrowser};

pub struct OpenBrowserOptions<'a> {
    pub browser: &'a ModelBrowser,
    pub query: &'a Query,
    pub search_engine: &'a SearchEngine,
}

pub fn open_browser(options: OpenBrowserOptions) -> Result<()> {
    let browser = choose_webbrowser(options.browser);
    let url = create_search_url(options.query, options.search_engine)?;
    open_webbrowser(browser, url.as_str())?;
    Ok(())
}

fn create_search_url(query: &Query, search_engine: &SearchEngine) -> Result<Url> {
    let search_template = search_engine.get_search_template();
    let encoded_query = encode(query.0.as_str());
    let search_url = format!("{}{}", search_template, encoded_query);
    let result = Url::parse(&search_url)?;
    Ok(result)
}

fn choose_webbrowser(browser: &ModelBrowser) -> WebBrowser {
    match browser {
        ModelBrowser::Default => WebBrowser::Default,
    }
}

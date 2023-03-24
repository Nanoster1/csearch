use crate::models::Browser as ModelBrowser;
use anyhow::Result;
use url::Url;
use webbrowser::{open_browser as open_webbrowser, Browser as WebBrowser};

pub fn open_browser(browser: &ModelBrowser, url: &Url) -> Result<()> {
    let browser = choose_browser(browser);
    open_webbrowser(browser, url.as_str())?;
    Ok(())
}

fn choose_browser(browser: &ModelBrowser) -> WebBrowser {
    match browser {
        ModelBrowser::Default => WebBrowser::Default,
    }
}

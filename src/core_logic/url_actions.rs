use anyhow::Result;
use url::Url;
use urlencoding::encode;

use crate::models::{Query, SearchEngine};

pub fn create_search_url(query: &Query, search_engine: &SearchEngine) -> Result<Url> {
    let search_template = search_engine.get_search_template();
    let encoded_query = encode(query.0.as_str());
    let search_url = format!("{}{}", search_template, encoded_query);
    let result = Url::parse(&search_url)?;
    Ok(result)
}

#[derive(Clone)]
pub enum SearchEngine {
    Yandex,
    Google,
    Bing,
    DuckDuckGo,
}

impl SearchEngine {
    pub fn get_search_template(&self) -> &str {
        match self {
            SearchEngine::Yandex => "https://yandex.ru/search/?text=",
            SearchEngine::Google => "https://www.google.com/search?q=",
            SearchEngine::Bing => "https://www.bing.com/search?q=",
            SearchEngine::DuckDuckGo => "https://duckduckgo.com/?q=",
        }
    }
}

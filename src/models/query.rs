use std::str::FromStr;

#[derive(Clone)]
pub struct Query(pub String);

impl FromStr for Query {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(String::from(s)))
    }
}

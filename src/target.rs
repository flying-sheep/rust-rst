use std::path::PathBuf;
use std::str::FromStr;
use std::string::ParseError;

use url::{self,Url};
use serde_derive::Serialize;


#[derive(Debug,PartialEq,Serialize)]
#[serde(untagged)]
pub enum Target {
    #[serde(serialize_with = "serialize_url")]
    Url(Url),
    Path(PathBuf),
}

impl From<Url> for Target {
    fn from(url: Url) -> Self {
        Target::Url(url)
    }
}

impl From<PathBuf> for Target {
    fn from(path: PathBuf) -> Self {
        Target::Path(path)
    }
}


impl FromStr for Target {
    type Err = ParseError;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match Url::parse(input) {
            Ok(url) => url.into(),
            Err(_) => PathBuf::from(input.trim()).into(),
        })
    }
}


pub fn serialize_url<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
	serializer.serialize_str(url.as_str())
}

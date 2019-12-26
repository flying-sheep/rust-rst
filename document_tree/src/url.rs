use std::fmt;
use std::str::FromStr;

use url::{self,ParseError};
use serde_derive::Serialize;


fn starts_with_scheme(input: &str) -> bool {
	let scheme = input.split(':').next().unwrap();
	if scheme == input || scheme.is_empty() {
		return false;
	}
	let mut chars = input.chars();
	// First character.
	if !chars.next().unwrap().is_ascii_alphabetic() {
		return false;
	}
	for ch in chars {
		if !ch.is_ascii_alphanumeric() && ch != '+' && ch != '-' && ch != '.' {
			return false;
		}
	}
	true
}

/// The string representation of a URL, either absolute or relative, that has
/// been verified as a valid URL on construction.
#[derive(Debug,PartialEq,Serialize,Clone)]
#[serde(transparent)]
pub struct Url(String);

impl Url {
	pub fn parse_absolute(input: &str) -> Result<Self, ParseError> {
		Ok(url::Url::parse(input)?.into())
	}
	pub fn parse_relative(input: &str) -> Result<Self, ParseError> {
		// We're assuming that any scheme through which RsT documents are being
		// accessed is a hierarchical scheme, and so we can parse relative to a
		// random hierarchical URL.
		if input.starts_with('/') || !starts_with_scheme(input) {
			// Continue only if the parse succeeded, disregarding its result.
			let random_base_url = url::Url::parse("https://a/b").unwrap();
			url::Url::options()
				.base_url(Some(&random_base_url))
				.parse(input)?;
			Ok(Url(input.into()))
		} else {
			// If this is a URL at all, it's an absolute one.
			// There's no appropriate variant of url::ParseError really.
			Err(ParseError::SetHostOnCannotBeABaseUrl)
		}
	}
	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}
}

impl From<url::Url> for Url {
	fn from(url: url::Url) -> Self {
		Url(url.into_string())
	}
}


impl fmt::Display for Url {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.as_str())
	}
}


impl FromStr for Url {
	type Err = ParseError;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		Url::parse_absolute(input)
			.or_else(|_| Url::parse_relative(input))
	}
}

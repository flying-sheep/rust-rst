use url::Url;

pub fn serialize_url<S>(url: &Url, serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
	serializer.serialize_str(url.as_str())
}

pub fn serialize_opt_url<S>(url_opt: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
	match url_opt {
		Some(ref url) => serializer.serialize_some(url.as_str()),
		None          => serializer.serialize_none(),
	}
}

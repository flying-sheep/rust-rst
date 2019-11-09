use failure::Error;
use pest::iterators::Pair;

use crate::document_tree::{
	elements as e,
	element_categories as c,
	extra_attributes as a,
	attribute_types as at
};

use crate::parser::{
	pest_rst::Rule,
//    pair_ext_parse::PairExt,
};

use crate::url::Url;
use super::whitespace_normalize_name;


pub fn convert_inline(pair: Pair<Rule>) -> Result<c::TextOrInlineElement, Error> {
	Ok(match pair.as_rule() {
		Rule::str       		=> pair.as_str().into(),
		Rule::reference 		=> convert_reference(pair)?,
		Rule::substitution_ref 	=> convert_substitution(pair)?.into(),
		rule => unimplemented!("unknown rule {:?}", rule),
	})
}

fn convert_reference(pair: Pair<Rule>) -> Result<c::TextOrInlineElement, Error> {
	let name;
	let refuri;
	let refid;
	let mut refname = vec![];
	let mut children: Vec<c::TextOrInlineElement> = vec![];
	let concrete = pair.into_inner().next().unwrap();
	match concrete.as_rule() {
		Rule::reference_target => {
			let rt_inner = concrete.into_inner().next().unwrap(); // reference_target_uq or target_name_qu
			match rt_inner.as_rule() {
				Rule::reference_target_uq => {
					refid  = None;
					name   = Some(rt_inner.as_str().into());
					refuri = None;
					refname.push(rt_inner.as_str().into());
					children.push(rt_inner.as_str().into());
				},
				Rule::reference_target_qu => {
					let (text, reference) = {
						let mut text = None;
						let mut reference = None;
						for inner in rt_inner.clone().into_inner() {
							match inner.as_rule() {
								Rule::reference_text => text = Some(inner),
								Rule::reference_bracketed => reference = Some(inner),
								_ => unreachable!()
							}
						}
						(text, reference)
					};
					let trimmed_text = match (&text, &reference) {
						(Some(text), None) => text.as_str(),
						(_, Some(reference)) => {
							text
								.map(|text| text.as_str().trim_end_matches(|ch| " \n\r".contains(ch)))
								.filter(|text| !text.is_empty())
								.unwrap_or_else(|| reference.clone().into_inner().next().unwrap().as_str())
						}
						(None, None) => unreachable!()
					};
					refid = None;
					name = Some(trimmed_text.into());
					refuri = if let Some(reference) = reference {
						let inner = reference.into_inner().next().unwrap();
						match inner.as_rule() {
							// The URL rules in our parser accept a narrow superset of
							// valid URLs, so we need to handle false positives.
							Rule::url => if let Ok(target) = Url::parse_absolute(inner.as_str()) {
								Some(target)
							} else if inner.as_str().ends_with('_') {
								// like target_name_qu (minus the final underscore)
								let full_str = inner.as_str();
								refname.push(full_str[0..full_str.len() - 1].into());
								None
							} else {
								// like relative_reference
								Some(Url::parse_relative(inner.as_str())?)
							},
							Rule::target_name_qu => {
								refname.push(inner.as_str().into());
								None
							},
							Rule::relative_reference => {
								Some(Url::parse_relative(inner.as_str())?)
							},
							_ => unreachable!()
						}
					} else {
						refname.push(trimmed_text.into());
						None
					};
					children.push(trimmed_text.into());
				},
				_ => unreachable!()
			}
		},
		Rule::reference_explicit => unimplemented!("explicit reference"),
		Rule::reference_auto => {
			let rt_inner = concrete.into_inner().next().unwrap();
			match rt_inner.as_rule() {
				Rule::url_auto => match Url::parse_absolute(rt_inner.as_str()) {
					Ok(target) => {
						refuri = Some(target);
						name   = None;
						refid  = None;
						children.push(rt_inner.as_str().into());
					},
					// if our parser got a URL wrong, return it as a string
					Err(_) => return Ok(rt_inner.as_str().into())
				},
				Rule::email => {
					let mailto_url = String::from("mailto:") + rt_inner.as_str();
					match Url::parse_absolute(&mailto_url) {
						Ok(target) => {
							refuri = Some(target);
							name   = None;
							refid  = None;
							children.push(rt_inner.as_str().into());
						},
						// if our parser got a URL wrong, return it as a string
						Err(_) => return Ok(rt_inner.as_str().into())
					}
				},
				_ => unreachable!()
			}
		},
		_ => unreachable!(),
	};
	Ok(e::Reference::new(
		Default::default(),
		a::Reference { name, refuri, refid, refname },
		children
	).into())
}

fn convert_substitution(pair: Pair<Rule>) -> Result<e::SubstitutionReference, Error> {
	let concrete = pair.into_inner().next().unwrap();
	match concrete.as_rule() {
		Rule::substitution_name => {
			let name = whitespace_normalize_name(concrete.as_str());
			Ok(a::ExtraAttributes::with_extra(
				a::SubstitutionReference {
					refname: vec![at::NameToken(name)]
				}
			))
		}
		_ => unreachable!()
	}
}

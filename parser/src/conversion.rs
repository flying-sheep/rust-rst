mod block;
mod inline;
#[cfg(test)]
mod tests;

use failure::Error;
use pest::iterators::Pairs;

use document_tree::{
	Element,HasChildren,
	elements as e,
	element_categories as c,
	attribute_types as at,
};

use crate::pest_rst::Rule;


fn ssubel_to_section_unchecked_mut(ssubel: &mut c::StructuralSubElement) -> &mut e::Section {
	match ssubel {
		c::StructuralSubElement::SubStructure(ref mut b) => match **b {
			c::SubStructure::Section(ref mut s) => s,
			_ => unreachable!(),
		},
		_ => unreachable!(),
	}
}


fn get_level<'tl>(toplevel: &'tl mut Vec<c::StructuralSubElement>, section_idxs: &[Option<usize>]) -> &'tl mut Vec<c::StructuralSubElement> {
	let mut level = toplevel;
	for maybe_i in section_idxs {
		if let Some(i) = *maybe_i {
			level = ssubel_to_section_unchecked_mut(&mut level[i]).children_mut();
		}
	}
	level
}


pub fn convert_document(pairs: Pairs<Rule>) -> Result<e::Document, Error> {
	use self::block::TitleOrSsubel::*;
	
	let mut toplevel: Vec<c::StructuralSubElement> = vec![];
	// The kinds of section titles encountered.
	// `section_idx[x]` has the kind `kinds[x]`, but `kinds` can be longer
	let mut kinds: Vec<block::TitleKind> = vec![];
	// Recursive indices into the tree, pointing at the active sections.
	// `None`s indicate skipped section levels:
	// toplevel[section_idxs.flatten()[0]].children[section_idxs.flatten()[1]]...
	let mut section_idxs: Vec<Option<usize>> = vec![];
	
	for pair in pairs {
		if let Some(ssubel) = block::convert_ssubel(pair)? { match ssubel {
			Title(title, kind) => {
				match kinds.iter().position(|k| k == &kind) {
					// Idx points to the level we want to add,
					// so idx-1 needs to be the last valid index.
					Some(idx) => {
						// If idx < len: Remove found section and all below
						section_idxs.truncate(idx);
						// If idx > len: Add None for skipped levels
						// TODO: test skipped levels
						while section_idxs.len() < idx { section_idxs.push(None) }
					},
					None => kinds.push(kind),
				}
				let super_level = get_level(&mut toplevel, &section_idxs);
				let slug = title.names().iter().next().map(|at::NameToken(name)| at::ID(name.to_owned()));
				let mut section = e::Section::with_children(vec![title.into()]);
				section.ids_mut().extend(slug.into_iter());
				super_level.push(section.into());
				section_idxs.push(Some(super_level.len() - 1));
			},
			Ssubel(elem) => get_level(&mut toplevel, &section_idxs).push(elem),
		}}
	}
	Ok(e::Document::with_children(toplevel))
}

/// Normalizes a name in terms of whitespace. Equivalent to docutils's
/// `docutils.nodes.whitespace_normalize_name`.
pub fn whitespace_normalize_name(name: &str) -> String {
	// Python's string.split() defines whitespace differently than Rust does.
	let split_iter = name.split(
		|ch: char| ch.is_whitespace() || ('\x1C'..='\x1F').contains(&ch)
	).filter(|split| !split.is_empty());
	let mut ret = String::new();
	for split in split_iter {
		if !ret.is_empty() {
			ret.push(' ');
		}
		ret.push_str(split);
	}
	ret
}

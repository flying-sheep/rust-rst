use document_tree::{
	elements as e,
	element_categories as c,
	HasChildren,
};

use crate::parse;

fn ssubel_to_section(ssubel: &c::StructuralSubElement) -> &e::Section {
	match ssubel {
		c::StructuralSubElement::SubStructure(ref b) => match **b {
			c::SubStructure::Section(ref s) => s,
			ref c => panic!("Expected section, not {:?}", c),
		},
		ref c => panic!("Expected SubStructure, not {:?}", c),
	}
}

const SECTIONS: &str = "\
Intro before first section title

Level 1
*******

-------
Level 2
-------

Level 3
=======

L1 again
********

L3 again, skipping L2
=====================
";

#[test]
fn convert_skipped_section() {
	let doctree = parse(SECTIONS).unwrap();
	let lvl0 = doctree.children();
	assert_eq!(lvl0.len(), 3, "Should be a paragraph and 2 sections: {:?}", lvl0);

	assert_eq!(lvl0[0], e::Paragraph::with_children(vec![
		"Intro before first section title".to_owned().into()
	]).into(), "The intro text should fit");

	let lvl1a = ssubel_to_section(&lvl0[1]).children();
	assert_eq!(lvl1a.len(), 2, "The 1st lvl1 section should have (a title and) a single lvl2 section as child: {:?}", lvl1a);
	//TODO: test title lvl1a[0]
	let lvl2  = ssubel_to_section(&lvl1a[1]).children();
	assert_eq!(lvl2.len(), 2, "The lvl2 section should have (a title and) a single lvl3 section as child: {:?}", lvl2);
	//TODO: test title lvl2[0]
	let lvl3a = ssubel_to_section(&lvl2[1]).children();
	assert_eq!(lvl3a.len(), 1, "The 1st lvl3 section should just a title: {:?}", lvl3a);
	//TODO: test title lvl3a[0]

	let lvl1b = ssubel_to_section(&lvl0[2]).children();
	assert_eq!(lvl1b.len(), 2, "The 2nd lvl1 section should have (a title and) a single lvl2 section as child: {:?}", lvl1b);
	//TODO: test title lvl1b[0]
	let lvl3b = ssubel_to_section(&lvl1b[1]).children();
	assert_eq!(lvl3b.len(), 1, "The 2nd lvl3 section should have just a title: {:?}", lvl3b);
	//TODO: test title lvl3b[0]
}

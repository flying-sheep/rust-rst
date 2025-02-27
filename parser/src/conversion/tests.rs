use document_tree::{
    HasChildren, element_categories as c, elements as e, extra_attributes::ExtraAttributes,
};

use crate::parse;

fn ssubel_to_section(ssubel: &c::StructuralSubElement) -> &e::Section {
    match ssubel {
        c::StructuralSubElement::SubStructure(b) => match b.as_ref() {
            c::SubStructure::Section(s) => s,
            c => panic!("Expected section, not {c:?}"),
        },
        c => panic!("Expected SubStructure, not {c:?}"),
    }
}

fn ssubel_to_body_element(ssubel: &c::StructuralSubElement) -> &c::BodyElement {
    match ssubel {
        c::StructuralSubElement::SubStructure(b) => match b.as_ref() {
            c::SubStructure::BodyElement(b) => b,
            c => panic!("Expected BodyElement, not {c:?}"),
        },
        c => panic!("Expected SubStructure, not {c:?}"),
    }
}

fn body_element_to_image(bodyel: &c::BodyElement) -> &e::Image {
    match bodyel {
        c::BodyElement::Image(i) => i,
        c => panic!("Expected Image, not {c:?}"),
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
    assert_eq!(
        lvl0.len(),
        3,
        "Should be a paragraph and 2 sections: {lvl0:?}"
    );

    assert_eq!(
        lvl0[0],
        e::Paragraph::with_children(vec!["Intro before first section title".to_owned().into()])
            .into(),
        "The intro text should fit"
    );

    let lvl1_a = ssubel_to_section(&lvl0[1]).children();
    assert_eq!(
        lvl1_a.len(),
        2,
        "The 1st lvl1 section should have (a title and) a single lvl2 section as child: {lvl1_a:?}"
    );
    //TODO: test title lvl1a[0]
    let lvl2 = ssubel_to_section(&lvl1_a[1]).children();
    assert_eq!(
        lvl2.len(),
        2,
        "The lvl2 section should have (a title and) a single lvl3 section as child: {lvl2:?}"
    );
    //TODO: test title lvl2[0]
    let lvl3_a = ssubel_to_section(&lvl2[1]).children();
    assert_eq!(
        lvl3_a.len(),
        1,
        "The 1st lvl3 section should just a title: {lvl3_a:?}"
    );
    //TODO: test title lvl3a[0]

    let lvl1_b = ssubel_to_section(&lvl0[2]).children();
    assert_eq!(
        lvl1_b.len(),
        2,
        "The 2nd lvl1 section should have (a title and) a single lvl2 section as child: {lvl1_b:?}"
    );
    //TODO: test title lvl1b[0]
    let lvl3_b = ssubel_to_section(&lvl1_b[1]).children();
    assert_eq!(
        lvl3_b.len(),
        1,
        "The 2nd lvl3 section should have just a title: {lvl3_b:?}"
    );
    //TODO: test title lvl3b[0]
}

#[test]
fn test_convert_image_scale() {
    let doctree = parse(".. image:: /path/to/img.jpg\n   :scale: 90%\n\n").unwrap();
    let lvl0 = doctree.children();
    assert_eq!(lvl0.len(), 1, "Should be a single image: {lvl0:?}");
    let be = ssubel_to_body_element(&lvl0[0]);
    let img = body_element_to_image(be);
    assert_eq!(img.extra().scale, Some(90));
    assert_eq!(img.extra().uri, "/path/to/img.jpg".parse().unwrap());
}

mod block;
mod inline;

use failure::Error;
use pest::iterators::Pairs;

use crate::document_tree::{
    HasChildren,
    elements as e,
    element_categories as c,
};

use super::pest_rst::Rule;


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
                super_level.push(e::Section::with_children(vec![title.into()]).into());
                section_idxs.push(Some(super_level.len() - 1));
            },
            Ssubel(elem) => get_level(&mut toplevel, &section_idxs).push(elem),
        }}
    }
    Ok(e::Document::with_children(toplevel))
}


#[cfg(test)]
mod tests {
    use crate::{
        parser::parse,
        document_tree::{
            elements as e,
            element_categories as c,
            HasChildren,
        }
    };
    
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
}

use failure::Error;
use pest::iterators::Pair;

use crate::document_tree::{
    ExtraAttributes,
    elements as e,
    element_categories as c,
    attribute_types::{ID,NameToken},
    extra_attributes as a,
};

use crate::parser::{
    pest_rst::Rule,
//    pair_ext_parse::PairExt,
};


pub fn convert_inline(pair: Pair<Rule>) -> Result<c::TextOrInlineElement, Error> {
    Ok(match pair.as_rule() {
        Rule::str       => pair.as_str().into(),
        Rule::reference => convert_reference(pair)?.into(),
        rule => unimplemented!("unknown rule {:?}", rule),
    })
}

fn convert_reference(pair: Pair<Rule>) -> Result<e::Reference, Error> {
    let mut name = None;
    let mut uri = None;
    let mut id = None;
    let mut name_tokens = vec![];
    let concrete = pair.into_inner().next().unwrap();
    match concrete.as_rule() {
        Rule::reference_target => {
            let rt_inner = concrete.into_inner().next().unwrap(); // reference_target_uq or target_name_qu
            //TODO: abstract
            id   = Some(       ID(rt_inner.as_str().to_owned().replace(' ', "-")));
            name = Some(NameToken(rt_inner.as_str().to_owned()));
        },
        Rule::reference_explicit => unimplemented!("explicit reference"),
        Rule::reference_auto => unimplemented!("auto reference"),
        _ => unreachable!(),
    };
    let extra = a::Reference {
        name: name,
        refuri: uri,
        refid: id,
        refname: name_tokens,
    };
    Ok(e::Reference::with_extra(extra))
}

use failure::Error;
use pest::iterators::Pair;

use crate::document_tree::{
    ExtraAttributes,
    elements as e,
    element_categories as c,
//    attribute_types::ID,
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
        rule => panic!("unknown rule {:?}", rule),
    })
}

fn convert_reference(pair: Pair<Rule>) -> Result<e::Reference, Error> {
    let name = None;
    let uri = None;
    let id = None;
    let name_tokens = vec![];
    let extra = a::Reference {
        name: name,
        refuri: uri,
        refid: id,
        refname: name_tokens,
    };
    Ok(e::Reference::with_extra(extra))
}

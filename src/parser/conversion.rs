use failure::{Error,bail};
use failure_derive::Fail;
use pest::iterators::{Pairs,Pair};

use crate::document_tree::{
    Element,HasChildren,ExtraAttributes,
    elements as e,
    element_categories as c,
    attribute_types::ID,
    extra_attributes as a,
};

use super::pest_rst::Rule;


#[derive(Debug, Fail)]
enum ConversionError {
    #[fail(display = "unknown rule: {:?}", rule)]
    UnknownRuleError {
        rule: Rule,
    },
}


trait PairExt<R> where R: pest::RuleType {
    fn parse<T, E>(&self) -> Result<T, pest::error::Error<R>> where T: std::str::FromStr<Err = E>, E: ToString;
}
impl<'l, R> PairExt<R> for Pair<'l, R> where R: pest::RuleType {
    fn parse<T, E>(&self) -> Result<T, pest::error::Error<R>> where T: std::str::FromStr<Err = E>, E: ToString {
        self.as_str().parse().map_err(|e: T::Err| {
            use pest::error::{Error,ErrorVariant};
            let var: ErrorVariant<R> = ErrorVariant::CustomError { message: e.to_string() };
            Error::new_from_span(var, self.as_span())
        })
    }
}


pub fn convert_document(pairs: Pairs<Rule>) -> Result<e::Document, Error> {
    let structural_elems = pairs.map(convert_ssubel)
        .filter_map(|elem| match elem { Ok(Some(e)) => Some(Ok(e)), Err(e) => Some(Err(e)), Ok(None) => None })
        .collect::<Result<_,_>>()?;
    Ok(e::Document::with_children(structural_elems))
}


fn convert_ssubel(pair: Pair<Rule>) -> Result<Option<c::StructuralSubElement>, Error> {
    // TODO: This is just a proof of concep. Keep closely to DTD in final version!
    Ok(Some(match pair.as_rule() {
        Rule::title            => convert_title(pair).into(),
        Rule::paragraph        => e::Paragraph::with_children(vec![pair.as_str().into()]).into(),
        Rule::target           => convert_target(pair)?.into(),
        Rule::substitution_def => convert_substitution_def(pair)?.into(),
        Rule::admonition_gen   => convert_admonition_gen(pair)?.into(),
        Rule::image            => convert_image::<e::Image>(pair)?.into(),
        Rule::EOI              => return Ok(None),
        rule => return Err(ConversionError::UnknownRuleError { rule }.into()),
    }))
}


fn convert_title(pair: Pair<Rule>) -> e::Title {
    let mut title: Option<&str> = None;
    let mut _adornment_char: Option<char> = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::line => title = Some(p.as_str()),
            Rule::adornments => _adornment_char = Some(p.as_str().chars().next().expect("Empty adornment?")),
            rule => panic!("Unexpected rule in title: {:?}", rule),
        };
    }
    // TODO adornment char
    e::Title::with_children(vec![
        title.expect("No text in title").into()
    ])
}

fn convert_target(pair: Pair<Rule>) -> Result<e::Target, Error> {
    let mut attrs = a::Target {
        anonymous: false,
        ..Default::default()
    };
    for p in pair.into_inner() {
        match p.as_rule() {
            // TODO: or is it refnames?
            Rule::target_name_uq | Rule::target_name_qu => attrs.refid = Some(ID(p.as_str().to_owned())),
            Rule::link_target => attrs.refuri = Some(p.parse()?),
            rule => panic!("Unexpected rule in target: {:?}", rule),
        }
    }
    Ok(e::Target::new(Default::default(), attrs))
}

fn convert_substitution_def(pair: Pair<Rule>) -> Result<e::SubstitutionDefinition, Error> {
    let mut pairs = pair.into_inner();
    let name = pairs.next().unwrap().as_str();  // Rule::substitution_name
    let inner_pair = pairs.next().unwrap();
    let inner: c::TextOrInlineElement = match inner_pair.as_rule() {
        Rule::image => convert_image::<e::ImageInline>(inner_pair)?.into(),
        rule => panic!("Unknown substitution rule {:?}", rule),
    };
    let mut subst_def = e::SubstitutionDefinition::with_children(vec![inner.into()]);
    subst_def.names_mut().push(name.to_owned());
    Ok(subst_def)
}

fn convert_image<I>(pair: Pair<Rule>) -> Result<I, Error> where I: Element + ExtraAttributes<a::Image> {
    let mut pairs = pair.into_inner();
    let mut image = I::with_extra(a::Image::new(
        pairs.next().unwrap().parse()?,  // line
    ));
    if let Some(opt_block) = pairs.next() {  // image_opt_block
        let options = opt_block.into_inner();
        for opt in options {
            let mut opt_iter = opt.into_inner();
            let opt_name = opt_iter.next().unwrap();
            let opt_val = opt_iter.next().unwrap();
            match opt_name.as_str() {
                "class"  => image.classes_mut().push(opt_val.as_str().to_owned()),
                "name"   => image.names_mut().push(opt_val.as_str().to_owned()),
                "alt"    => image.extra_mut().alt    = Some(opt_val.as_str().to_owned()),
                "height" => image.extra_mut().height = Some(opt_val.parse()?),
                "width"  => image.extra_mut().width  = Some(opt_val.parse()?),
                "scale"  => image.extra_mut().scale  = Some(opt_val.parse()?),  // TODO: can end with %
                "align"  => image.extra_mut().align  = Some(opt_val.parse()?),
                "target" => image.extra_mut().target = Some(opt_val.parse()?),
                name => bail!("Unknown Image option {}", name),
            }
        }
    }
    Ok(image)
}

fn convert_admonition_gen(pair: Pair<Rule>) -> Result<c::BodyElement, Error> {
    let mut iter = pair.into_inner();
    let typ = iter.next().unwrap().as_str();
    // TODO: in reality it contains body elements.
    let children: Vec<c::BodyElement> = iter.map(|p| e::Paragraph::with_children(vec![p.as_str().into()]).into()).collect();
    Ok(match typ {
        "attention" => e::Attention::with_children(children).into(),
        "hint"      =>      e::Hint::with_children(children).into(),
        "note"      =>      e::Note::with_children(children).into(),
        "caution"   =>   e::Caution::with_children(children).into(),
        "danger"    =>    e::Danger::with_children(children).into(),
        "error"     =>     e::Error::with_children(children).into(),
        "important" => e::Important::with_children(children).into(),
        "tip"       =>       e::Tip::with_children(children).into(),
        "warning"   =>   e::Warning::with_children(children).into(),
        typ         => panic!("Unknown admontion type {}!", typ),
    })
}

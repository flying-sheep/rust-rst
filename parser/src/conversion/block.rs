use anyhow::{Error, bail};
use pest::iterators::Pair;

use document_tree::{
    Element, ExtraAttributes, HasChildren, attribute_types as at, element_categories as c,
    elements as e, extra_attributes as a,
};

use super::{inline::convert_inlines, whitespace_normalize_name};
use crate::{pair_ext_parse::PairExt, pest_rst::Rule};

#[derive(PartialEq)]
pub(super) enum TitleKind {
    Double(char),
    Single(char),
}

pub(super) enum TitleOrSsubel {
    Title(e::Title, TitleKind),
    Ssubel(c::StructuralSubElement),
}

pub(super) fn convert_ssubel(pair: Pair<Rule>) -> Result<Option<TitleOrSsubel>, Error> {
    use self::TitleOrSsubel::{Ssubel, Title};
    Ok(Some(match pair.as_rule() {
        Rule::title => {
            let (t, k) = convert_title(pair)?;
            Title(t, k)
        }
        //TODO: subtitle, decoration, docinfo
        Rule::EOI => return Ok(None),
        _ => Ssubel(convert_substructure(pair)?.into()),
    }))
}

fn convert_substructure(pair: Pair<Rule>) -> Result<c::SubStructure, Error> {
    #[allow(clippy::match_single_binding)]
    Ok(match pair.as_rule() {
        // TODO: Topic, Sidebar, Transition
        // no section here, as it’s constructed from titles
        _ => convert_body_elem(pair)?.into(),
    })
}

fn convert_body_elem(pair: Pair<Rule>) -> Result<c::BodyElement, Error> {
    Ok(match pair.as_rule() {
        Rule::paragraph => convert_paragraph(pair)?.into(),
        Rule::target => convert_target(pair)?.into(),
        Rule::footnote => convert_footnote(pair)?.into(),
        Rule::substitution_def => convert_substitution_def(pair)?.into(),
        Rule::block_quote_directive => convert_block_quote_directive(pair)?.into(),
        Rule::admonition_gen => convert_admonition_gen(pair),
        Rule::image => convert_image::<e::Image>(pair)?.into(),
        Rule::bullet_list => convert_bullet_list(pair)?.into(),
        Rule::block_quote => convert_block_quote(pair)?.into(),
        Rule::literal_block => convert_literal_block(pair).into(),
        Rule::code_directive => convert_code_directive(pair).into(),
        Rule::raw_directive => convert_raw_directive(pair).into(),
        Rule::block_comment => convert_comment(pair).into(),
        rule => unimplemented!("unhandled rule {:?}", rule),
    })
}

fn convert_title(pair: Pair<Rule>) -> Result<(e::Title, TitleKind), Error> {
    let mut title: Option<String> = None;
    let mut title_inlines: Option<Vec<c::TextOrInlineElement>> = None;
    let mut adornment_char: Option<char> = None;
    // title_double or title_single. Extract kind before consuming
    let inner_pair = pair.into_inner().next().unwrap();
    let kind = inner_pair.as_rule();
    for p in inner_pair.into_inner() {
        match p.as_rule() {
            Rule::line => {
                title = Some(p.as_str().to_owned());
                title_inlines = Some(convert_inlines(p)?);
            }
            Rule::adornments => {
                adornment_char = Some(p.as_str().chars().next().expect("Empty adornment?"));
            }
            rule => unimplemented!("Unexpected rule in title: {:?}", rule),
        }
    }
    // now we encountered one line of text and one of adornments
    // TODO: emit error if the adornment line is too short (has to match title length)
    let mut elem = e::Title::with_children(title_inlines.expect("No text in title"));
    if let Some(title) = title {
        //TODO: slugify properly
        let slug = title.to_lowercase().replace('\n', "").replace(' ', "-");
        elem.names_mut().push(at::NameToken(slug));
    }
    let title_kind = match kind {
        Rule::title_double => TitleKind::Double(adornment_char.unwrap()),
        Rule::title_single => TitleKind::Single(adornment_char.unwrap()),
        _ => unreachable!(),
    };
    Ok((elem, title_kind))
}

fn convert_paragraph(pair: Pair<Rule>) -> Result<e::Paragraph, Error> {
    Ok(e::Paragraph::with_children(convert_inlines(pair)?))
}

fn convert_target(pair: Pair<Rule>) -> Result<e::Target, Error> {
    let mut elem = e::Target::default();
    elem.extra_mut().anonymous = false;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::target_name_uq | Rule::target_name_qu => {
                elem.ids_mut().push(p.as_str().into());
                elem.names_mut().push(p.as_str().into());
            }
            // TODO: also handle non-urls
            Rule::link_target => elem.extra_mut().refuri = Some(p.parse()?),
            rule => panic!("Unexpected rule in target: {rule:?}"),
        }
    }
    Ok(elem)
}

/// Converts a footnote.
/// - named auto-numbered footnotes get their name set
/// - explicitly numbered footnotes get their label set
fn convert_footnote(pair: Pair<Rule>) -> Result<e::Footnote, Error> {
    let mut pairs = pair.into_inner();
    let label = pairs.next().unwrap().as_str();
    let mut children: Vec<c::SubFootnote> = vec![];
    // turn `line` into paragraph
    children.push(convert_paragraph(pairs.next().unwrap())?.into());
    for p in pairs {
        children.push(convert_body_elem(p)?.into());
    }
    let mut footnote = e::Footnote::with_children(children);
    footnote.extra_mut().auto = label.chars().next().unwrap().try_into().ok();
    match footnote.extra().auto {
        Some(at::FootnoteType::Number) => {
            if label.len() > 1 {
                let name = whitespace_normalize_name(&label[1..]);
                footnote.names_mut().push(at::NameToken(name));
            }
        }
        Some(at::FootnoteType::Symbol) => {}
        None => {
            footnote
                .children_mut()
                .insert(0, e::Label::with_children(vec![label.into()]).into());
        }
    }
    Ok(footnote)
}

fn convert_substitution_def(pair: Pair<Rule>) -> Result<e::SubstitutionDefinition, Error> {
    let mut pairs = pair.into_inner();
    let name = whitespace_normalize_name(pairs.next().unwrap().as_str()); // Rule::substitution_name
    let inner_pair = pairs.next().unwrap();
    let inner: Vec<c::TextOrInlineElement> = match inner_pair.as_rule() {
        Rule::replace => convert_replace(inner_pair)?,
        Rule::image => vec![convert_image::<e::ImageInline>(inner_pair)?.into()],
        rule => panic!("Unknown substitution rule {rule:?}"),
    };
    let mut subst_def = e::SubstitutionDefinition::with_children(inner);
    subst_def.names_mut().push(at::NameToken(name));
    Ok(subst_def)
}

fn convert_replace(pair: Pair<Rule>) -> Result<Vec<c::TextOrInlineElement>, Error> {
    let mut pairs = pair.into_inner();
    let paragraph = pairs.next().unwrap();
    convert_inlines(paragraph)
}

fn convert_image<I>(pair: Pair<Rule>) -> Result<I, Error>
where
    I: Element + ExtraAttributes<a::Image>,
{
    let mut pairs = pair.into_inner();
    let mut image = I::with_extra(a::Image::new(
        pairs.next().unwrap().as_str().trim().parse()?, // line
    ));
    for opt in pairs {
        let mut opt_iter = opt.into_inner();
        let opt_name = opt_iter.next().unwrap();
        let opt_val = opt_iter.next().unwrap();
        match opt_name.as_str() {
            "class" => image.classes_mut().push(opt_val.as_str().to_owned()),
            "name" => image.names_mut().push(opt_val.as_str().into()),
            "alt" => image.extra_mut().alt = Some(opt_val.as_str().to_owned()),
            "height" => image.extra_mut().height = Some(opt_val.parse()?),
            "width" => image.extra_mut().width = Some(opt_val.parse()?),
            "scale" => image.extra_mut().scale = Some(parse_scale(&opt_val)?),
            "align" => image.extra_mut().align = Some(opt_val.parse()?),
            "target" => image.extra_mut().target = Some(opt_val.parse()?),
            name => bail!("Unknown Image option {name}"),
        }
    }
    Ok(image)
}

fn parse_scale(pair: &Pair<Rule>) -> Result<u8, Error> {
    use pest::error::{Error, ErrorVariant};

    let input = pair.as_str().trim();
    let input = if let Some(percentage) = input.strip_suffix('%') {
        percentage.trim_end()
    } else {
        input
    };
    Ok(input.parse().map_err(|e: std::num::ParseIntError| {
        let var: ErrorVariant<Rule> = ErrorVariant::CustomError {
            message: e.to_string(),
        };
        Error::new_from_span(var, pair.as_span())
    })?)
}

fn convert_admonition_gen(pair: Pair<Rule>) -> document_tree::element_categories::BodyElement {
    let mut iter = pair.into_inner();
    let typ = iter.next().unwrap().as_str();
    // TODO: in reality it contains body elements.
    let children: Vec<c::BodyElement> = iter
        .map(|p| e::Paragraph::with_children(vec![p.as_str().into()]).into())
        .collect();
    match typ {
        "attention" => e::Attention::with_children(children).into(),
        "hint" => e::Hint::with_children(children).into(),
        "note" => e::Note::with_children(children).into(),
        "caution" => e::Caution::with_children(children).into(),
        "danger" => e::Danger::with_children(children).into(),
        "error" => e::Error::with_children(children).into(),
        "important" => e::Important::with_children(children).into(),
        "tip" => e::Tip::with_children(children).into(),
        "warning" => e::Warning::with_children(children).into(),
        typ => panic!("Unknown admontion type {typ}!"),
    }
}

fn convert_bullet_list(pair: Pair<Rule>) -> Result<e::BulletList, Error> {
    Ok(e::BulletList::with_children(
        pair.into_inner()
            .map(convert_bullet_item)
            .collect::<Result<_, _>>()?,
    ))
}

fn convert_bullet_item(pair: Pair<Rule>) -> Result<e::ListItem, Error> {
    let mut iter = pair.into_inner();
    let mut children: Vec<c::BodyElement> = vec![convert_paragraph(iter.next().unwrap())?.into()];
    for p in iter {
        children.push(convert_body_elem(p)?);
    }
    Ok(e::ListItem::with_children(children))
}

fn convert_block_quote(pair: Pair<Rule>) -> Result<e::BlockQuote, Error> {
    Ok(e::BlockQuote::with_children(
        pair.into_inner()
            .map(convert_block_quote_inner)
            .collect::<Result<_, _>>()?,
    ))
}

fn convert_block_quote_directive(pair: Pair<Rule>) -> Result<e::BlockQuote, Error> {
    let mut iter = pair.into_inner();
    let typ = iter.next().unwrap().as_str();
    let children: Vec<c::SubBlockQuote> = iter
        .map(convert_block_quote_inner)
        .collect::<Result<_, _>>()?;
    let mut bq = e::BlockQuote::with_children(children);
    bq.classes_mut().push(typ.to_owned());
    Ok(bq)
}

fn convert_block_quote_inner(pair: Pair<Rule>) -> Result<c::SubBlockQuote, Error> {
    Ok(if pair.as_rule() == Rule::attribution {
        e::Attribution::with_children(convert_inlines(pair)?).into()
    } else {
        convert_body_elem(pair)?.into()
    })
}

fn convert_literal_block(pair: Pair<Rule>) -> e::LiteralBlock {
    convert_literal_lines(pair.into_inner().next().unwrap())
}

fn convert_literal_lines(pair: Pair<Rule>) -> e::LiteralBlock {
    let children = pair
        .into_inner()
        .map(|l| {
            match l.as_rule() {
                Rule::literal_line => l.as_str(),
                Rule::literal_line_blank => "\n",
                _ => unreachable!(),
            }
            .into()
        })
        .collect();
    e::LiteralBlock::with_children(children)
}

fn convert_code_directive(pair: Pair<Rule>) -> e::LiteralBlock {
    let mut iter = pair.into_inner();
    let (lang, code) = match (iter.next().unwrap(), iter.next()) {
        (lang, Some(code)) => (Some(lang), code),
        (code, None) => (None, code),
    };
    let mut code_block = convert_literal_lines(code);
    code_block.classes_mut().push("code".to_owned());
    if let Some(lang) = lang {
        code_block.classes_mut().push(lang.as_str().to_owned());
    }
    code_block
}

fn convert_raw_directive(pair: Pair<Rule>) -> e::Raw {
    let mut iter = pair.into_inner();
    let format = iter.next().unwrap();
    let block = iter.next().unwrap();
    let children = block
        .into_inner()
        .map(|l| {
            match l.as_rule() {
                Rule::raw_line => l.as_str(),
                Rule::raw_line_blank => "\n",
                _ => unreachable!(),
            }
            .into()
        })
        .collect();
    let mut raw_block = e::Raw::with_children(children);
    raw_block
        .extra_mut()
        .format
        .push(at::NameToken(format.as_str().to_owned()));
    raw_block
}

fn convert_comment(pair: Pair<Rule>) -> e::Comment {
    let lines = pair
        .into_inner()
        .map(|l| {
            match l.as_rule() {
                Rule::comment_line_blank => "\n",
                Rule::comment_line => l.as_str(),
                _ => unreachable!(),
            }
            .into()
        })
        .collect();
    e::Comment::with_children(lines)
}

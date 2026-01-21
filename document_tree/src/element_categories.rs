use std::fmt::{self, Debug, Formatter};

use schemars::JsonSchema;
use serde_derive::Serialize;

#[allow(clippy::wildcard_imports)]
use crate::elements::*;

pub trait HasChildren<C> {
    fn with_children(children: Vec<C>) -> Self;
    fn children(&self) -> &Vec<C>;
    fn children_mut(&mut self) -> &mut Vec<C>;
    fn append_child<R: Into<C>>(&mut self, child: R) {
        self.children_mut().push(child.into());
    }
    fn append_children<R: Into<C> + Clone>(&mut self, more: &[R]) {
        let children = self.children_mut();
        children.reserve(more.len());
        for child in more {
            children.push(child.clone().into());
        }
    }
}

macro_rules! impl_into {
    ([ $( (($subcat:ident :: $entry:ident), $supcat:ident), )+ ]) => {
        $( impl_into!($subcat::$entry => $supcat); )+
    };
    ($subcat:ident :: $entry:ident => $supcat:ident ) => {
        impl From<$entry> for $supcat {
            fn from(inner: $entry) -> Self {
                $supcat::$subcat(Box::new(inner.into()))
            }
        }
    };
}

macro_rules! synonymous_enum {
    ( $subcat:ident : $($supcat:ident),+ ; $midcat:ident : $supsupcat:ident {
        $($(#[$attr:meta])? $entry:ident),+ $(,)*
    } ) => {
        synonymous_enum!($subcat : $( $supcat ),+ , $midcat { $($(#[$attr])? $entry,)+ });
        $( impl_into!($midcat::$entry => $supsupcat); )+
    };
    ( $subcat:ident : $($supcat:ident),+ {
        $($(#[$attr:meta])? $entry:ident),+ $(,)*
    } ) => {
        synonymous_enum!($subcat { $($(#[$attr])? $entry,)* });
        cartesian!(impl_into, [ $( ($subcat::$entry) ),+ ], [ $($supcat),+ ]);
    };
    ( $name:ident {
        $($(#[$attr:meta])? $entry:ident),+ $(,)*
    } ) => {
        #[derive(Clone, PartialEq, Serialize, JsonSchema)]
        #[serde(tag = "type")]
        #[schemars(_unstable_ref_variants)]
        pub enum $name { $(
            $(#[$attr])?
            $entry(Box<$entry>),
        )* }

        impl Debug for $name {
            fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
                match *self {
                    $( $name::$entry(ref inner) => inner.fmt(fmt), )*
                }
            }
        }

        $( impl From<$entry> for $name {
            fn from(inner: $entry) -> Self {
                $name::$entry(Box::new(inner))
            }
        } )*
    };
}

synonymous_enum!(StructuralSubElement {
    Title,
    Subtitle,
    Decoration,
    Docinfo,
    #[serde(untagged)]
    SubStructure
});
synonymous_enum!(SubStructure: StructuralSubElement {
    Topic, Sidebar, Transition, Section, #[serde(untagged)] BodyElement
});
synonymous_enum!(BodyElement: SubTopic, SubSidebar, SubBlockQuote, SubFootnote, SubFigure; SubStructure: StructuralSubElement {
    //Simple
    Paragraph, LiteralBlock, DoctestBlock, MathBlock, Rubric, SubstitutionDefinition, Comment, Pending, Target, Raw, Image,
    //Compound
    Compound, Container,
    BulletList, EnumeratedList, DefinitionList, FieldList, OptionList,
    LineBlock, BlockQuote, Admonition, Attention, Hint, Note, Caution, Danger, Error, Important, Tip, Warning, Footnote, Citation, SystemMessage, Figure, Table
});

impl<'a> TryFrom<&'a StructuralSubElement> for &'a BodyElement {
    type Error = ();

    fn try_from(value: &'a StructuralSubElement) -> Result<Self, ()> {
        match value {
            StructuralSubElement::SubStructure(s) => s.as_ref().try_into(),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a SubStructure> for &'a BodyElement {
    type Error = ();

    fn try_from(value: &'a SubStructure) -> Result<Self, ()> {
        match value {
            SubStructure::BodyElement(s) => Ok(s.as_ref()),
            _ => Err(()),
        }
    }
}

synonymous_enum!(BibliographicElement {
    Authors,
    // author info, contained in Authors above:
    Author,
    Organization,
    Address,
    Contact,
    // other:
    Version,
    Revision,
    Status,
    Date,
    Copyright,
    Field
});

synonymous_enum!(TextOrInlineElement {
    String,
    Emphasis,
    Strong,
    Literal,
    Reference,
    FootnoteReference,
    CitationReference,
    SubstitutionReference,
    TitleReference,
    Abbreviation,
    Acronym,
    Superscript,
    Subscript,
    Inline,
    Problematic,
    Generated,
    Math,
    //also have non-inline versions. Inline image is no figure child, inline target has content
    TargetInline,
    RawInline,
    ImageInline
});

//--------------\\
//Content Models\\
//--------------\\

synonymous_enum!(AuthorInfo {
    Author,
    Organization,
    Address,
    Contact
});
synonymous_enum!(DecorationElement { Header, Footer });
synonymous_enum!(SubTopic { Title, BodyElement });
synonymous_enum!(SubSidebar {
    Topic,
    Title,
    Subtitle,
    BodyElement
});
synonymous_enum!(SubDLItem {
    Term,
    Classifier,
    Definition
});
synonymous_enum!(SubField {
    FieldName,
    FieldBody
});
synonymous_enum!(SubOptionListItem {
    OptionGroup,
    Description
});
synonymous_enum!(SubOption {
    OptionString,
    OptionArgument
});
synonymous_enum!(SubLineBlock { LineBlock, Line });
synonymous_enum!(SubBlockQuote {
    Attribution,
    BodyElement
});
synonymous_enum!(SubFootnote { Label, BodyElement });
synonymous_enum!(SubFigure {
    Caption,
    Legend,
    BodyElement
});
synonymous_enum!(SubTable { Title, TableGroup });
synonymous_enum!(SubTableGroup {
    TableColspec,
    TableHead,
    TableBody
});

// indirect conversions
impl From<SubTopic> for SubSidebar {
    fn from(inner: SubTopic) -> Self {
        match inner {
            SubTopic::Title(e) => (*e).into(),
            SubTopic::BodyElement(e) => (*e).into(),
        }
    }
}

impl From<SubTopic> for StructuralSubElement {
    fn from(inner: SubTopic) -> Self {
        match inner {
            SubTopic::Title(e) => (*e).into(),
            SubTopic::BodyElement(e) => (*e).into(),
        }
    }
}

impl From<SubSidebar> for StructuralSubElement {
    fn from(inner: SubSidebar) -> Self {
        match inner {
            SubSidebar::Topic(e) => (*e).into(),
            SubSidebar::Title(e) => (*e).into(),
            SubSidebar::Subtitle(e) => (*e).into(),
            SubSidebar::BodyElement(e) => (*e).into(),
        }
    }
}

impl From<AuthorInfo> for BibliographicElement {
    fn from(inner: AuthorInfo) -> Self {
        match inner {
            AuthorInfo::Author(e) => (*e).into(),
            AuthorInfo::Organization(e) => (*e).into(),
            AuthorInfo::Address(e) => (*e).into(),
            AuthorInfo::Contact(e) => (*e).into(),
        }
    }
}

#[cfg(test)]
mod conversion_tests {
    use super::*;
    use std::default::Default;

    #[test]
    fn basic() {
        let _: BodyElement = Paragraph::default().into();
    }

    #[test]
    fn more() {
        let _: SubStructure = Paragraph::default().into();
    }

    #[test]
    fn even_more() {
        let _: StructuralSubElement = Paragraph::default().into();
    }

    #[test]
    fn super_() {
        let be: BodyElement = Paragraph::default().into();
        let _: StructuralSubElement = be.into();
    }
}

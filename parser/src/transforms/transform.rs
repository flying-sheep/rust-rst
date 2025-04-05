use std::iter::once;

use document_tree::HasChildren;
use document_tree::element_categories as c;
use document_tree::elements as e;

/// Helper trait for [`Transform::transform_children`] while [Fn traits] are not stable yet.
///
/// See <https://users.rust-lang.org/t/127846>.
///
/// [Fn traits]: https://github.com/rust-lang/rust/issues/29625
pub trait IteratorMaker<This, C>: FnMut(This, C) -> Self::Iter {
    type Iter: Iterator<Item = C>;
}

impl<F, This, C, Iter> IteratorMaker<This, C> for F
where
    F: ?Sized + FnMut(This, C) -> Iter,
    Iter: Iterator<Item = C>,
{
    type Iter = Iter;
}

#[inline]
fn box_iter<'a, I>(i: impl Iterator<Item = I> + 'a) -> Box<dyn Iterator<Item = I> + 'a> {
    Box::new(i)
}

/// Transform a document tree.
///
/// Override individual methods to modify the document tree.
/// By default, every method transforms an element’s children (if applicable) and then returns the element.
pub trait Transform {
    /// Transform children of an element.
    ///
    /// Can be used in other `transform_<element>` methods to recurse, e.g.:
    ///
    /// ```rust
    /// fn transform_header(&mut self, mut e: e::Header) -> impl Iterator<Item = c::DecorationElement> {
    ///     self.transform_children(&mut e, Self::transform_body_element);
    ///     std::iter::once(e.into())
    /// }
    /// ```
    fn transform_children<C, E>(
        &mut self,
        e: &mut E,
        mut meth: impl for<'a> IteratorMaker<&'a mut Self, C>,
    ) where
        E: HasChildren<C>,
    {
        let mut new = Vec::new();
        for c in e.children_mut().drain(..) {
            new.extend(meth(self, c));
        }
        e.children_mut().extend(new);
    }

    /// Transform a whole document tree.
    #[must_use]
    fn transform(&mut self, mut d: e::Document) -> e::Document {
        self.transform_children(&mut d, Self::transform_structural_sub_element);
        d
    }

    ////////////////
    // categories //
    ////////////////

    #[must_use]
    fn transform_structural_sub_element(
        &mut self,
        c: c::StructuralSubElement,
    ) -> impl Iterator<Item = c::StructuralSubElement> {
        use c::StructuralSubElement as S;
        match c {
            S::Title(e) => box_iter(self.transform_title(*e).map(Into::into)),
            S::Subtitle(e) => box_iter(self.transform_subtitle(*e).map(S::from)),
            S::Decoration(e) => box_iter(self.transform_decoration(*e)),
            S::Docinfo(e) => box_iter(self.transform_docinfo(*e)),
            S::SubStructure(e) => box_iter(self.transform_substructure(*e).map(S::from)),
        }
    }
    #[must_use]
    fn transform_substructure(
        &mut self,
        c: c::SubStructure,
    ) -> impl Iterator<Item = c::SubStructure> {
        use c::SubStructure as S;
        match c {
            S::Topic(e) => box_iter(self.transform_topic(*e).map(Into::into)),
            S::Sidebar(e) => box_iter(self.transform_sidebar(*e)),
            S::Transition(e) => box_iter(self.transform_transition(*e)),
            S::Section(e) => box_iter(self.transform_section(*e)),
            S::BodyElement(e) => box_iter(self.transform_body_element(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn transform_body_element(
        &mut self,
        c: c::BodyElement,
    ) -> impl Iterator<Item = c::BodyElement> {
        use c::BodyElement as B;
        match c {
            B::Paragraph(e) => box_iter(self.transform_paragraph(*e)),
            B::LiteralBlock(e) => box_iter(self.transform_literal_block(*e)),
            B::DoctestBlock(e) => box_iter(self.transform_doctest_block(*e)),
            B::MathBlock(e) => box_iter(self.transform_math_block(*e)),
            B::Rubric(e) => box_iter(self.transform_rubric(*e)),
            B::SubstitutionDefinition(e) => box_iter(self.transform_substitution_definition(*e)),
            B::Comment(e) => box_iter(self.transform_comment(*e)),
            B::Pending(e) => box_iter(self.transform_pending(*e)),
            B::Target(e) => box_iter(self.transform_target(*e)),
            B::Raw(e) => box_iter(self.transform_raw(*e)),
            B::Image(e) => box_iter(self.transform_image(*e)),
            B::Compound(e) => box_iter(self.transform_compound(*e)),
            B::Container(e) => box_iter(self.transform_container(*e)),
            B::BulletList(e) => box_iter(self.transform_bullet_list(*e)),
            B::EnumeratedList(e) => box_iter(self.transform_enumerated_list(*e)),
            B::DefinitionList(e) => box_iter(self.transform_definition_list(*e)),
            B::FieldList(e) => box_iter(self.transform_field_list(*e)),
            B::OptionList(e) => box_iter(self.transform_option_list(*e)),
            B::LineBlock(e) => box_iter(self.transform_line_block(*e).map(Into::into)),
            B::BlockQuote(e) => box_iter(self.transform_block_quote(*e)),
            B::Admonition(e) => box_iter(self.transform_admonition(*e)),
            B::Attention(e) => box_iter(self.transform_attention(*e)),
            B::Hint(e) => box_iter(self.transform_hint(*e)),
            B::Note(e) => box_iter(self.transform_note(*e)),
            B::Caution(e) => box_iter(self.transform_caution(*e)),
            B::Danger(e) => box_iter(self.transform_danger(*e)),
            B::Error(e) => box_iter(self.transform_error(*e)),
            B::Important(e) => box_iter(self.transform_important(*e)),
            B::Tip(e) => box_iter(self.transform_tip(*e)),
            B::Warning(e) => box_iter(self.transform_warning(*e)),
            B::Footnote(e) => box_iter(self.transform_footnote(*e)),
            B::Citation(e) => box_iter(self.transform_citation(*e)),
            B::SystemMessage(e) => box_iter(self.transform_system_message(*e)),
            B::Figure(e) => box_iter(self.transform_figure(*e)),
            B::Table(e) => box_iter(self.transform_table(*e)),
        }
    }
    #[must_use]
    fn transform_bibliographic_element(
        &mut self,
        c: c::BibliographicElement,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        use c::BibliographicElement as B;
        match c {
            B::Authors(e) => box_iter(self.transform_authors(*e)),
            B::Author(e) => box_iter(self.transform_author(*e).map(Into::into)),
            B::Organization(e) => box_iter(self.transform_organization(*e).map(Into::into)),
            B::Address(e) => box_iter(self.transform_address(*e).map(Into::into)),
            B::Contact(e) => box_iter(self.transform_contact(*e).map(Into::into)),
            B::Version(e) => box_iter(self.transform_version(*e)),
            B::Revision(e) => box_iter(self.transform_revision(*e)),
            B::Status(e) => box_iter(self.transform_status(*e)),
            B::Date(e) => box_iter(self.transform_date(*e)),
            B::Copyright(e) => box_iter(self.transform_copyright(*e)),
            B::Field(e) => box_iter(self.transform_field(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn transform_text_or_inline_element(
        &mut self,
        c: c::TextOrInlineElement,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        use c::TextOrInlineElement as T;
        match c {
            T::String(e) => box_iter(self.transform_string(*e).map(Into::into)),
            T::Emphasis(e) => box_iter(self.transform_emphasis(*e)),
            T::Strong(e) => box_iter(self.transform_strong(*e)),
            T::Literal(e) => box_iter(self.transform_literal(*e)),
            T::Reference(e) => box_iter(self.transform_reference(*e)),
            T::FootnoteReference(e) => box_iter(self.transform_footnote_reference(*e)),
            T::CitationReference(e) => box_iter(self.transform_citation_reference(*e)),
            T::SubstitutionReference(e) => box_iter(self.transform_substitution_reference(*e)),
            T::TitleReference(e) => box_iter(self.transform_title_reference(*e)),
            T::Abbreviation(e) => box_iter(self.transform_abbreviation(*e)),
            T::Acronym(e) => box_iter(self.transform_acronym(*e)),
            T::Superscript(e) => box_iter(self.transform_superscript(*e)),
            T::Subscript(e) => box_iter(self.transform_subscript(*e)),
            T::Inline(e) => box_iter(self.transform_inline(*e)),
            T::Problematic(e) => box_iter(self.transform_problematic(*e)),
            T::Generated(e) => box_iter(self.transform_generated(*e)),
            T::Math(e) => box_iter(self.transform_math(*e)),
            T::TargetInline(e) => box_iter(self.transform_target_inline(*e)),
            T::RawInline(e) => box_iter(self.transform_raw_inline(*e)),
            T::ImageInline(e) => box_iter(self.transform_image_inline(*e)),
        }
    }
    #[must_use]
    fn transform_author_info(&mut self, c: c::AuthorInfo) -> impl Iterator<Item = c::AuthorInfo> {
        use c::AuthorInfo as A;
        match c {
            A::Author(e) => box_iter(self.transform_author(*e)),
            A::Organization(e) => box_iter(self.transform_organization(*e)),
            A::Address(e) => box_iter(self.transform_address(*e)),
            A::Contact(e) => box_iter(self.transform_contact(*e)),
        }
    }
    #[must_use]
    fn transform_decoration_element(
        &mut self,
        c: c::DecorationElement,
    ) -> impl Iterator<Item = c::DecorationElement> {
        use c::DecorationElement as D;
        match c {
            D::Header(e) => box_iter(self.transform_header(*e)),
            D::Footer(e) => box_iter(self.transform_footer(*e)),
        }
    }
    #[must_use]
    fn transform_sub_topic(&mut self, c: c::SubTopic) -> impl Iterator<Item = c::SubTopic> {
        use c::SubTopic as S;
        match c {
            S::Title(e) => box_iter(self.transform_title(*e).map(Into::into)),
            S::BodyElement(e) => box_iter(self.transform_body_element(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn transform_sub_sidebar(&mut self, c: c::SubSidebar) -> impl Iterator<Item = c::SubSidebar> {
        use c::SubSidebar as S;
        match c {
            S::Topic(e) => box_iter(self.transform_topic(*e).map(Into::into)),
            S::Title(e) => box_iter(self.transform_title(*e).map(Into::into)),
            S::Subtitle(e) => box_iter(self.transform_subtitle(*e)),
            S::BodyElement(e) => box_iter(self.transform_body_element(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn transform_sub_dl_item(&mut self, c: c::SubDLItem) -> impl Iterator<Item = c::SubDLItem> {
        use c::SubDLItem as S;
        match c {
            S::Term(e) => box_iter(self.transform_term(*e)),
            S::Classifier(e) => box_iter(self.transform_classifier(*e)),
            S::Definition(e) => box_iter(self.transform_definition(*e)),
        }
    }
    #[must_use]
    fn transform_sub_field(&mut self, c: c::SubField) -> impl Iterator<Item = c::SubField> {
        use c::SubField as S;
        match c {
            S::FieldName(e) => box_iter(self.transform_field_name(*e)),
            S::FieldBody(e) => box_iter(self.transform_field_body(*e)),
        }
    }
    #[must_use]
    fn transform_sub_option_list_item(
        &mut self,
        c: c::SubOptionListItem,
    ) -> impl Iterator<Item = c::SubOptionListItem> {
        use c::SubOptionListItem as S;
        match c {
            S::OptionGroup(e) => box_iter(self.transform_option_group(*e)),
            S::Description(e) => box_iter(self.transform_description(*e)),
        }
    }
    #[must_use]
    fn transform_sub_option(&mut self, c: c::SubOption) -> impl Iterator<Item = c::SubOption> {
        use c::SubOption as S;
        match c {
            S::OptionString(e) => box_iter(self.transform_option_string(*e)),
            S::OptionArgument(e) => box_iter(self.transform_option_argument(*e)),
        }
    }
    #[must_use]
    fn transform_sub_line_block(
        &mut self,
        c: c::SubLineBlock,
    ) -> impl Iterator<Item = c::SubLineBlock> {
        use c::SubLineBlock as S;
        match c {
            S::LineBlock(e) => box_iter(self.transform_line_block(*e).map(Into::into)),
            S::Line(e) => box_iter(self.transform_line(*e)),
        }
    }
    #[must_use]
    fn transform_sub_block_quote(
        &mut self,
        c: c::SubBlockQuote,
    ) -> impl Iterator<Item = c::SubBlockQuote> {
        use c::SubBlockQuote as S;
        match c {
            S::Attribution(e) => box_iter(self.transform_attribution(*e)),
            S::BodyElement(e) => box_iter(self.transform_body_element(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn transform_sub_footnote(
        &mut self,
        c: c::SubFootnote,
    ) -> impl Iterator<Item = c::SubFootnote> {
        use c::SubFootnote as S;
        match c {
            S::Label(e) => box_iter(self.transform_label(*e)),
            S::BodyElement(e) => box_iter(self.transform_body_element(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn transform_sub_figure(&mut self, c: c::SubFigure) -> impl Iterator<Item = c::SubFigure> {
        use c::SubFigure as S;
        match c {
            S::Caption(e) => box_iter(self.transform_caption(*e)),
            S::Legend(e) => box_iter(self.transform_legend(*e)),
            S::BodyElement(e) => box_iter(self.transform_body_element(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn transform_sub_table(&mut self, c: c::SubTable) -> impl Iterator<Item = c::SubTable> {
        use c::SubTable as S;
        match c {
            S::Title(e) => box_iter(self.transform_title(*e).map(Into::into)),
            S::TableGroup(e) => box_iter(self.transform_table_group(*e)),
        }
    }
    #[must_use]
    fn transform_sub_table_group(
        &mut self,
        c: c::SubTableGroup,
    ) -> impl Iterator<Item = c::SubTableGroup> {
        use c::SubTableGroup as S;
        match c {
            S::TableColspec(e) => box_iter(self.transform_table_colspec(*e)),
            S::TableHead(e) => box_iter(self.transform_table_head(*e)),
            S::TableBody(e) => box_iter(self.transform_table_body(*e)),
        }
    }

    //////////////
    // elements //
    //////////////

    //structual elements
    #[must_use]
    fn transform_section(&mut self, mut e: e::Section) -> impl Iterator<Item = c::SubStructure> {
        self.transform_children(&mut e, Self::transform_structural_sub_element);
        once(e.into())
    }
    #[must_use]
    // TODO: introduce and return category for topic|bodyelement
    fn transform_topic(&mut self, mut e: e::Topic) -> impl Iterator<Item = e::Topic> {
        self.transform_children(&mut e, Self::transform_sub_topic);
        once(e)
    }
    #[must_use]
    fn transform_sidebar(&mut self, mut e: e::Sidebar) -> impl Iterator<Item = c::SubStructure> {
        self.transform_children(&mut e, Self::transform_sub_sidebar);
        once(e.into())
    }

    //structural subelements
    #[must_use]
    fn transform_title(&mut self, mut e: e::Title) -> impl Iterator<Item = e::Title> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e)
    }
    #[must_use]
    fn transform_subtitle(&mut self, mut e: e::Subtitle) -> impl Iterator<Item = c::SubSidebar> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_decoration(
        &mut self,
        mut e: e::Decoration,
    ) -> impl Iterator<Item = c::StructuralSubElement> {
        self.transform_children(&mut e, Self::transform_decoration_element);
        once(e.into())
    }
    #[must_use]
    fn transform_docinfo(
        &mut self,
        mut e: e::Docinfo,
    ) -> impl Iterator<Item = c::StructuralSubElement> {
        self.transform_children(&mut e, Self::transform_bibliographic_element);
        once(e.into())
    }
    #[must_use]
    fn transform_transition(&mut self, e: e::Transition) -> impl Iterator<Item = c::SubStructure> {
        once(e.into())
    }

    //bibliographic elements
    #[must_use]
    fn transform_authors(
        &mut self,
        mut e: e::Authors,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        self.transform_children(&mut e, Self::transform_author_info);
        once(e.into())
    }
    #[must_use]
    fn transform_author(&mut self, mut e: e::Author) -> impl Iterator<Item = c::AuthorInfo> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_organization(
        &mut self,
        mut e: e::Organization,
    ) -> impl Iterator<Item = c::AuthorInfo> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_address(&mut self, mut e: e::Address) -> impl Iterator<Item = c::AuthorInfo> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_contact(&mut self, mut e: e::Contact) -> impl Iterator<Item = c::AuthorInfo> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_version(
        &mut self,
        mut e: e::Version,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_revision(
        &mut self,
        mut e: e::Revision,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_status(
        &mut self,
        mut e: e::Status,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_date(&mut self, mut e: e::Date) -> impl Iterator<Item = c::BibliographicElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_copyright(
        &mut self,
        mut e: e::Copyright,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_field(&mut self, mut e: e::Field) -> impl Iterator<Item = e::Field> {
        self.transform_children(&mut e, Self::transform_sub_field);
        once(e)
    }

    //decoration elements
    #[must_use]
    fn transform_header(&mut self, mut e: e::Header) -> impl Iterator<Item = c::DecorationElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_footer(&mut self, mut e: e::Footer) -> impl Iterator<Item = c::DecorationElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }

    //simple body elements
    #[must_use]
    fn transform_paragraph(&mut self, mut e: e::Paragraph) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_literal_block(
        &mut self,
        mut e: e::LiteralBlock,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_doctest_block(
        &mut self,
        mut e: e::DoctestBlock,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_math_block(
        &mut self,
        mut e: e::MathBlock,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_string);
        once(e.into())
    }
    #[must_use]
    fn transform_rubric(&mut self, mut e: e::Rubric) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_substitution_definition(
        &mut self,
        mut e: e::SubstitutionDefinition,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_comment(&mut self, mut e: e::Comment) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_pending(&mut self, e: e::Pending) -> impl Iterator<Item = c::BodyElement> {
        once(e.into())
    }
    #[must_use]
    fn transform_target(&mut self, e: e::Target) -> impl Iterator<Item = c::BodyElement> {
        once(e.into())
    }
    #[must_use]
    fn transform_raw(&mut self, mut e: e::Raw) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_string);
        once(e.into())
    }
    #[must_use]
    fn transform_image(&mut self, e: e::Image) -> impl Iterator<Item = c::BodyElement> {
        once(e.into())
    }

    //compound body elements
    #[must_use]
    fn transform_compound(&mut self, mut e: e::Compound) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_container(&mut self, mut e: e::Container) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_bullet_list(
        &mut self,
        mut e: e::BulletList,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_list_item);
        once(e.into())
    }
    #[must_use]
    fn transform_enumerated_list(
        &mut self,
        mut e: e::EnumeratedList,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_list_item);
        once(e.into())
    }
    #[must_use]
    fn transform_definition_list(
        &mut self,
        mut e: e::DefinitionList,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_definition_list_item);
        once(e.into())
    }
    #[must_use]
    fn transform_field_list(
        &mut self,
        mut e: e::FieldList,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_field);
        once(e.into())
    }
    #[must_use]
    fn transform_option_list(
        &mut self,
        mut e: e::OptionList,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_option_list_item);
        once(e.into())
    }
    #[must_use]
    fn transform_line_block(&mut self, mut e: e::LineBlock) -> impl Iterator<Item = e::LineBlock> {
        self.transform_children(&mut e, Self::transform_sub_line_block);
        once(e)
    }
    #[must_use]
    fn transform_block_quote(
        &mut self,
        mut e: e::BlockQuote,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_sub_block_quote);
        once(e.into())
    }
    #[must_use]
    fn transform_admonition(
        &mut self,
        mut e: e::Admonition,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_sub_topic);
        once(e.into())
    }
    #[must_use]
    fn transform_attention(&mut self, mut e: e::Attention) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_hint(&mut self, mut e: e::Hint) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_note(&mut self, mut e: e::Note) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_caution(&mut self, mut e: e::Caution) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_danger(&mut self, mut e: e::Danger) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_error(&mut self, mut e: e::Error) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_important(&mut self, mut e: e::Important) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_tip(&mut self, mut e: e::Tip) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_warning(&mut self, mut e: e::Warning) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_footnote(&mut self, mut e: e::Footnote) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_sub_footnote);
        once(e.into())
    }
    #[must_use]
    fn transform_citation(&mut self, mut e: e::Citation) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_sub_footnote);
        once(e.into())
    }
    #[must_use]
    fn transform_system_message(
        &mut self,
        mut e: e::SystemMessage,
    ) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_figure(&mut self, mut e: e::Figure) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_sub_figure);
        once(e.into())
    }
    #[must_use]
    fn transform_table(&mut self, mut e: e::Table) -> impl Iterator<Item = c::BodyElement> {
        self.transform_children(&mut e, Self::transform_sub_table);
        once(e.into())
    }

    //table elements
    #[must_use]
    fn transform_table_group(&mut self, mut e: e::TableGroup) -> impl Iterator<Item = c::SubTable> {
        self.transform_children(&mut e, Self::transform_sub_table_group);
        once(e.into())
    }
    #[must_use]
    fn transform_table_head(
        &mut self,
        mut e: e::TableHead,
    ) -> impl Iterator<Item = c::SubTableGroup> {
        self.transform_children(&mut e, Self::transform_table_row);
        once(e.into())
    }
    #[must_use]
    fn transform_table_body(
        &mut self,
        mut e: e::TableBody,
    ) -> impl Iterator<Item = c::SubTableGroup> {
        self.transform_children(&mut e, Self::transform_table_row);
        once(e.into())
    }
    #[must_use]
    fn transform_table_row(&mut self, mut e: e::TableRow) -> impl Iterator<Item = e::TableRow> {
        self.transform_children(&mut e, Self::transform_table_entry);
        once(e)
    }
    #[must_use]
    fn transform_table_entry(
        &mut self,
        mut e: e::TableEntry,
    ) -> impl Iterator<Item = e::TableEntry> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e)
    }
    #[must_use]
    fn transform_table_colspec(
        &mut self,
        e: e::TableColspec,
    ) -> impl Iterator<Item = c::SubTableGroup> {
        once(e.into())
    }

    //body sub elements
    #[must_use]
    fn transform_list_item(&mut self, mut e: e::ListItem) -> impl Iterator<Item = e::ListItem> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e)
    }
    #[must_use]
    fn transform_definition_list_item(
        &mut self,
        mut e: e::DefinitionListItem,
    ) -> impl Iterator<Item = e::DefinitionListItem> {
        self.transform_children(&mut e, Self::transform_sub_dl_item);
        once(e)
    }
    #[must_use]
    fn transform_term(&mut self, mut e: e::Term) -> impl Iterator<Item = c::SubDLItem> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_classifier(&mut self, mut e: e::Classifier) -> impl Iterator<Item = c::SubDLItem> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_definition(&mut self, mut e: e::Definition) -> impl Iterator<Item = c::SubDLItem> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_field_name(&mut self, mut e: e::FieldName) -> impl Iterator<Item = c::SubField> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_field_body(&mut self, mut e: e::FieldBody) -> impl Iterator<Item = c::SubField> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_option_list_item(
        &mut self,
        mut e: e::OptionListItem,
    ) -> impl Iterator<Item = e::OptionListItem> {
        self.transform_children(&mut e, Self::transform_sub_option_list_item);
        once(e)
    }
    #[must_use]
    fn transform_option_group(
        &mut self,
        mut e: e::OptionGroup,
    ) -> impl Iterator<Item = c::SubOptionListItem> {
        self.transform_children(&mut e, Self::transform_option);
        once(e.into())
    }
    #[must_use]
    fn transform_description(
        &mut self,
        mut e: e::Description,
    ) -> impl Iterator<Item = c::SubOptionListItem> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }
    #[must_use]
    fn transform_option(&mut self, mut e: e::Option_) -> impl Iterator<Item = e::Option_> {
        self.transform_children(&mut e, Self::transform_sub_option);
        once(e)
    }
    #[must_use]
    fn transform_option_string(
        &mut self,
        mut e: e::OptionString,
    ) -> impl Iterator<Item = c::SubOption> {
        self.transform_children(&mut e, Self::transform_string);
        once(e.into())
    }
    #[must_use]
    fn transform_option_argument(
        &mut self,
        mut e: e::OptionArgument,
    ) -> impl Iterator<Item = c::SubOption> {
        self.transform_children(&mut e, Self::transform_string);
        once(e.into())
    }
    #[must_use]
    fn transform_line(&mut self, mut e: e::Line) -> impl Iterator<Item = c::SubLineBlock> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_attribution(
        &mut self,
        mut e: e::Attribution,
    ) -> impl Iterator<Item = c::SubBlockQuote> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_label(&mut self, mut e: e::Label) -> impl Iterator<Item = c::SubFootnote> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_caption(&mut self, mut e: e::Caption) -> impl Iterator<Item = c::SubFigure> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_legend(&mut self, mut e: e::Legend) -> impl Iterator<Item = c::SubFigure> {
        self.transform_children(&mut e, Self::transform_body_element);
        once(e.into())
    }

    //inline elements
    #[must_use]
    fn transform_string(&mut self, e: String) -> impl Iterator<Item = String> {
        once(e)
    }
    #[must_use]
    fn transform_emphasis(
        &mut self,
        mut e: e::Emphasis,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_literal(
        &mut self,
        mut e: e::Literal,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_string);
        once(e.into())
    }
    #[must_use]
    fn transform_reference(
        &mut self,
        mut e: e::Reference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_strong(
        &mut self,
        mut e: e::Strong,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_footnote_reference(
        &mut self,
        mut e: e::FootnoteReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_citation_reference(
        &mut self,
        mut e: e::CitationReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_substitution_reference(
        &mut self,
        mut e: e::SubstitutionReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_title_reference(
        &mut self,
        mut e: e::TitleReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_abbreviation(
        &mut self,
        mut e: e::Abbreviation,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_acronym(
        &mut self,
        mut e: e::Acronym,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_superscript(
        &mut self,
        mut e: e::Superscript,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_subscript(
        &mut self,
        mut e: e::Subscript,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_inline(
        &mut self,
        mut e: e::Inline,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_problematic(
        &mut self,
        mut e: e::Problematic,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_generated(
        &mut self,
        mut e: e::Generated,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_text_or_inline_element);
        once(e.into())
    }
    #[must_use]
    fn transform_math(&mut self, mut e: e::Math) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_string);
        once(e.into())
    }
    #[must_use]
    fn transform_target_inline(
        &mut self,
        mut e: e::TargetInline,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_string);
        once(e.into())
    }
    #[must_use]
    fn transform_raw_inline(
        &mut self,
        mut e: e::RawInline,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        self.transform_children(&mut e, Self::transform_string);
        once(e.into())
    }
    #[must_use]
    fn transform_image_inline(
        &mut self,
        e: e::ImageInline,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        once(e.into())
    }
}

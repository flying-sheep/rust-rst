use std::iter::once;

use document_tree::HasChildren as _;
use document_tree::element_categories as c;
use document_tree::elements as e;

#[macro_export]
macro_rules! transform_children {
    ($e:ident, $self:ident . $m:ident) => {
        let mut new = Vec::new();
        for c in $e.children_mut().drain(..) {
            new.extend($self.$m(c));
        }
        $e.children_mut().extend(new);
    };
}

#[inline]
fn box_iter<'a, I>(i: impl Iterator<Item = I> + 'a) -> Box<dyn Iterator<Item = I> + 'a> {
    Box::new(i)
}

pub trait VisitMut {
    #[must_use]
    fn visit_mut(&mut self, mut d: e::Document) -> e::Document {
        transform_children!(d, self.visit_structural_sub_element_mut);
        d
    }

    ////////////////
    // categories //
    ////////////////

    #[must_use]
    fn visit_structural_sub_element_mut(
        &mut self,
        c: c::StructuralSubElement,
    ) -> impl Iterator<Item = c::StructuralSubElement> {
        use c::StructuralSubElement as S;
        match c {
            S::Title(e) => box_iter(self.visit_title_mut(*e).map(Into::into)),
            S::Subtitle(e) => box_iter(self.visit_subtitle_mut(*e).map(S::from)),
            S::Decoration(e) => box_iter(self.visit_decoration_mut(*e)),
            S::Docinfo(e) => box_iter(self.visit_docinfo_mut(*e)),
            S::SubStructure(e) => box_iter(self.visit_substructure_mut(*e).map(S::from)),
        }
    }
    #[must_use]
    fn visit_substructure_mut(
        &mut self,
        c: c::SubStructure,
    ) -> impl Iterator<Item = c::SubStructure> {
        use c::SubStructure as S;
        match c {
            S::Topic(e) => box_iter(self.visit_topic_mut(*e).map(Into::into)),
            S::Sidebar(e) => box_iter(self.visit_sidebar_mut(*e)),
            S::Transition(e) => box_iter(self.visit_transition_mut(*e)),
            S::Section(e) => box_iter(self.visit_section_mut(*e)),
            S::BodyElement(e) => box_iter(self.visit_body_element_mut(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn visit_body_element_mut(
        &mut self,
        c: c::BodyElement,
    ) -> impl Iterator<Item = c::BodyElement> {
        use c::BodyElement as B;
        match c {
            B::Paragraph(e) => box_iter(self.visit_paragraph_mut(*e)),
            B::LiteralBlock(e) => box_iter(self.visit_literal_block_mut(*e)),
            B::DoctestBlock(e) => box_iter(self.visit_doctest_block_mut(*e)),
            B::MathBlock(e) => box_iter(self.visit_math_block_mut(*e)),
            B::Rubric(e) => box_iter(self.visit_rubric_mut(*e)),
            B::SubstitutionDefinition(e) => box_iter(self.visit_substitution_definition_mut(*e)),
            B::Comment(e) => box_iter(self.visit_comment_mut(*e)),
            B::Pending(e) => box_iter(self.visit_pending_mut(*e)),
            B::Target(e) => box_iter(self.visit_target_mut(*e)),
            B::Raw(e) => box_iter(self.visit_raw_mut(*e)),
            B::Image(e) => box_iter(self.visit_image_mut(*e)),
            B::Compound(e) => box_iter(self.visit_compound_mut(*e)),
            B::Container(e) => box_iter(self.visit_container_mut(*e)),
            B::BulletList(e) => box_iter(self.visit_bullet_list_mut(*e)),
            B::EnumeratedList(e) => box_iter(self.visit_enumerated_list_mut(*e)),
            B::DefinitionList(e) => box_iter(self.visit_definition_list_mut(*e)),
            B::FieldList(e) => box_iter(self.visit_field_list_mut(*e)),
            B::OptionList(e) => box_iter(self.visit_option_list_mut(*e)),
            B::LineBlock(e) => box_iter(self.visit_line_block_mut(*e).map(Into::into)),
            B::BlockQuote(e) => box_iter(self.visit_block_quote_mut(*e)),
            B::Admonition(e) => box_iter(self.visit_admonition_mut(*e)),
            B::Attention(e) => box_iter(self.visit_attention_mut(*e)),
            B::Hint(e) => box_iter(self.visit_hint_mut(*e)),
            B::Note(e) => box_iter(self.visit_note_mut(*e)),
            B::Caution(e) => box_iter(self.visit_caution_mut(*e)),
            B::Danger(e) => box_iter(self.visit_danger_mut(*e)),
            B::Error(e) => box_iter(self.visit_error_mut(*e)),
            B::Important(e) => box_iter(self.visit_important_mut(*e)),
            B::Tip(e) => box_iter(self.visit_tip_mut(*e)),
            B::Warning(e) => box_iter(self.visit_warning_mut(*e)),
            B::Footnote(e) => box_iter(self.visit_footnote_mut(*e)),
            B::Citation(e) => box_iter(self.visit_citation_mut(*e)),
            B::SystemMessage(e) => box_iter(self.visit_system_message_mut(*e)),
            B::Figure(e) => box_iter(self.visit_figure_mut(*e)),
            B::Table(e) => box_iter(self.visit_table_mut(*e)),
        }
    }
    #[must_use]
    fn visit_bibliographic_element_mut(
        &mut self,
        c: c::BibliographicElement,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        use c::BibliographicElement as B;
        match c {
            B::Authors(e) => box_iter(self.visit_authors_mut(*e)),
            B::Author(e) => box_iter(self.visit_author_mut(*e).map(Into::into)),
            B::Organization(e) => box_iter(self.visit_organization_mut(*e).map(Into::into)),
            B::Address(e) => box_iter(self.visit_address_mut(*e).map(Into::into)),
            B::Contact(e) => box_iter(self.visit_contact_mut(*e).map(Into::into)),
            B::Version(e) => box_iter(self.visit_version_mut(*e)),
            B::Revision(e) => box_iter(self.visit_revision_mut(*e)),
            B::Status(e) => box_iter(self.visit_status_mut(*e)),
            B::Date(e) => box_iter(self.visit_date_mut(*e)),
            B::Copyright(e) => box_iter(self.visit_copyright_mut(*e)),
            B::Field(e) => box_iter(self.visit_field_mut(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn visit_text_or_inline_element_mut(
        &mut self,
        c: c::TextOrInlineElement,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        use c::TextOrInlineElement as T;
        match c {
            T::String(e) => box_iter(self.visit_string_mut(*e).map(Into::into)),
            T::Emphasis(e) => box_iter(self.visit_emphasis_mut(*e)),
            T::Strong(e) => box_iter(self.visit_strong_mut(*e)),
            T::Literal(e) => box_iter(self.visit_literal_mut(*e)),
            T::Reference(e) => box_iter(self.visit_reference_mut(*e)),
            T::FootnoteReference(e) => box_iter(self.visit_footnote_reference_mut(*e)),
            T::CitationReference(e) => box_iter(self.visit_citation_reference_mut(*e)),
            T::SubstitutionReference(e) => box_iter(self.visit_substitution_reference_mut(*e)),
            T::TitleReference(e) => box_iter(self.visit_title_reference_mut(*e)),
            T::Abbreviation(e) => box_iter(self.visit_abbreviation_mut(*e)),
            T::Acronym(e) => box_iter(self.visit_acronym_mut(*e)),
            T::Superscript(e) => box_iter(self.visit_superscript_mut(*e)),
            T::Subscript(e) => box_iter(self.visit_subscript_mut(*e)),
            T::Inline(e) => box_iter(self.visit_inline_mut(*e)),
            T::Problematic(e) => box_iter(self.visit_problematic_mut(*e)),
            T::Generated(e) => box_iter(self.visit_generated_mut(*e)),
            T::Math(e) => box_iter(self.visit_math_mut(*e)),
            T::TargetInline(e) => box_iter(self.visit_target_inline_mut(*e)),
            T::RawInline(e) => box_iter(self.visit_raw_inline_mut(*e)),
            T::ImageInline(e) => box_iter(self.visit_image_inline_mut(*e)),
        }
    }
    #[must_use]
    fn visit_author_info_mut(&mut self, c: c::AuthorInfo) -> impl Iterator<Item = c::AuthorInfo> {
        use c::AuthorInfo as A;
        match c {
            A::Author(e) => box_iter(self.visit_author_mut(*e)),
            A::Organization(e) => box_iter(self.visit_organization_mut(*e)),
            A::Address(e) => box_iter(self.visit_address_mut(*e)),
            A::Contact(e) => box_iter(self.visit_contact_mut(*e)),
        }
    }
    #[must_use]
    fn visit_decoration_element_mut(
        &mut self,
        c: c::DecorationElement,
    ) -> impl Iterator<Item = c::DecorationElement> {
        use c::DecorationElement as D;
        match c {
            D::Header(e) => box_iter(self.visit_header_mut(*e)),
            D::Footer(e) => box_iter(self.visit_footer_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_topic_mut(&mut self, c: c::SubTopic) -> impl Iterator<Item = c::SubTopic> {
        use c::SubTopic as S;
        match c {
            S::Title(e) => box_iter(self.visit_title_mut(*e).map(Into::into)),
            S::BodyElement(e) => box_iter(self.visit_body_element_mut(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn visit_sub_sidebar_mut(&mut self, c: c::SubSidebar) -> impl Iterator<Item = c::SubSidebar> {
        use c::SubSidebar as S;
        match c {
            S::Topic(e) => box_iter(self.visit_topic_mut(*e).map(Into::into)),
            S::Title(e) => box_iter(self.visit_title_mut(*e).map(Into::into)),
            S::Subtitle(e) => box_iter(self.visit_subtitle_mut(*e)),
            S::BodyElement(e) => box_iter(self.visit_body_element_mut(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn visit_sub_dl_item_mut(&mut self, c: c::SubDLItem) -> impl Iterator<Item = c::SubDLItem> {
        use c::SubDLItem as S;
        match c {
            S::Term(e) => box_iter(self.visit_term_mut(*e)),
            S::Classifier(e) => box_iter(self.visit_classifier_mut(*e)),
            S::Definition(e) => box_iter(self.visit_definition_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_field_mut(&mut self, c: c::SubField) -> impl Iterator<Item = c::SubField> {
        use c::SubField as S;
        match c {
            S::FieldName(e) => box_iter(self.visit_field_name_mut(*e)),
            S::FieldBody(e) => box_iter(self.visit_field_body_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_option_list_item_mut(
        &mut self,
        c: c::SubOptionListItem,
    ) -> impl Iterator<Item = c::SubOptionListItem> {
        use c::SubOptionListItem as S;
        match c {
            S::OptionGroup(e) => box_iter(self.visit_option_group_mut(*e)),
            S::Description(e) => box_iter(self.visit_description_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_option_mut(&mut self, c: c::SubOption) -> impl Iterator<Item = c::SubOption> {
        use c::SubOption as S;
        match c {
            S::OptionString(e) => box_iter(self.visit_option_string_mut(*e)),
            S::OptionArgument(e) => box_iter(self.visit_option_argument_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_line_block_mut(
        &mut self,
        c: c::SubLineBlock,
    ) -> impl Iterator<Item = c::SubLineBlock> {
        use c::SubLineBlock as S;
        match c {
            S::LineBlock(e) => box_iter(self.visit_line_block_mut(*e).map(Into::into)),
            S::Line(e) => box_iter(self.visit_line_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_block_quote_mut(
        &mut self,
        c: c::SubBlockQuote,
    ) -> impl Iterator<Item = c::SubBlockQuote> {
        use c::SubBlockQuote as S;
        match c {
            S::Attribution(e) => box_iter(self.visit_attribution_mut(*e)),
            S::BodyElement(e) => box_iter(self.visit_body_element_mut(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn visit_sub_footnote_mut(
        &mut self,
        c: c::SubFootnote,
    ) -> impl Iterator<Item = c::SubFootnote> {
        use c::SubFootnote as S;
        match c {
            S::Label(e) => box_iter(self.visit_label_mut(*e)),
            S::BodyElement(e) => box_iter(self.visit_body_element_mut(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn visit_sub_figure_mut(&mut self, c: c::SubFigure) -> impl Iterator<Item = c::SubFigure> {
        use c::SubFigure as S;
        match c {
            S::Caption(e) => box_iter(self.visit_caption_mut(*e)),
            S::Legend(e) => box_iter(self.visit_legend_mut(*e)),
            S::BodyElement(e) => box_iter(self.visit_body_element_mut(*e).map(Into::into)),
        }
    }
    #[must_use]
    fn visit_sub_table_mut(&mut self, c: c::SubTable) -> impl Iterator<Item = c::SubTable> {
        use c::SubTable as S;
        match c {
            S::Title(e) => box_iter(self.visit_title_mut(*e).map(Into::into)),
            S::TableGroup(e) => box_iter(self.visit_table_group_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_table_group_mut(
        &mut self,
        c: c::SubTableGroup,
    ) -> impl Iterator<Item = c::SubTableGroup> {
        use c::SubTableGroup as S;
        match c {
            S::TableColspec(e) => box_iter(self.visit_table_colspec_mut(*e)),
            S::TableHead(e) => box_iter(self.visit_table_head_mut(*e)),
            S::TableBody(e) => box_iter(self.visit_table_body_mut(*e)),
        }
    }

    //////////////
    // elements //
    //////////////

    //structual elements
    #[must_use]
    fn visit_section_mut(&mut self, mut e: e::Section) -> impl Iterator<Item = c::SubStructure> {
        transform_children!(e, self.visit_structural_sub_element_mut);
        once(e.into())
    }
    #[must_use]
    // TODO: introduce and return category for topic|bodyelement
    fn visit_topic_mut(&mut self, mut e: e::Topic) -> impl Iterator<Item = e::Topic> {
        transform_children!(e, self.visit_sub_topic_mut);
        once(e)
    }
    #[must_use]
    fn visit_sidebar_mut(&mut self, mut e: e::Sidebar) -> impl Iterator<Item = c::SubStructure> {
        transform_children!(e, self.visit_sub_sidebar_mut);
        once(e.into())
    }

    //structural subelements
    #[must_use]
    fn visit_title_mut(&mut self, mut e: e::Title) -> impl Iterator<Item = e::Title> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e)
    }
    #[must_use]
    fn visit_subtitle_mut(&mut self, mut e: e::Subtitle) -> impl Iterator<Item = c::SubSidebar> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_decoration_mut(
        &mut self,
        mut e: e::Decoration,
    ) -> impl Iterator<Item = c::StructuralSubElement> {
        transform_children!(e, self.visit_decoration_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_docinfo_mut(
        &mut self,
        mut e: e::Docinfo,
    ) -> impl Iterator<Item = c::StructuralSubElement> {
        transform_children!(e, self.visit_bibliographic_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_transition_mut(&mut self, e: e::Transition) -> impl Iterator<Item = c::SubStructure> {
        once(e.into())
    }

    //bibliographic elements
    #[must_use]
    fn visit_authors_mut(
        &mut self,
        mut e: e::Authors,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        transform_children!(e, self.visit_author_info_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_author_mut(&mut self, mut e: e::Author) -> impl Iterator<Item = c::AuthorInfo> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_organization_mut(
        &mut self,
        mut e: e::Organization,
    ) -> impl Iterator<Item = c::AuthorInfo> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_address_mut(&mut self, mut e: e::Address) -> impl Iterator<Item = c::AuthorInfo> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_contact_mut(&mut self, mut e: e::Contact) -> impl Iterator<Item = c::AuthorInfo> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_version_mut(
        &mut self,
        mut e: e::Version,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_revision_mut(
        &mut self,
        mut e: e::Revision,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_status_mut(
        &mut self,
        mut e: e::Status,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_date_mut(&mut self, mut e: e::Date) -> impl Iterator<Item = c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_copyright_mut(
        &mut self,
        mut e: e::Copyright,
    ) -> impl Iterator<Item = c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_field_mut(&mut self, mut e: e::Field) -> impl Iterator<Item = e::Field> {
        transform_children!(e, self.visit_sub_field_mut);
        once(e)
    }

    //decoration elements
    #[must_use]
    fn visit_header_mut(&mut self, mut e: e::Header) -> impl Iterator<Item = c::DecorationElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_footer_mut(&mut self, mut e: e::Footer) -> impl Iterator<Item = c::DecorationElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }

    //simple body elements
    #[must_use]
    fn visit_paragraph_mut(&mut self, mut e: e::Paragraph) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_literal_block_mut(
        &mut self,
        mut e: e::LiteralBlock,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_doctest_block_mut(
        &mut self,
        mut e: e::DoctestBlock,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_math_block_mut(
        &mut self,
        mut e: e::MathBlock,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_string_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_rubric_mut(&mut self, mut e: e::Rubric) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_substitution_definition_mut(
        &mut self,
        mut e: e::SubstitutionDefinition,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_comment_mut(&mut self, mut e: e::Comment) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_pending_mut(&mut self, e: e::Pending) -> impl Iterator<Item = c::BodyElement> {
        once(e.into())
    }
    #[must_use]
    fn visit_target_mut(&mut self, e: e::Target) -> impl Iterator<Item = c::BodyElement> {
        once(e.into())
    }
    #[must_use]
    fn visit_raw_mut(&mut self, mut e: e::Raw) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_string_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_image_mut(&mut self, e: e::Image) -> impl Iterator<Item = c::BodyElement> {
        once(e.into())
    }

    //compound body elements
    #[must_use]
    fn visit_compound_mut(&mut self, mut e: e::Compound) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_container_mut(&mut self, mut e: e::Container) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_bullet_list_mut(
        &mut self,
        mut e: e::BulletList,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_list_item_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_enumerated_list_mut(
        &mut self,
        mut e: e::EnumeratedList,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_list_item_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_definition_list_mut(
        &mut self,
        mut e: e::DefinitionList,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_definition_list_item_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_field_list_mut(
        &mut self,
        mut e: e::FieldList,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_field_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_option_list_mut(
        &mut self,
        mut e: e::OptionList,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_option_list_item_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_line_block_mut(&mut self, mut e: e::LineBlock) -> impl Iterator<Item = e::LineBlock> {
        transform_children!(e, self.visit_sub_line_block_mut);
        once(e)
    }
    #[must_use]
    fn visit_block_quote_mut(
        &mut self,
        mut e: e::BlockQuote,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_sub_block_quote_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_admonition_mut(
        &mut self,
        mut e: e::Admonition,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_sub_topic_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_attention_mut(&mut self, mut e: e::Attention) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_hint_mut(&mut self, mut e: e::Hint) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_note_mut(&mut self, mut e: e::Note) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_caution_mut(&mut self, mut e: e::Caution) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_danger_mut(&mut self, mut e: e::Danger) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_error_mut(&mut self, mut e: e::Error) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_important_mut(&mut self, mut e: e::Important) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_tip_mut(&mut self, mut e: e::Tip) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_warning_mut(&mut self, mut e: e::Warning) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_footnote_mut(&mut self, mut e: e::Footnote) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_sub_footnote_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_citation_mut(&mut self, mut e: e::Citation) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_sub_footnote_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_system_message_mut(
        &mut self,
        mut e: e::SystemMessage,
    ) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_figure_mut(&mut self, mut e: e::Figure) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_sub_figure_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_table_mut(&mut self, mut e: e::Table) -> impl Iterator<Item = c::BodyElement> {
        transform_children!(e, self.visit_sub_table_mut);
        once(e.into())
    }

    //table elements
    #[must_use]
    fn visit_table_group_mut(&mut self, mut e: e::TableGroup) -> impl Iterator<Item = c::SubTable> {
        transform_children!(e, self.visit_sub_table_group_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_table_head_mut(
        &mut self,
        mut e: e::TableHead,
    ) -> impl Iterator<Item = c::SubTableGroup> {
        transform_children!(e, self.visit_table_row_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_table_body_mut(
        &mut self,
        mut e: e::TableBody,
    ) -> impl Iterator<Item = c::SubTableGroup> {
        transform_children!(e, self.visit_table_row_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_table_row_mut(&mut self, mut e: e::TableRow) -> impl Iterator<Item = e::TableRow> {
        transform_children!(e, self.visit_table_entry_mut);
        once(e)
    }
    #[must_use]
    fn visit_table_entry_mut(
        &mut self,
        mut e: e::TableEntry,
    ) -> impl Iterator<Item = e::TableEntry> {
        transform_children!(e, self.visit_body_element_mut);
        once(e)
    }
    #[must_use]
    fn visit_table_colspec_mut(
        &mut self,
        e: e::TableColspec,
    ) -> impl Iterator<Item = c::SubTableGroup> {
        once(e.into())
    }

    //body sub elements
    #[must_use]
    fn visit_list_item_mut(&mut self, mut e: e::ListItem) -> impl Iterator<Item = e::ListItem> {
        transform_children!(e, self.visit_body_element_mut);
        once(e)
    }
    #[must_use]
    fn visit_definition_list_item_mut(
        &mut self,
        mut e: e::DefinitionListItem,
    ) -> impl Iterator<Item = e::DefinitionListItem> {
        transform_children!(e, self.visit_sub_dl_item_mut);
        once(e)
    }
    #[must_use]
    fn visit_term_mut(&mut self, mut e: e::Term) -> impl Iterator<Item = c::SubDLItem> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_classifier_mut(&mut self, mut e: e::Classifier) -> impl Iterator<Item = c::SubDLItem> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_definition_mut(&mut self, mut e: e::Definition) -> impl Iterator<Item = c::SubDLItem> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_field_name_mut(&mut self, mut e: e::FieldName) -> impl Iterator<Item = c::SubField> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_field_body_mut(&mut self, mut e: e::FieldBody) -> impl Iterator<Item = c::SubField> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_option_list_item_mut(
        &mut self,
        mut e: e::OptionListItem,
    ) -> impl Iterator<Item = e::OptionListItem> {
        transform_children!(e, self.visit_sub_option_list_item_mut);
        once(e)
    }
    #[must_use]
    fn visit_option_group_mut(
        &mut self,
        mut e: e::OptionGroup,
    ) -> impl Iterator<Item = c::SubOptionListItem> {
        transform_children!(e, self.visit_option_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_description_mut(
        &mut self,
        mut e: e::Description,
    ) -> impl Iterator<Item = c::SubOptionListItem> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_option_mut(&mut self, mut e: e::Option_) -> impl Iterator<Item = e::Option_> {
        transform_children!(e, self.visit_sub_option_mut);
        once(e)
    }
    #[must_use]
    fn visit_option_string_mut(
        &mut self,
        mut e: e::OptionString,
    ) -> impl Iterator<Item = c::SubOption> {
        transform_children!(e, self.visit_string_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_option_argument_mut(
        &mut self,
        mut e: e::OptionArgument,
    ) -> impl Iterator<Item = c::SubOption> {
        transform_children!(e, self.visit_string_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_line_mut(&mut self, mut e: e::Line) -> impl Iterator<Item = c::SubLineBlock> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_attribution_mut(
        &mut self,
        mut e: e::Attribution,
    ) -> impl Iterator<Item = c::SubBlockQuote> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_label_mut(&mut self, mut e: e::Label) -> impl Iterator<Item = c::SubFootnote> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_caption_mut(&mut self, mut e: e::Caption) -> impl Iterator<Item = c::SubFigure> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_legend_mut(&mut self, mut e: e::Legend) -> impl Iterator<Item = c::SubFigure> {
        transform_children!(e, self.visit_body_element_mut);
        once(e.into())
    }

    //inline elements
    #[must_use]
    fn visit_string_mut(&mut self, e: String) -> impl Iterator<Item = String> {
        once(e)
    }
    #[must_use]
    fn visit_emphasis_mut(
        &mut self,
        mut e: e::Emphasis,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_literal_mut(
        &mut self,
        mut e: e::Literal,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_string_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_reference_mut(
        &mut self,
        mut e: e::Reference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_strong_mut(
        &mut self,
        mut e: e::Strong,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_footnote_reference_mut(
        &mut self,
        mut e: e::FootnoteReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_citation_reference_mut(
        &mut self,
        mut e: e::CitationReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_substitution_reference_mut(
        &mut self,
        mut e: e::SubstitutionReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_title_reference_mut(
        &mut self,
        mut e: e::TitleReference,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_abbreviation_mut(
        &mut self,
        mut e: e::Abbreviation,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_acronym_mut(
        &mut self,
        mut e: e::Acronym,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_superscript_mut(
        &mut self,
        mut e: e::Superscript,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_subscript_mut(
        &mut self,
        mut e: e::Subscript,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_inline_mut(
        &mut self,
        mut e: e::Inline,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_problematic_mut(
        &mut self,
        mut e: e::Problematic,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_generated_mut(
        &mut self,
        mut e: e::Generated,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_math_mut(&mut self, mut e: e::Math) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_string_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_target_inline_mut(
        &mut self,
        mut e: e::TargetInline,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_string_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_raw_inline_mut(
        &mut self,
        mut e: e::RawInline,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        transform_children!(e, self.visit_string_mut);
        once(e.into())
    }
    #[must_use]
    fn visit_image_inline_mut(
        &mut self,
        e: e::ImageInline,
    ) -> impl Iterator<Item = c::TextOrInlineElement> {
        once(e.into())
    }
}

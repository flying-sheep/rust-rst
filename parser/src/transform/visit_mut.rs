use std::vec;

use document_tree::HasChildren as _;
use document_tree::element_categories as c;
use document_tree::elements as e;

#[macro_export]
macro_rules! transform_children {
    ($e:ident, $self:ident . $m:ident) => {
        let new: Vec<_> = $e
            .children_mut()
            .drain(..)
            .flat_map(|c| $self.$m(c))
            .collect();
        $e.children_mut().extend(new);
    };
}

fn vec_into<F, E>(mut v: Vec<F>) -> Vec<E>
where
    F: Into<E>,
{
    v.drain(..).map(Into::into).collect()
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
    ) -> Vec<c::StructuralSubElement> {
        use c::StructuralSubElement as S;
        match c {
            S::Title(e) => vec_into(self.visit_title_mut(*e)),
            S::Subtitle(e) => vec_into(self.visit_subtitle_mut(*e)),
            S::Decoration(e) => self.visit_decoration_mut(*e),
            S::Docinfo(e) => self.visit_docinfo_mut(*e),
            S::SubStructure(e) => vec_into(self.visit_substructure_mut(*e)),
        }
    }
    #[must_use]
    fn visit_substructure_mut(&mut self, c: c::SubStructure) -> Vec<c::SubStructure> {
        use c::SubStructure as S;
        match c {
            S::Topic(e) => vec_into(self.visit_topic_mut(*e)),
            S::Sidebar(e) => self.visit_sidebar_mut(*e),
            S::Transition(e) => self.visit_transition_mut(*e),
            S::Section(e) => self.visit_section_mut(*e),
            S::BodyElement(e) => vec_into(self.visit_body_element_mut(*e)),
        }
    }
    #[must_use]
    fn visit_body_element_mut(&mut self, c: c::BodyElement) -> Vec<c::BodyElement> {
        use c::BodyElement as B;
        match c {
            B::Paragraph(e) => self.visit_paragraph_mut(*e),
            B::LiteralBlock(e) => self.visit_literal_block_mut(*e),
            B::DoctestBlock(e) => self.visit_doctest_block_mut(*e),
            B::MathBlock(e) => self.visit_math_block_mut(*e),
            B::Rubric(e) => self.visit_rubric_mut(*e),
            B::SubstitutionDefinition(e) => self.visit_substitution_definition_mut(*e),
            B::Comment(e) => self.visit_comment_mut(*e),
            B::Pending(e) => self.visit_pending_mut(*e),
            B::Target(e) => self.visit_target_mut(*e),
            B::Raw(e) => self.visit_raw_mut(*e),
            B::Image(e) => self.visit_image_mut(*e),
            B::Compound(e) => self.visit_compound_mut(*e),
            B::Container(e) => self.visit_container_mut(*e),
            B::BulletList(e) => self.visit_bullet_list_mut(*e),
            B::EnumeratedList(e) => self.visit_enumerated_list_mut(*e),
            B::DefinitionList(e) => self.visit_definition_list_mut(*e),
            B::FieldList(e) => self.visit_field_list_mut(*e),
            B::OptionList(e) => self.visit_option_list_mut(*e),
            B::LineBlock(e) => vec_into(self.visit_line_block_mut(*e)),
            B::BlockQuote(e) => self.visit_block_quote_mut(*e),
            B::Admonition(e) => self.visit_admonition_mut(*e),
            B::Attention(e) => self.visit_attention_mut(*e),
            B::Hint(e) => self.visit_hint_mut(*e),
            B::Note(e) => self.visit_note_mut(*e),
            B::Caution(e) => self.visit_caution_mut(*e),
            B::Danger(e) => self.visit_danger_mut(*e),
            B::Error(e) => self.visit_error_mut(*e),
            B::Important(e) => self.visit_important_mut(*e),
            B::Tip(e) => self.visit_tip_mut(*e),
            B::Warning(e) => self.visit_warning_mut(*e),
            B::Footnote(e) => self.visit_footnote_mut(*e),
            B::Citation(e) => self.visit_citation_mut(*e),
            B::SystemMessage(e) => self.visit_system_message_mut(*e),
            B::Figure(e) => self.visit_figure_mut(*e),
            B::Table(e) => self.visit_table_mut(*e),
        }
    }
    #[must_use]
    fn visit_bibliographic_element_mut(
        &mut self,
        c: c::BibliographicElement,
    ) -> Vec<c::BibliographicElement> {
        use c::BibliographicElement as B;
        match c {
            B::Authors(e) => self.visit_authors_mut(*e),
            B::Author(e) => vec_into(self.visit_author_mut(*e)),
            B::Organization(e) => vec_into(self.visit_organization_mut(*e)),
            B::Address(e) => vec_into(self.visit_address_mut(*e)),
            B::Contact(e) => vec_into(self.visit_contact_mut(*e)),
            B::Version(e) => self.visit_version_mut(*e),
            B::Revision(e) => self.visit_revision_mut(*e),
            B::Status(e) => self.visit_status_mut(*e),
            B::Date(e) => self.visit_date_mut(*e),
            B::Copyright(e) => self.visit_copyright_mut(*e),
            B::Field(e) => vec_into(self.visit_field_mut(*e)),
        }
    }
    #[must_use]
    fn visit_text_or_inline_element_mut(
        &mut self,
        c: c::TextOrInlineElement,
    ) -> Vec<c::TextOrInlineElement> {
        use c::TextOrInlineElement as T;
        match c {
            T::String(e) => vec_into(self.visit_string_mut(*e)),
            T::Emphasis(e) => self.visit_emphasis_mut(*e),
            T::Strong(e) => self.visit_strong_mut(*e),
            T::Literal(e) => self.visit_literal_mut(*e),
            T::Reference(e) => self.visit_reference_mut(*e),
            T::FootnoteReference(e) => self.visit_footnote_reference_mut(*e),
            T::CitationReference(e) => self.visit_citation_reference_mut(*e),
            T::SubstitutionReference(e) => self.visit_substitution_reference_mut(*e),
            T::TitleReference(e) => self.visit_title_reference_mut(*e),
            T::Abbreviation(e) => self.visit_abbreviation_mut(*e),
            T::Acronym(e) => self.visit_acronym_mut(*e),
            T::Superscript(e) => self.visit_superscript_mut(*e),
            T::Subscript(e) => self.visit_subscript_mut(*e),
            T::Inline(e) => self.visit_inline_mut(*e),
            T::Problematic(e) => self.visit_problematic_mut(*e),
            T::Generated(e) => self.visit_generated_mut(*e),
            T::Math(e) => self.visit_math_mut(*e),
            T::TargetInline(e) => self.visit_target_inline_mut(*e),
            T::RawInline(e) => self.visit_raw_inline_mut(*e),
            T::ImageInline(e) => self.visit_image_inline_mut(*e),
        }
    }
    #[must_use]
    fn visit_author_info_mut(&mut self, c: c::AuthorInfo) -> Vec<c::AuthorInfo> {
        use c::AuthorInfo as A;
        match c {
            A::Author(e) => self.visit_author_mut(*e),
            A::Organization(e) => self.visit_organization_mut(*e),
            A::Address(e) => self.visit_address_mut(*e),
            A::Contact(e) => self.visit_contact_mut(*e),
        }
    }
    #[must_use]
    fn visit_decoration_element_mut(
        &mut self,
        c: c::DecorationElement,
    ) -> Vec<c::DecorationElement> {
        use c::DecorationElement as D;
        match c {
            D::Header(e) => self.visit_header_mut(*e),
            D::Footer(e) => self.visit_footer_mut(*e),
        }
    }
    #[must_use]
    fn visit_sub_topic_mut(&mut self, c: c::SubTopic) -> Vec<c::SubTopic> {
        use c::SubTopic as S;
        match c {
            S::Title(e) => vec_into(self.visit_title_mut(*e)),
            S::BodyElement(e) => vec_into(self.visit_body_element_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_sidebar_mut(&mut self, c: c::SubSidebar) -> Vec<c::SubSidebar> {
        use c::SubSidebar as S;
        match c {
            S::Topic(e) => vec_into(self.visit_topic_mut(*e)),
            S::Title(e) => vec_into(self.visit_title_mut(*e)),
            S::Subtitle(e) => self.visit_subtitle_mut(*e),
            S::BodyElement(e) => vec_into(self.visit_body_element_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_dl_item_mut(&mut self, c: c::SubDLItem) -> Vec<c::SubDLItem> {
        use c::SubDLItem as S;
        match c {
            S::Term(e) => self.visit_term_mut(*e),
            S::Classifier(e) => self.visit_classifier_mut(*e),
            S::Definition(e) => self.visit_definition_mut(*e),
        }
    }
    #[must_use]
    fn visit_sub_field_mut(&mut self, c: c::SubField) -> Vec<c::SubField> {
        use c::SubField as S;
        match c {
            S::FieldName(e) => self.visit_field_name_mut(*e),
            S::FieldBody(e) => self.visit_field_body_mut(*e),
        }
    }
    #[must_use]
    fn visit_sub_option_list_item_mut(
        &mut self,
        c: c::SubOptionListItem,
    ) -> Vec<c::SubOptionListItem> {
        use c::SubOptionListItem as S;
        match c {
            S::OptionGroup(e) => self.visit_option_group_mut(*e),
            S::Description(e) => self.visit_description_mut(*e),
        }
    }
    #[must_use]
    fn visit_sub_option_mut(&mut self, c: c::SubOption) -> Vec<c::SubOption> {
        use c::SubOption as S;
        match c {
            S::OptionString(e) => self.visit_option_string_mut(*e),
            S::OptionArgument(e) => self.visit_option_argument_mut(*e),
        }
    }
    #[must_use]
    fn visit_sub_line_block_mut(&mut self, c: c::SubLineBlock) -> Vec<c::SubLineBlock> {
        use c::SubLineBlock as S;
        match c {
            S::LineBlock(e) => vec_into(self.visit_line_block_mut(*e)),
            S::Line(e) => self.visit_line_mut(*e),
        }
    }
    #[must_use]
    fn visit_sub_block_quote_mut(&mut self, c: c::SubBlockQuote) -> Vec<c::SubBlockQuote> {
        use c::SubBlockQuote as S;
        match c {
            S::Attribution(e) => self.visit_attribution_mut(*e),
            S::BodyElement(e) => vec_into(self.visit_body_element_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_footnote_mut(&mut self, c: c::SubFootnote) -> Vec<c::SubFootnote> {
        use c::SubFootnote as S;
        match c {
            S::Label(e) => self.visit_label_mut(*e),
            S::BodyElement(e) => vec_into(self.visit_body_element_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_figure_mut(&mut self, c: c::SubFigure) -> Vec<c::SubFigure> {
        use c::SubFigure as S;
        match c {
            S::Caption(e) => self.visit_caption_mut(*e),
            S::Legend(e) => self.visit_legend_mut(*e),
            S::BodyElement(e) => vec_into(self.visit_body_element_mut(*e)),
        }
    }
    #[must_use]
    fn visit_sub_table_mut(&mut self, c: c::SubTable) -> Vec<c::SubTable> {
        use c::SubTable as S;
        match c {
            S::Title(e) => vec_into(self.visit_title_mut(*e)),
            S::TableGroup(e) => self.visit_table_group_mut(*e),
        }
    }
    #[must_use]
    fn visit_sub_table_group_mut(&mut self, c: c::SubTableGroup) -> Vec<c::SubTableGroup> {
        use c::SubTableGroup as S;
        match c {
            S::TableColspec(e) => self.visit_table_colspec_mut(*e),
            S::TableHead(e) => self.visit_table_head_mut(*e),
            S::TableBody(e) => self.visit_table_body_mut(*e),
        }
    }

    //////////////
    // elements //
    //////////////

    //structual elements
    #[must_use]
    fn visit_section_mut(&mut self, mut e: e::Section) -> Vec<c::SubStructure> {
        transform_children!(e, self.visit_structural_sub_element_mut);
        vec![e.into()]
    }
    #[must_use]
    // TODO: introduce and return category for topic|bodyelement
    fn visit_topic_mut(&mut self, mut e: e::Topic) -> Vec<e::Topic> {
        transform_children!(e, self.visit_sub_topic_mut);
        vec![e]
    }
    #[must_use]
    fn visit_sidebar_mut(&mut self, mut e: e::Sidebar) -> Vec<c::SubStructure> {
        transform_children!(e, self.visit_sub_sidebar_mut);
        vec![e.into()]
    }

    //structural subelements
    #[must_use]
    fn visit_title_mut(&mut self, mut e: e::Title) -> Vec<e::Title> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e]
    }
    #[must_use]
    fn visit_subtitle_mut(&mut self, mut e: e::Subtitle) -> Vec<c::SubSidebar> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_decoration_mut(&mut self, mut e: e::Decoration) -> Vec<c::StructuralSubElement> {
        transform_children!(e, self.visit_decoration_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_docinfo_mut(&mut self, mut e: e::Docinfo) -> Vec<c::StructuralSubElement> {
        transform_children!(e, self.visit_bibliographic_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_transition_mut(&mut self, e: e::Transition) -> Vec<c::SubStructure> {
        vec![e.into()]
    }

    //bibliographic elements
    #[must_use]
    fn visit_authors_mut(&mut self, mut e: e::Authors) -> Vec<c::BibliographicElement> {
        transform_children!(e, self.visit_author_info_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_author_mut(&mut self, mut e: e::Author) -> Vec<c::AuthorInfo> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_organization_mut(&mut self, mut e: e::Organization) -> Vec<c::AuthorInfo> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_address_mut(&mut self, mut e: e::Address) -> Vec<c::AuthorInfo> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_contact_mut(&mut self, mut e: e::Contact) -> Vec<c::AuthorInfo> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_version_mut(&mut self, mut e: e::Version) -> Vec<c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_revision_mut(&mut self, mut e: e::Revision) -> Vec<c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_status_mut(&mut self, mut e: e::Status) -> Vec<c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_date_mut(&mut self, mut e: e::Date) -> Vec<c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_copyright_mut(&mut self, mut e: e::Copyright) -> Vec<c::BibliographicElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_field_mut(&mut self, mut e: e::Field) -> Vec<e::Field> {
        transform_children!(e, self.visit_sub_field_mut);
        vec![e]
    }

    //decoration elements
    #[must_use]
    fn visit_header_mut(&mut self, mut e: e::Header) -> Vec<c::DecorationElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_footer_mut(&mut self, mut e: e::Footer) -> Vec<c::DecorationElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }

    //simple body elements
    #[must_use]
    fn visit_paragraph_mut(&mut self, mut e: e::Paragraph) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_literal_block_mut(&mut self, mut e: e::LiteralBlock) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_doctest_block_mut(&mut self, mut e: e::DoctestBlock) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_math_block_mut(&mut self, mut e: e::MathBlock) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_string_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_rubric_mut(&mut self, mut e: e::Rubric) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_substitution_definition_mut(
        &mut self,
        mut e: e::SubstitutionDefinition,
    ) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_comment_mut(&mut self, mut e: e::Comment) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_pending_mut(&mut self, e: e::Pending) -> Vec<c::BodyElement> {
        vec![e.into()]
    }
    #[must_use]
    fn visit_target_mut(&mut self, e: e::Target) -> Vec<c::BodyElement> {
        vec![e.into()]
    }
    #[must_use]
    fn visit_raw_mut(&mut self, mut e: e::Raw) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_string_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_image_mut(&mut self, e: e::Image) -> Vec<c::BodyElement> {
        vec![e.into()]
    }

    //compound body elements
    #[must_use]
    fn visit_compound_mut(&mut self, mut e: e::Compound) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_container_mut(&mut self, mut e: e::Container) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_bullet_list_mut(&mut self, mut e: e::BulletList) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_list_item_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_enumerated_list_mut(&mut self, mut e: e::EnumeratedList) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_list_item_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_definition_list_mut(&mut self, mut e: e::DefinitionList) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_definition_list_item_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_field_list_mut(&mut self, mut e: e::FieldList) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_field_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_option_list_mut(&mut self, mut e: e::OptionList) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_option_list_item_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_line_block_mut(&mut self, mut e: e::LineBlock) -> Vec<e::LineBlock> {
        transform_children!(e, self.visit_sub_line_block_mut);
        vec![e]
    }
    #[must_use]
    fn visit_block_quote_mut(&mut self, mut e: e::BlockQuote) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_sub_block_quote_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_admonition_mut(&mut self, mut e: e::Admonition) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_sub_topic_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_attention_mut(&mut self, mut e: e::Attention) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_hint_mut(&mut self, mut e: e::Hint) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_note_mut(&mut self, mut e: e::Note) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_caution_mut(&mut self, mut e: e::Caution) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_danger_mut(&mut self, mut e: e::Danger) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_error_mut(&mut self, mut e: e::Error) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_important_mut(&mut self, mut e: e::Important) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_tip_mut(&mut self, mut e: e::Tip) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_warning_mut(&mut self, mut e: e::Warning) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_footnote_mut(&mut self, mut e: e::Footnote) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_sub_footnote_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_citation_mut(&mut self, mut e: e::Citation) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_sub_footnote_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_system_message_mut(&mut self, mut e: e::SystemMessage) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_figure_mut(&mut self, mut e: e::Figure) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_sub_figure_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_table_mut(&mut self, mut e: e::Table) -> Vec<c::BodyElement> {
        transform_children!(e, self.visit_sub_table_mut);
        vec![e.into()]
    }

    //table elements
    #[must_use]
    fn visit_table_group_mut(&mut self, mut e: e::TableGroup) -> Vec<c::SubTable> {
        transform_children!(e, self.visit_sub_table_group_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_table_head_mut(&mut self, mut e: e::TableHead) -> Vec<c::SubTableGroup> {
        transform_children!(e, self.visit_table_row_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_table_body_mut(&mut self, mut e: e::TableBody) -> Vec<c::SubTableGroup> {
        transform_children!(e, self.visit_table_row_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_table_row_mut(&mut self, mut e: e::TableRow) -> Vec<e::TableRow> {
        transform_children!(e, self.visit_table_entry_mut);
        vec![e]
    }
    #[must_use]
    fn visit_table_entry_mut(&mut self, mut e: e::TableEntry) -> Vec<e::TableEntry> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e]
    }
    #[must_use]
    fn visit_table_colspec_mut(&mut self, e: e::TableColspec) -> Vec<c::SubTableGroup> {
        vec![e.into()]
    }

    //body sub elements
    #[must_use]
    fn visit_list_item_mut(&mut self, mut e: e::ListItem) -> Vec<e::ListItem> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e]
    }
    #[must_use]
    fn visit_definition_list_item_mut(
        &mut self,
        mut e: e::DefinitionListItem,
    ) -> Vec<e::DefinitionListItem> {
        transform_children!(e, self.visit_sub_dl_item_mut);
        vec![e]
    }
    #[must_use]
    fn visit_term_mut(&mut self, mut e: e::Term) -> Vec<c::SubDLItem> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_classifier_mut(&mut self, mut e: e::Classifier) -> Vec<c::SubDLItem> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_definition_mut(&mut self, mut e: e::Definition) -> Vec<c::SubDLItem> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_field_name_mut(&mut self, mut e: e::FieldName) -> Vec<c::SubField> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_field_body_mut(&mut self, mut e: e::FieldBody) -> Vec<c::SubField> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_option_list_item_mut(&mut self, mut e: e::OptionListItem) -> Vec<e::OptionListItem> {
        transform_children!(e, self.visit_sub_option_list_item_mut);
        vec![e]
    }
    #[must_use]
    fn visit_option_group_mut(&mut self, mut e: e::OptionGroup) -> Vec<c::SubOptionListItem> {
        transform_children!(e, self.visit_option_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_description_mut(&mut self, mut e: e::Description) -> Vec<c::SubOptionListItem> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_option_mut(&mut self, mut e: e::Option_) -> Vec<e::Option_> {
        transform_children!(e, self.visit_sub_option_mut);
        vec![e]
    }
    #[must_use]
    fn visit_option_string_mut(&mut self, mut e: e::OptionString) -> Vec<c::SubOption> {
        transform_children!(e, self.visit_string_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_option_argument_mut(&mut self, mut e: e::OptionArgument) -> Vec<c::SubOption> {
        transform_children!(e, self.visit_string_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_line_mut(&mut self, mut e: e::Line) -> Vec<c::SubLineBlock> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_attribution_mut(&mut self, mut e: e::Attribution) -> Vec<c::SubBlockQuote> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_label_mut(&mut self, mut e: e::Label) -> Vec<c::SubFootnote> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_caption_mut(&mut self, mut e: e::Caption) -> Vec<c::SubFigure> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_legend_mut(&mut self, mut e: e::Legend) -> Vec<c::SubFigure> {
        transform_children!(e, self.visit_body_element_mut);
        vec![e.into()]
    }

    //inline elements
    #[must_use]
    fn visit_string_mut(&mut self, e: String) -> Vec<String> {
        vec![e]
    }
    #[must_use]
    fn visit_emphasis_mut(&mut self, mut e: e::Emphasis) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_literal_mut(&mut self, mut e: e::Literal) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_string_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_reference_mut(&mut self, mut e: e::Reference) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_strong_mut(&mut self, mut e: e::Strong) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_footnote_reference_mut(
        &mut self,
        mut e: e::FootnoteReference,
    ) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_citation_reference_mut(
        &mut self,
        mut e: e::CitationReference,
    ) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_substitution_reference_mut(
        &mut self,
        mut e: e::SubstitutionReference,
    ) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_title_reference_mut(
        &mut self,
        mut e: e::TitleReference,
    ) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_abbreviation_mut(&mut self, mut e: e::Abbreviation) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_acronym_mut(&mut self, mut e: e::Acronym) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_superscript_mut(&mut self, mut e: e::Superscript) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_subscript_mut(&mut self, mut e: e::Subscript) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_inline_mut(&mut self, mut e: e::Inline) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_problematic_mut(&mut self, mut e: e::Problematic) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_generated_mut(&mut self, mut e: e::Generated) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_text_or_inline_element_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_math_mut(&mut self, mut e: e::Math) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_string_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_target_inline_mut(&mut self, mut e: e::TargetInline) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_string_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_raw_inline_mut(&mut self, mut e: e::RawInline) -> Vec<c::TextOrInlineElement> {
        transform_children!(e, self.visit_string_mut);
        vec![e.into()]
    }
    #[must_use]
    fn visit_image_inline_mut(&mut self, e: e::ImageInline) -> Vec<c::TextOrInlineElement> {
        vec![e.into()]
    }
}

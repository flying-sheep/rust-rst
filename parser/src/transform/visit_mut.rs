use document_tree::HasChildren as _;
use document_tree::element_categories as c;
use document_tree::elements as e;

pub trait VisitMut {
    ////////////////
    // categories //
    ////////////////

    fn visit_structural_sub_element_mut(&mut self, c: &mut c::StructuralSubElement) {
        use c::StructuralSubElement as S;
        match c {
            S::Title(e) => self.visit_title_mut(e),
            S::Subtitle(e) => self.visit_subtitle_mut(e),
            S::Decoration(e) => self.visit_decoration_mut(e),
            S::Docinfo(e) => self.visit_docinfo_mut(e),
            S::SubStructure(e) => self.visit_substructure_mut(e),
        }
    }
    fn visit_substructure_mut(&mut self, c: &mut c::SubStructure) {
        use c::SubStructure as S;
        match c {
            S::Topic(e) => self.visit_topic_mut(e),
            S::Sidebar(e) => self.visit_sidebar_mut(e),
            S::Transition(e) => self.visit_transition_mut(e),
            S::Section(e) => self.visit_section_mut(e),
            S::BodyElement(e) => self.visit_body_element_mut(e),
        }
    }
    fn visit_body_element_mut(&mut self, c: &mut c::BodyElement) {
        use c::BodyElement as B;
        match c {
            B::Paragraph(e) => self.visit_paragraph_mut(e),
            B::LiteralBlock(e) => self.visit_literal_block_mut(e),
            B::DoctestBlock(e) => self.visit_doctest_block_mut(e),
            B::MathBlock(e) => self.visit_math_block_mut(e),
            B::Rubric(e) => self.visit_rubric_mut(e),
            B::SubstitutionDefinition(e) => self.visit_substitution_definition_mut(e),
            B::Comment(e) => self.visit_comment_mut(e),
            B::Pending(e) => self.visit_pending_mut(e),
            B::Target(e) => self.visit_target_mut(e),
            B::Raw(e) => self.visit_raw_mut(e),
            B::Image(e) => self.visit_image_mut(e),
            B::Compound(e) => self.visit_compound_mut(e),
            B::Container(e) => self.visit_container_mut(e),
            B::BulletList(e) => self.visit_bullet_list_mut(e),
            B::EnumeratedList(e) => self.visit_enumerated_list_mut(e),
            B::DefinitionList(e) => self.visit_definition_list_mut(e),
            B::FieldList(e) => self.visit_field_list_mut(e),
            B::OptionList(e) => self.visit_option_list_mut(e),
            B::LineBlock(e) => self.visit_line_block_mut(e),
            B::BlockQuote(e) => self.visit_block_quote_mut(e),
            B::Admonition(e) => self.visit_admonition_mut(e),
            B::Attention(e) => self.visit_attention_mut(e),
            B::Hint(e) => self.visit_hint_mut(e),
            B::Note(e) => self.visit_note_mut(e),
            B::Caution(e) => self.visit_caution_mut(e),
            B::Danger(e) => self.visit_danger_mut(e),
            B::Error(e) => self.visit_error_mut(e),
            B::Important(e) => self.visit_important_mut(e),
            B::Tip(e) => self.visit_tip_mut(e),
            B::Warning(e) => self.visit_warning_mut(e),
            B::Footnote(e) => self.visit_footnote_mut(e),
            B::Citation(e) => self.visit_citation_mut(e),
            B::SystemMessage(e) => self.visit_system_message_mut(e),
            B::Figure(e) => self.visit_figure_mut(e),
            B::Table(e) => self.visit_table_mut(e),
        }
    }
    fn visit_bibliographic_element_mut(&mut self, c: &mut c::BibliographicElement) {
        use c::BibliographicElement as B;
        match c {
            B::Author(e) => self.visit_author_mut(e),
            B::Authors(e) => self.visit_authors_mut(e),
            B::Organization(e) => self.visit_organization_mut(e),
            B::Address(e) => self.visit_address_mut(e),
            B::Contact(e) => self.visit_contact_mut(e),
            B::Version(e) => self.visit_version_mut(e),
            B::Revision(e) => self.visit_revision_mut(e),
            B::Status(e) => self.visit_status_mut(e),
            B::Date(e) => self.visit_date_mut(e),
            B::Copyright(e) => self.visit_copyright_mut(e),
            B::Field(e) => self.visit_field_mut(e),
        }
    }
    fn visit_text_or_inline_element_mut(&mut self, c: &mut c::TextOrInlineElement) {
        use c::TextOrInlineElement as T;
        match c {
            T::String(e) => self.visit_string_mut(e),
            T::Emphasis(e) => self.visit_emphasis_mut(e),
            T::Strong(e) => self.visit_strong_mut(e),
            T::Literal(e) => self.visit_literal_mut(e),
            T::Reference(e) => self.visit_reference_mut(e),
            T::FootnoteReference(e) => self.visit_footnote_reference_mut(e),
            T::CitationReference(e) => self.visit_citation_reference_mut(e),
            T::SubstitutionReference(e) => self.visit_substitution_reference_mut(e),
            T::TitleReference(e) => self.visit_title_reference_mut(e),
            T::Abbreviation(e) => self.visit_abbreviation_mut(e),
            T::Acronym(e) => self.visit_acronym_mut(e),
            T::Superscript(e) => self.visit_superscript_mut(e),
            T::Subscript(e) => self.visit_subscript_mut(e),
            T::Inline(e) => self.visit_inline_mut(e),
            T::Problematic(e) => self.visit_problematic_mut(e),
            T::Generated(e) => self.visit_generated_mut(e),
            T::Math(e) => self.visit_math_mut(e),
            T::TargetInline(e) => self.visit_target_inline_mut(e),
            T::RawInline(e) => self.visit_raw_inline_mut(e),
            T::ImageInline(e) => self.visit_image_inline_mut(e),
        }
    }
    fn visit_author_info_mut(&mut self, c: &mut c::AuthorInfo) {
        use c::AuthorInfo as A;
        match c {
            A::Author(e) => self.visit_author_mut(e),
            A::Organization(e) => self.visit_organization_mut(e),
            A::Address(e) => self.visit_address_mut(e),
            A::Contact(e) => self.visit_contact_mut(e),
        }
    }
    fn visit_decoration_element_mut(&mut self, c: &mut c::DecorationElement) {
        use c::DecorationElement as D;
        match c {
            D::Header(e) => self.visit_header_mut(e),
            D::Footer(e) => self.visit_footer_mut(e),
        }
    }
    fn visit_sub_topic_mut(&mut self, c: &mut c::SubTopic) {
        use c::SubTopic as S;
        match c {
            S::Title(e) => self.visit_title_mut(e),
            S::BodyElement(e) => self.visit_body_element_mut(e),
        }
    }
    fn visit_sub_sidebar_mut(&mut self, c: &mut c::SubSidebar) {
        use c::SubSidebar as S;
        match c {
            S::Topic(e) => self.visit_topic_mut(e),
            S::Title(e) => self.visit_title_mut(e),
            S::Subtitle(e) => self.visit_subtitle_mut(e),
            S::BodyElement(e) => self.visit_body_element_mut(e),
        }
    }
    fn visit_sub_dl_item_mut(&mut self, c: &mut c::SubDLItem) {
        use c::SubDLItem as S;
        match c {
            S::Term(e) => self.visit_term_mut(e),
            S::Classifier(e) => self.visit_classifier_mut(e),
            S::Definition(e) => self.visit_definition_mut(e),
        }
    }
    fn visit_sub_field_mut(&mut self, c: &mut c::SubField) {
        use c::SubField as S;
        match c {
            S::FieldName(e) => self.visit_field_name_mut(e),
            S::FieldBody(e) => self.visit_field_body_mut(e),
        }
    }
    fn visit_sub_option_list_item_mut(&mut self, c: &mut c::SubOptionListItem) {
        use c::SubOptionListItem as S;
        match c {
            S::OptionGroup(e) => self.visit_option_group_mut(e),
            S::Description(e) => self.visit_description_mut(e),
        }
    }
    fn visit_sub_option_mut(&mut self, c: &mut c::SubOption) {
        use c::SubOption as S;
        match c {
            S::OptionString(e) => self.visit_option_string_mut(e),
            S::OptionArgument(e) => self.visit_option_argument_mut(e),
        }
    }
    fn visit_sub_line_block_mut(&mut self, c: &mut c::SubLineBlock) {
        use c::SubLineBlock as S;
        match c {
            S::LineBlock(e) => self.visit_line_block_mut(e),
            S::Line(e) => self.visit_line_mut(e),
        }
    }
    fn visit_sub_block_quote_mut(&mut self, c: &mut c::SubBlockQuote) {
        use c::SubBlockQuote as S;
        match c {
            S::Attribution(e) => self.visit_attribution_mut(e),
            S::BodyElement(e) => self.visit_body_element_mut(e),
        }
    }
    fn visit_sub_footnote_mut(&mut self, c: &mut c::SubFootnote) {
        use c::SubFootnote as S;
        match c {
            S::Label(e) => self.visit_label_mut(e),
            S::BodyElement(e) => self.visit_body_element_mut(e),
        }
    }
    fn visit_sub_figure_mut(&mut self, c: &mut c::SubFigure) {
        use c::SubFigure as S;
        match c {
            S::Caption(e) => self.visit_caption_mut(e),
            S::Legend(e) => self.visit_legend_mut(e),
            S::BodyElement(e) => self.visit_body_element_mut(e),
        }
    }
    fn visit_sub_table_mut(&mut self, c: &mut c::SubTable) {
        use c::SubTable as S;
        match c {
            S::Title(e) => self.visit_title_mut(e),
            S::TableGroup(e) => self.visit_table_group_mut(e),
        }
    }
    fn visit_sub_table_group_mut(&mut self, c: &mut c::SubTableGroup) {
        use c::SubTableGroup as S;
        match c {
            S::TableColspec(e) => self.visit_table_colspec_mut(e),
            S::TableHead(e) => self.visit_table_head_mut(e),
            S::TableBody(e) => self.visit_table_body_mut(e),
        }
    }

    //////////////
    // elements //
    //////////////

    //structual elements
    fn visit_section_mut(&mut self, e: &mut e::Section) {
        for c in e.children_mut() {
            self.visit_structural_sub_element_mut(c);
        }
    }
    fn visit_topic_mut(&mut self, e: &mut e::Topic) {
        for c in e.children_mut() {
            self.visit_sub_topic_mut(c);
        }
    }
    fn visit_sidebar_mut(&mut self, e: &mut e::Sidebar) {
        for c in e.children_mut() {
            self.visit_sub_sidebar_mut(c);
        }
    }

    //structural subelements
    fn visit_title_mut(&mut self, e: &mut e::Title) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_subtitle_mut(&mut self, e: &mut e::Subtitle) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_decoration_mut(&mut self, e: &mut e::Decoration) {
        for c in e.children_mut() {
            self.visit_decoration_element_mut(c);
        }
    }
    fn visit_docinfo_mut(&mut self, e: &mut e::Docinfo) {
        for c in e.children_mut() {
            self.visit_bibliographic_element_mut(c);
        }
    }
    fn visit_transition_mut(&mut self, _: &mut e::Transition) {}

    //bibliographic elements
    fn visit_author_mut(&mut self, e: &mut e::Author) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_authors_mut(&mut self, e: &mut e::Authors) {
        for c in e.children_mut() {
            self.visit_author_info_mut(c);
        }
    }
    fn visit_organization_mut(&mut self, e: &mut e::Organization) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_address_mut(&mut self, e: &mut e::Address) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_contact_mut(&mut self, e: &mut e::Contact) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_version_mut(&mut self, e: &mut e::Version) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_revision_mut(&mut self, e: &mut e::Revision) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_status_mut(&mut self, e: &mut e::Status) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_date_mut(&mut self, e: &mut e::Date) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_copyright_mut(&mut self, e: &mut e::Copyright) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_field_mut(&mut self, e: &mut e::Field) {
        for c in e.children_mut() {
            self.visit_sub_field_mut(c);
        }
    }

    //decoration elements
    fn visit_header_mut(&mut self, e: &mut e::Header) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_footer_mut(&mut self, e: &mut e::Footer) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }

    //simple body elements
    fn visit_paragraph_mut(&mut self, e: &mut e::Paragraph) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_literal_block_mut(&mut self, e: &mut e::LiteralBlock) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_doctest_block_mut(&mut self, e: &mut e::DoctestBlock) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_math_block_mut(&mut self, e: &mut e::MathBlock) {
        for c in e.children_mut() {
            self.visit_string_mut(c);
        }
    }
    fn visit_rubric_mut(&mut self, e: &mut e::Rubric) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_substitution_definition_mut(&mut self, e: &mut e::SubstitutionDefinition) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_comment_mut(&mut self, e: &mut e::Comment) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_pending_mut(&mut self, _: &mut e::Pending) {}
    fn visit_target_mut(&mut self, _: &mut e::Target) {}
    fn visit_raw_mut(&mut self, e: &mut e::Raw) {
        for c in e.children_mut() {
            self.visit_string_mut(c);
        }
    }
    fn visit_image_mut(&mut self, _: &mut e::Image) {}

    //compound body elements
    fn visit_compound_mut(&mut self, e: &mut e::Compound) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_container_mut(&mut self, e: &mut e::Container) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_bullet_list_mut(&mut self, e: &mut e::BulletList) {
        for c in e.children_mut() {
            self.visit_list_item_mut(c);
        }
    }
    fn visit_enumerated_list_mut(&mut self, e: &mut e::EnumeratedList) {
        for c in e.children_mut() {
            self.visit_list_item_mut(c);
        }
    }
    fn visit_definition_list_mut(&mut self, e: &mut e::DefinitionList) {
        for c in e.children_mut() {
            self.visit_definition_list_item_mut(c);
        }
    }
    fn visit_field_list_mut(&mut self, e: &mut e::FieldList) {
        for c in e.children_mut() {
            self.visit_field_mut(c);
        }
    }
    fn visit_option_list_mut(&mut self, e: &mut e::OptionList) {
        for c in e.children_mut() {
            self.visit_option_list_item(c);
        }
    }
    fn visit_line_block_mut(&mut self, e: &mut e::LineBlock) {
        for c in e.children_mut() {
            self.visit_sub_line_block_mut(c);
        }
    }
    fn visit_block_quote_mut(&mut self, e: &mut e::BlockQuote) {
        for c in e.children_mut() {
            self.visit_sub_block_quote_mut(c);
        }
    }
    fn visit_admonition_mut(&mut self, e: &mut e::Admonition) {
        for c in e.children_mut() {
            self.visit_sub_topic_mut(c);
        }
    }
    fn visit_attention_mut(&mut self, e: &mut e::Attention) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_hint_mut(&mut self, e: &mut e::Hint) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_note_mut(&mut self, e: &mut e::Note) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_caution_mut(&mut self, e: &mut e::Caution) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_danger_mut(&mut self, e: &mut e::Danger) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_error_mut(&mut self, e: &mut e::Error) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_important_mut(&mut self, e: &mut e::Important) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_tip_mut(&mut self, e: &mut e::Tip) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_warning_mut(&mut self, e: &mut e::Warning) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_footnote_mut(&mut self, e: &mut e::Footnote) {
        for c in e.children_mut() {
            self.visit_sub_footnote_mut(c);
        }
    }
    fn visit_citation_mut(&mut self, e: &mut e::Citation) {
        for c in e.children_mut() {
            self.visit_sub_footnote_mut(c);
        }
    }
    fn visit_system_message_mut(&mut self, e: &mut e::SystemMessage) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_figure_mut(&mut self, e: &mut e::Figure) {
        for c in e.children_mut() {
            self.visit_sub_figure_mut(c);
        }
    }
    fn visit_table_mut(&mut self, e: &mut e::Table) {
        for c in e.children_mut() {
            self.visit_sub_table_mut(c);
        }
    }

    //table elements
    fn visit_table_group_mut(&mut self, e: &mut e::TableGroup) {
        for c in e.children_mut() {
            self.visit_sub_table_group_mut(c);
        }
    }
    fn visit_table_head_mut(&mut self, e: &mut e::TableHead) {
        for c in e.children_mut() {
            self.visit_table_row_mut(c);
        }
    }
    fn visit_table_body_mut(&mut self, e: &mut e::TableBody) {
        for c in e.children_mut() {
            self.visit_table_row_mut(c);
        }
    }
    fn visit_table_row_mut(&mut self, e: &mut e::TableRow) {
        for c in e.children_mut() {
            self.visit_table_entry_mut(c);
        }
    }
    fn visit_table_entry_mut(&mut self, e: &mut e::TableEntry) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_table_colspec_mut(&mut self, _: &mut e::TableColspec) {}

    //body sub elements
    fn visit_list_item_mut(&mut self, e: &mut e::ListItem) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_definition_list_item_mut(&mut self, e: &mut e::DefinitionListItem) {
        for c in e.children_mut() {
            self.visit_sub_dl_item_mut(c);
        }
    }
    fn visit_term_mut(&mut self, e: &mut e::Term) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_classifier_mut(&mut self, e: &mut e::Classifier) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_definition_mut(&mut self, e: &mut e::Definition) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_field_name_mut(&mut self, e: &mut e::FieldName) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_field_body_mut(&mut self, e: &mut e::FieldBody) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_option_list_item(&mut self, e: &mut e::OptionListItem) {
        for c in e.children_mut() {
            self.visit_sub_option_list_item_mut(c);
        }
    }
    fn visit_option_group_mut(&mut self, e: &mut e::OptionGroup) {
        for c in e.children_mut() {
            self.visit_option_mut(c);
        }
    }
    fn visit_description_mut(&mut self, e: &mut e::Description) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }
    fn visit_option_mut(&mut self, e: &mut e::Option_) {
        for c in e.children_mut() {
            self.visit_sub_option_mut(c);
        }
    }
    fn visit_option_string_mut(&mut self, e: &mut e::OptionString) {
        for c in e.children_mut() {
            self.visit_string_mut(c);
        }
    }
    fn visit_option_argument_mut(&mut self, e: &mut e::OptionArgument) {
        for c in e.children_mut() {
            self.visit_string_mut(c);
        }
    }
    fn visit_line_mut(&mut self, e: &mut e::Line) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_attribution_mut(&mut self, e: &mut e::Attribution) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_label_mut(&mut self, e: &mut e::Label) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_caption_mut(&mut self, e: &mut e::Caption) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_legend_mut(&mut self, e: &mut e::Legend) {
        for c in e.children_mut() {
            self.visit_body_element_mut(c);
        }
    }

    //inline elements
    fn visit_string_mut(&mut self, _: &mut str) {}
    fn visit_emphasis_mut(&mut self, e: &mut e::Emphasis) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_literal_mut(&mut self, e: &mut e::Literal) {
        for c in e.children_mut() {
            self.visit_string_mut(c);
        }
    }
    fn visit_reference_mut(&mut self, e: &mut e::Reference) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_strong_mut(&mut self, e: &mut e::Strong) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_footnote_reference_mut(&mut self, e: &mut e::FootnoteReference) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_citation_reference_mut(&mut self, e: &mut e::CitationReference) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_substitution_reference_mut(&mut self, e: &mut e::SubstitutionReference) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_title_reference_mut(&mut self, e: &mut e::TitleReference) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_abbreviation_mut(&mut self, e: &mut e::Abbreviation) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_acronym_mut(&mut self, e: &mut e::Acronym) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_superscript_mut(&mut self, e: &mut e::Superscript) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_subscript_mut(&mut self, e: &mut e::Subscript) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_inline_mut(&mut self, e: &mut e::Inline) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_problematic_mut(&mut self, e: &mut e::Problematic) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_generated_mut(&mut self, e: &mut e::Generated) {
        for c in e.children_mut() {
            self.visit_text_or_inline_element_mut(c);
        }
    }
    fn visit_math_mut(&mut self, e: &mut e::Math) {
        for c in e.children_mut() {
            self.visit_string_mut(c);
        }
    }

    //non-inline versions of inline elements
    fn visit_target_inline_mut(&mut self, e: &mut e::TargetInline) {
        for c in e.children_mut() {
            self.visit_string_mut(c);
        }
    }
    fn visit_raw_inline_mut(&mut self, e: &mut e::RawInline) {
        for c in e.children_mut() {
            self.visit_string_mut(c);
        }
    }
    fn visit_image_inline_mut(&mut self, _: &mut e::ImageInline) {}
}

use document_tree::HasChildren as _;
use document_tree::element_categories as c;
use document_tree::elements as e;

pub trait Visit<'tree> {
    ////////////////
    // categories //
    ////////////////

    fn visit_structural_sub_element(&mut self, c: &'tree c::StructuralSubElement) {
        use c::StructuralSubElement as S;
        match c {
            S::Title(e) => self.visit_title(e),
            S::Subtitle(e) => self.visit_subtitle(e),
            S::Decoration(e) => self.visit_decoration(e),
            S::Docinfo(e) => self.visit_docinfo(e),
            S::SubStructure(e) => self.visit_substructure(e),
        }
    }
    fn visit_substructure(&mut self, c: &'tree c::SubStructure) {
        use c::SubStructure as S;
        match c {
            S::Topic(e) => self.visit_topic(e),
            S::Sidebar(e) => self.visit_sidebar(e),
            S::Transition(e) => self.visit_transition(e),
            S::Section(e) => self.visit_section(e),
            S::BodyElement(e) => self.visit_body_element(e),
        }
    }
    fn visit_body_element(&mut self, c: &'tree c::BodyElement) {
        use c::BodyElement as B;
        match c {
            B::Paragraph(e) => self.visit_paragraph(e),
            B::LiteralBlock(e) => self.visit_literal_block(e),
            B::DoctestBlock(e) => self.visit_doctest_block(e),
            B::MathBlock(e) => self.visit_math_block(e),
            B::Rubric(e) => self.visit_rubric(e),
            B::SubstitutionDefinition(e) => self.visit_substitution_definition(e),
            B::Comment(e) => self.visit_comment(e),
            B::Pending(e) => self.visit_pending(e),
            B::Target(e) => self.visit_target(e),
            B::Raw(e) => self.visit_raw(e),
            B::Image(e) => self.visit_image(e),
            B::Compound(e) => self.visit_compound(e),
            B::Container(e) => self.visit_container(e),
            B::BulletList(e) => self.visit_bullet_list(e),
            B::EnumeratedList(e) => self.visit_enumerated_list(e),
            B::DefinitionList(e) => self.visit_definition_list(e),
            B::FieldList(e) => self.visit_field_list(e),
            B::OptionList(e) => self.visit_option_list(e),
            B::LineBlock(e) => self.visit_line_block(e),
            B::BlockQuote(e) => self.visit_block_quote(e),
            B::Admonition(e) => self.visit_admonition(e),
            B::Attention(e) => self.visit_attention(e),
            B::Hint(e) => self.visit_hint(e),
            B::Note(e) => self.visit_note(e),
            B::Caution(e) => self.visit_caution(e),
            B::Danger(e) => self.visit_danger(e),
            B::Error(e) => self.visit_error(e),
            B::Important(e) => self.visit_important(e),
            B::Tip(e) => self.visit_tip(e),
            B::Warning(e) => self.visit_warning(e),
            B::Footnote(e) => self.visit_footnote(e),
            B::Citation(e) => self.visit_citation(e),
            B::SystemMessage(e) => self.visit_system_message(e),
            B::Figure(e) => self.visit_figure(e),
            B::Table(e) => self.visit_table(e),
        }
    }
    fn visit_bibliographic_element(&mut self, c: &'tree c::BibliographicElement) {
        use c::BibliographicElement as B;
        match c {
            B::Author(e) => self.visit_author(e),
            B::Authors(e) => self.visit_authors(e),
            B::Organization(e) => self.visit_organization(e),
            B::Address(e) => self.visit_address(e),
            B::Contact(e) => self.visit_contact(e),
            B::Version(e) => self.visit_version(e),
            B::Revision(e) => self.visit_revision(e),
            B::Status(e) => self.visit_status(e),
            B::Date(e) => self.visit_date(e),
            B::Copyright(e) => self.visit_copyright(e),
            B::Field(e) => self.visit_field(e),
        }
    }
    fn visit_text_or_inline_element(&mut self, c: &'tree c::TextOrInlineElement) {
        use c::TextOrInlineElement as T;
        match c {
            T::String(e) => self.visit_string(e),
            T::Emphasis(e) => self.visit_emphasis(e),
            T::Strong(e) => self.visit_strong(e),
            T::Literal(e) => self.visit_literal(e),
            T::Reference(e) => self.visit_reference(e),
            T::FootnoteReference(e) => self.visit_footnote_reference(e),
            T::CitationReference(e) => self.visit_citation_reference(e),
            T::SubstitutionReference(e) => self.visit_substitution_reference(e),
            T::TitleReference(e) => self.visit_title_reference(e),
            T::Abbreviation(e) => self.visit_abbreviation(e),
            T::Acronym(e) => self.visit_acronym(e),
            T::Superscript(e) => self.visit_superscript(e),
            T::Subscript(e) => self.visit_subscript(e),
            T::Inline(e) => self.visit_inline(e),
            T::Problematic(e) => self.visit_problematic(e),
            T::Generated(e) => self.visit_generated(e),
            T::Math(e) => self.visit_math(e),
            T::TargetInline(e) => self.visit_target_inline(e),
            T::RawInline(e) => self.visit_raw_inline(e),
            T::ImageInline(e) => self.visit_image_inline(e),
        }
    }
    fn visit_author_info(&mut self, c: &'tree c::AuthorInfo) {
        use c::AuthorInfo as A;
        match c {
            A::Author(e) => self.visit_author(e),
            A::Organization(e) => self.visit_organization(e),
            A::Address(e) => self.visit_address(e),
            A::Contact(e) => self.visit_contact(e),
        }
    }
    fn visit_decoration_element(&mut self, c: &'tree c::DecorationElement) {
        use c::DecorationElement as D;
        match c {
            D::Header(e) => self.visit_header(e),
            D::Footer(e) => self.visit_footer(e),
        }
    }
    fn visit_sub_topic(&mut self, c: &'tree c::SubTopic) {
        use c::SubTopic as S;
        match c {
            S::Title(e) => self.visit_title(e),
            S::BodyElement(e) => self.visit_body_element(e),
        }
    }
    fn visit_sub_sidebar(&mut self, c: &'tree c::SubSidebar) {
        use c::SubSidebar as S;
        match c {
            S::Topic(e) => self.visit_topic(e),
            S::Title(e) => self.visit_title(e),
            S::Subtitle(e) => self.visit_subtitle(e),
            S::BodyElement(e) => self.visit_body_element(e),
        }
    }
    fn visit_sub_dl_item(&mut self, c: &'tree c::SubDLItem) {
        use c::SubDLItem as S;
        match c {
            S::Term(e) => self.visit_term(e),
            S::Classifier(e) => self.visit_classifier(e),
            S::Definition(e) => self.visit_definition(e),
        }
    }
    fn visit_sub_field(&mut self, c: &'tree c::SubField) {
        use c::SubField as S;
        match c {
            S::FieldName(e) => self.visit_field_name(e),
            S::FieldBody(e) => self.visit_field_body(e),
        }
    }
    fn visit_sub_option_list_item(&mut self, c: &'tree c::SubOptionListItem) {
        use c::SubOptionListItem as S;
        match c {
            S::OptionGroup(e) => self.visit_option_group(e),
            S::Description(e) => self.visit_description(e),
        }
    }
    fn visit_sub_option(&mut self, c: &'tree c::SubOption) {
        use c::SubOption as S;
        match c {
            S::OptionString(e) => self.visit_option_string(e),
            S::OptionArgument(e) => self.visit_option_argument(e),
        }
    }
    fn visit_sub_line_block(&mut self, c: &'tree c::SubLineBlock) {
        use c::SubLineBlock as S;
        match c {
            S::LineBlock(e) => self.visit_line_block(e),
            S::Line(e) => self.visit_line(e),
        }
    }
    fn visit_sub_block_quote(&mut self, c: &'tree c::SubBlockQuote) {
        use c::SubBlockQuote as S;
        match c {
            S::Attribution(e) => self.visit_attribution(e),
            S::BodyElement(e) => self.visit_body_element(e),
        }
    }
    fn visit_sub_footnote(&mut self, c: &'tree c::SubFootnote) {
        use c::SubFootnote as S;
        match c {
            S::Label(e) => self.visit_label(e),
            S::BodyElement(e) => self.visit_body_element(e),
        }
    }
    fn visit_sub_figure(&mut self, c: &'tree c::SubFigure) {
        use c::SubFigure as S;
        match c {
            S::Caption(e) => self.visit_caption(e),
            S::Legend(e) => self.visit_legend(e),
            S::BodyElement(e) => self.visit_body_element(e),
        }
    }
    fn visit_sub_table(&mut self, c: &'tree c::SubTable) {
        use c::SubTable as S;
        match c {
            S::Title(e) => self.visit_title(e),
            S::TableGroup(e) => self.visit_table_group(e),
        }
    }
    fn visit_sub_table_group(&mut self, c: &'tree c::SubTableGroup) {
        use c::SubTableGroup as S;
        match c {
            S::TableColspec(e) => self.visit_table_colspec(e),
            S::TableHead(e) => self.visit_table_head(e),
            S::TableBody(e) => self.visit_table_body(e),
        }
    }

    //////////////
    // elements //
    //////////////

    //structual elements
    fn visit_section(&mut self, e: &'tree e::Section) {
        for c in e.children() {
            self.visit_structural_sub_element(c);
        }
    }
    fn visit_topic(&mut self, e: &'tree e::Topic) {
        for c in e.children() {
            self.visit_sub_topic(c);
        }
    }
    fn visit_sidebar(&mut self, e: &'tree e::Sidebar) {
        for c in e.children() {
            self.visit_sub_sidebar(c);
        }
    }

    //structural subelements
    fn visit_title(&mut self, e: &'tree e::Title) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_subtitle(&mut self, e: &'tree e::Subtitle) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_decoration(&mut self, e: &'tree e::Decoration) {
        for c in e.children() {
            self.visit_decoration_element(c);
        }
    }
    fn visit_docinfo(&mut self, e: &'tree e::Docinfo) {
        for c in e.children() {
            self.visit_bibliographic_element(c);
        }
    }
    fn visit_transition(&mut self, _: &'tree e::Transition) {}

    //bibliographic elements
    fn visit_author(&mut self, e: &'tree e::Author) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_authors(&mut self, e: &'tree e::Authors) {
        for c in e.children() {
            self.visit_author_info(c);
        }
    }
    fn visit_organization(&mut self, e: &'tree e::Organization) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_address(&mut self, e: &'tree e::Address) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_contact(&mut self, e: &'tree e::Contact) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_version(&mut self, e: &'tree e::Version) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_revision(&mut self, e: &'tree e::Revision) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_status(&mut self, e: &'tree e::Status) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_date(&mut self, e: &'tree e::Date) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_copyright(&mut self, e: &'tree e::Copyright) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_field(&mut self, e: &'tree e::Field) {
        for c in e.children() {
            self.visit_sub_field(c);
        }
    }

    //decoration elements
    fn visit_header(&mut self, e: &'tree e::Header) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_footer(&mut self, e: &'tree e::Footer) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }

    //simple body elements
    fn visit_paragraph(&mut self, e: &'tree e::Paragraph) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_literal_block(&mut self, e: &'tree e::LiteralBlock) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_doctest_block(&mut self, e: &'tree e::DoctestBlock) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_math_block(&mut self, e: &'tree e::MathBlock) {
        for c in e.children() {
            self.visit_string(c);
        }
    }
    fn visit_rubric(&mut self, e: &'tree e::Rubric) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_substitution_definition(&mut self, e: &'tree e::SubstitutionDefinition) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_comment(&mut self, e: &'tree e::Comment) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_pending(&mut self, _: &'tree e::Pending) {}
    fn visit_target(&mut self, _: &'tree e::Target) {}
    fn visit_raw(&mut self, e: &'tree e::Raw) {
        for c in e.children() {
            self.visit_string(c);
        }
    }
    fn visit_image(&mut self, _: &'tree e::Image) {}

    //compound body elements
    fn visit_compound(&mut self, e: &'tree e::Compound) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_container(&mut self, e: &'tree e::Container) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_bullet_list(&mut self, e: &'tree e::BulletList) {
        for c in e.children() {
            self.visit_list_item(c);
        }
    }
    fn visit_enumerated_list(&mut self, e: &'tree e::EnumeratedList) {
        for c in e.children() {
            self.visit_list_item(c);
        }
    }
    fn visit_definition_list(&mut self, e: &'tree e::DefinitionList) {
        for c in e.children() {
            self.visit_definition_list_item(c);
        }
    }
    fn visit_field_list(&mut self, e: &'tree e::FieldList) {
        for c in e.children() {
            self.visit_field(c);
        }
    }
    fn visit_option_list(&mut self, e: &'tree e::OptionList) {
        for c in e.children() {
            self.visit_option_list_item(c);
        }
    }
    fn visit_line_block(&mut self, e: &'tree e::LineBlock) {
        for c in e.children() {
            self.visit_sub_line_block(c);
        }
    }
    fn visit_block_quote(&mut self, e: &'tree e::BlockQuote) {
        for c in e.children() {
            self.visit_sub_block_quote(c);
        }
    }
    fn visit_admonition(&mut self, e: &'tree e::Admonition) {
        for c in e.children() {
            self.visit_sub_topic(c);
        }
    }
    fn visit_attention(&mut self, e: &'tree e::Attention) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_hint(&mut self, e: &'tree e::Hint) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_note(&mut self, e: &'tree e::Note) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_caution(&mut self, e: &'tree e::Caution) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_danger(&mut self, e: &'tree e::Danger) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_error(&mut self, e: &'tree e::Error) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_important(&mut self, e: &'tree e::Important) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_tip(&mut self, e: &'tree e::Tip) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_warning(&mut self, e: &'tree e::Warning) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_footnote(&mut self, e: &'tree e::Footnote) {
        for c in e.children() {
            self.visit_sub_footnote(c);
        }
    }
    fn visit_citation(&mut self, e: &'tree e::Citation) {
        for c in e.children() {
            self.visit_sub_footnote(c);
        }
    }
    fn visit_system_message(&mut self, e: &'tree e::SystemMessage) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_figure(&mut self, e: &'tree e::Figure) {
        for c in e.children() {
            self.visit_sub_figure(c);
        }
    }
    fn visit_table(&mut self, e: &'tree e::Table) {
        for c in e.children() {
            self.visit_sub_table(c);
        }
    }

    //table elements
    fn visit_table_group(&mut self, e: &'tree e::TableGroup) {
        for c in e.children() {
            self.visit_sub_table_group(c);
        }
    }
    fn visit_table_head(&mut self, e: &'tree e::TableHead) {
        for c in e.children() {
            self.visit_table_row(c);
        }
    }
    fn visit_table_body(&mut self, e: &'tree e::TableBody) {
        for c in e.children() {
            self.visit_table_row(c);
        }
    }
    fn visit_table_row(&mut self, e: &'tree e::TableRow) {
        for c in e.children() {
            self.visit_table_entry(c);
        }
    }
    fn visit_table_entry(&mut self, e: &'tree e::TableEntry) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_table_colspec(&mut self, _: &'tree e::TableColspec) {}

    //body sub elements
    fn visit_list_item(&mut self, e: &'tree e::ListItem) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_definition_list_item(&mut self, e: &'tree e::DefinitionListItem) {
        for c in e.children() {
            self.visit_sub_dl_item(c);
        }
    }
    fn visit_term(&mut self, e: &'tree e::Term) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_classifier(&mut self, e: &'tree e::Classifier) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_definition(&mut self, e: &'tree e::Definition) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_field_name(&mut self, e: &'tree e::FieldName) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_field_body(&mut self, e: &'tree e::FieldBody) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_option_list_item(&mut self, e: &'tree e::OptionListItem) {
        for c in e.children() {
            self.visit_sub_option_list_item(c);
        }
    }
    fn visit_option_group(&mut self, e: &'tree e::OptionGroup) {
        for c in e.children() {
            self.visit_option(c);
        }
    }
    fn visit_description(&mut self, e: &'tree e::Description) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }
    fn visit_option(&mut self, e: &'tree e::Option_) {
        for c in e.children() {
            self.visit_sub_option(c);
        }
    }
    fn visit_option_string(&mut self, e: &'tree e::OptionString) {
        for c in e.children() {
            self.visit_string(c);
        }
    }
    fn visit_option_argument(&mut self, e: &'tree e::OptionArgument) {
        for c in e.children() {
            self.visit_string(c);
        }
    }
    fn visit_line(&mut self, e: &'tree e::Line) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_attribution(&mut self, e: &'tree e::Attribution) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_label(&mut self, e: &'tree e::Label) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_caption(&mut self, e: &'tree e::Caption) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_legend(&mut self, e: &'tree e::Legend) {
        for c in e.children() {
            self.visit_body_element(c);
        }
    }

    //inline elements
    fn visit_string(&mut self, _: &'tree str) {}
    fn visit_emphasis(&mut self, e: &'tree e::Emphasis) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_literal(&mut self, e: &'tree e::Literal) {
        for c in e.children() {
            self.visit_string(c);
        }
    }
    fn visit_reference(&mut self, e: &'tree e::Reference) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_strong(&mut self, e: &'tree e::Strong) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_footnote_reference(&mut self, e: &'tree e::FootnoteReference) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_citation_reference(&mut self, e: &'tree e::CitationReference) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_substitution_reference(&mut self, e: &'tree e::SubstitutionReference) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_title_reference(&mut self, e: &'tree e::TitleReference) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_abbreviation(&mut self, e: &'tree e::Abbreviation) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_acronym(&mut self, e: &'tree e::Acronym) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_superscript(&mut self, e: &'tree e::Superscript) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_subscript(&mut self, e: &'tree e::Subscript) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_inline(&mut self, e: &'tree e::Inline) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_problematic(&mut self, e: &'tree e::Problematic) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_generated(&mut self, e: &'tree e::Generated) {
        for c in e.children() {
            self.visit_text_or_inline_element(c);
        }
    }
    fn visit_math(&mut self, e: &'tree e::Math) {
        for c in e.children() {
            self.visit_string(c);
        }
    }

    //non-inline versions of inline elements
    fn visit_target_inline(&mut self, e: &'tree e::TargetInline) {
        for c in e.children() {
            self.visit_string(c);
        }
    }
    fn visit_raw_inline(&mut self, e: &'tree e::RawInline) {
        for c in e.children() {
            self.visit_string(c);
        }
    }
    fn visit_image_inline(&mut self, _: &'tree e::ImageInline) {}
}

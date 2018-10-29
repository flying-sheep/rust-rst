pub mod token;

#[allow(unused_imports)]
use pest::consumes_to;
#[allow(unused_imports)]
use pest::parses_to;

#[derive(Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;



#[test]
fn plain() {
    parses_to! {
        parser: RstParser,
        input:  "line\n",
        rule:   Rule::plain,
        tokens: [
            plain(0, 5, [
                inlines(0, 5, [
                    inline(0, 4, [str(0, 4)]),
                    EOI(5, 5)
                ])
            ])
        ]
    };
}

#[test]
fn title() {
    parses_to! {
        parser: RstParser,
        input:  "\
Title
=====
",
        rule:   Rule::heading,
        tokens: [
            heading(0, 12, [
                inline(0, 5, [str(0, 5)]),
                setext_bottom(6, 12),
            ])
        ]
    };
}

#[test]
fn heading_title() {
    parses_to! {
        parser: RstParser,
        input:  "\
-----
Title
-----
",
        rule:   Rule::heading_title,
        tokens: [
            heading_title(0, 18, [
                setext_bottom(0, 6),
                inline(6, 11, [str(6, 11)]),
                setext_bottom(12, 18),
            ])
        ]
    };
}

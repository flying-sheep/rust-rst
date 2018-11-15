pub mod token;
pub mod serialize;

#[allow(unused_imports)]
use pest::consumes_to;
#[allow(unused_imports)]
use pest::parses_to;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rst.pest"]
pub struct RstParser;


#[test]
fn plain() {
    parses_to! {
        parser: RstParser,
        input:  "line\n",
        rule:   Rule::paragraph,
        tokens: [
            paragraph(0, 5, [
                line(0, 5)
            ])
        ]
    };
}

/* #[test]
fn title() {
    parses_to! {
        parser: RstParser,
        input:  "\
Title
=====
",
        rule:   Rule::title,
        tokens: [
            title(0, 12, [
                line(0, 6),
                adornments(6, 11),
            ])
        ]
    };
}

#[test]
fn title_overline() {
    parses_to! {
        parser: RstParser,
        input:  "\
-----
Title
-----
",
        rule:   Rule::title,
        tokens: [
            title(0, 18, [
                adornments(0, 6),
                line(6, 12),
                adornments(12, 18),
            ])
        ]
    };
} */

#[test]
fn nested_lists() {
    parses_to! {
        parser: RstParser,
        input: "\
paragraph

-  item 1
-  item 2
   more text
   more text 2
   more text 3
   - nested item 1
   - nested item 2
   - nested item 3
",
        rule: Rule::document,
        tokens: [
            paragraph(0, 10, [ line(0, 10) ]),
            bullet_list(11, 131, [
                bullet_item(11, 21, [ line(14, 21) ]),
                bullet_item(21, 131, [
                    line(24, 31),
                    paragraph(34, 74, [
                        line(34, 44),
                        line(47, 59),
                        line(62, 74),
                    ]),
                    bullet_list(77, 131, [
                        bullet_item(77, 93, [ line(79, 93) ]),
                        bullet_item(96, 112, [ line(98, 112) ]),
                        bullet_item(115, 131, [ line(117, 131) ]),
                    ]),
                ]),
            ]),
        ]
    }
}

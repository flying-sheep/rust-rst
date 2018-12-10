use pest::consumes_to;
use pest::parses_to;
use super::pest_rst::{RstParser, Rule};

#[test]
fn plain() {
    parses_to! {
        parser: RstParser,
        input: "line\n",
        rule: Rule::paragraph,
        tokens: [
            paragraph(0, 4, [
                str(0, 4)
            ])
        ]
    };
}

#[test]
fn title() {
    parses_to! {
        parser: RstParser,
        input: "\
Title
=====
",
        rule: Rule::title,
        tokens: [
            title(0, 12, [ title_single(0, 12, [
                line(0, 6, [ str(0, 5) ]),
                adornments(6, 11),
            ]) ])
        ]
    };
}

#[test]
fn title_overline() {
    parses_to! {
        parser: RstParser,
        input: "\
-----
Title
-----
",
        rule: Rule::title,
        tokens: [
            title(0, 17, [ title_double(0, 17, [
                adornments(0, 5),
                line(6, 12, [ str(6, 11) ]),
            ]) ])
        ]
    };
}

#[allow(clippy::cyclomatic_complexity)]
#[test]
fn two_targets() {
    parses_to! {
        parser: RstParser,
        input: "\
.. _a: http://example.com
.. _`b_`: https://example.org
",
        rule: Rule::document,
        tokens: [
            target(0, 26, [
                target_name_uq(4, 5),
                link_target(7, 25),
            ]),
            target(26, 56, [
                target_name_qu(31, 33),
                link_target(36, 55),
            ]),
        ]
    };
}

#[allow(clippy::cyclomatic_complexity)]
#[test]
fn admonitions() {
    parses_to! {
        parser: RstParser,
        input: "\
.. note::
   Just next line
.. admonition:: In line title

   Next line

.. danger:: Just this line
",
        rule: Rule::document,
        tokens: [
            admonition_gen(0, 27, [
                admonition_type(3, 7),
                paragraph(13, 27, [ str(13, 27) ]),
            ]),
            admonition(28, 71, [
                line(43, 58, [ str(43, 57) ]),
                paragraph(62, 71, [ str(62, 71) ]),
            ]),
            admonition_gen(73, 100, [
                admonition_type(76, 82),
                line(84, 100, [ str(84, 99) ]),
            ]),
        ]
    };
}

// TODO: test substitutions
// TODO: test images

#[allow(clippy::cyclomatic_complexity)]
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
            paragraph(0, 9, [ str(0, 9) ]),
            bullet_list(11, 131, [
                bullet_item(11, 21, [
                    line(14, 21, [ str(14, 20) ]),
                ]),
                bullet_item(21, 131, [
                    line(24, 31, [ str(24, 30) ]),
                    paragraph(34, 74, [
                        str(34, 43),
                        str(47, 58),
                        str(62, 73),
                    ]),
                    bullet_list(77, 131, [
                        bullet_item( 77,  93, [ line( 79,  93, [str( 79,  92)]) ]),
                        bullet_item( 96, 112, [ line( 98, 112, [str( 98, 111)]) ]),
                        bullet_item(115, 131, [ line(117, 131, [str(117, 130)]) ]),
                    ]),
                ]),
            ]),
        ]
    }
}

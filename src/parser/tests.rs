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
            paragraph(0, 5, [
                line(0, 5)
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
        input: "\
-----
Title
-----
",
        rule: Rule::title,
        tokens: [
            title(0, 17, [
                adornments(0, 5),
                line(6, 12),
            ])
        ]
    };
}

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

#[test]
fn admonitions() {
    parses_to! {
        parser: RstParser,
        input: "\
.. note:: In line
   Next line
.. admonition::

   Just next line

.. danger:: Just this line
",
        rule: Rule::document,
        tokens: [
            admonition(0, 31, [
                admonition_type(3, 7),
                line(9, 18),
                paragraph(21, 31, [ line(21, 31) ]),
            ]),
            admonition(31, 66, [
                admonition_type(34, 44),
                paragraph(51, 66, [ line(51, 66) ]),
            ]),
            admonition(67, 94, [
                admonition_type(70, 76),
                line(78, 94),
            ]),
        ]
    };
}

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

use pest::consumes_to;
use pest::parses_to;

use crate::pest_rst::{RstParser, Rule};

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
fn emph_only() {
    parses_to! {
        parser: RstParser,
        input: "*emphasis*",
        rule: Rule::emph_outer,
        tokens: [
            emph(1, 9, [str_nested(1, 9)])
        ]
    };
}

#[test]
fn emph() {
    parses_to! {
        parser: RstParser,
        input: "line *with markup*\n",
        rule: Rule::paragraph,
        tokens: [
            paragraph(0, 18, [
                str(0, 5),
                emph(6, 17, [str_nested(6, 17)]),
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

#[allow(clippy::cognitive_complexity)]
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
fn inline_code_literal_with_underscore() {
    parses_to! {
        parser: RstParser,
        input: "``NAME_WITH_UNDERSCORE``",
        rule: Rule::inline,
        tokens: [
            literal(2, 22),
        ]
    };
}

#[allow(clippy::cognitive_complexity)]
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

#[test]
fn blockquote_no_attribution() {
    parses_to! {
        parser: RstParser,
        input: "\
Paragraph.

   Block quote.

Paragraph.
",
        rule: Rule::document,
        tokens: [
            paragraph(0, 10, [ str(0, 10) ]),
            block_quote(12, 29, [
                paragraph(15, 27, [ str(15, 27) ]),
            ]),
            paragraph(29, 39, [ str(29, 39) ]),
        ]
    };
}

#[test]
fn blockquote_fake_attribution() {
    parses_to! {
        parser: RstParser,
        input: "\
Paragraph.

   -- Not an attribution

Paragraph.
",
        rule: Rule::document,
        tokens: [
            paragraph(0, 10, [ str(0, 10) ]),
            block_quote(12, 38, [
                paragraph(15, 36, [ str(15, 36) ]),
            ]),
            paragraph(38, 48, [ str(38, 48) ]),
        ]
    };
}

#[test]
fn blockquote_simple() {
    parses_to! {
        parser: RstParser,
        input: "\
Paragraph.

   Block quote.

   -- Attribution

Paragraph.
",
        rule: Rule::document,
        tokens: [
            paragraph(0, 10, [ str(0, 10) ]),
            block_quote(12, 47, [
                paragraph(15, 27, [ str(15, 27) ]),
                attribution(29, 47, [ line(35, 47, [ str(35, 46) ]) ]),
            ]),
            paragraph(48, 58, [ str(48, 58) ]),
        ]
    };
}

#[test]
fn blockquote_multiline_attribution() {
    parses_to! {
        parser: RstParser,
        input: "\
Paragraph.

   Block quote.

   --Attribution line one
   and line two

Paragraph.
",
        rule: Rule::document,
        tokens: [
            paragraph(0, 10, [ str(0, 10) ]),
            block_quote(12, 71, [
                paragraph(15, 27, [ str(15, 27) ]),
                attribution(29, 71, [
                    line(34, 55, [ str(34, 54) ]),
                    line(55, 71, [ str(55, 70) ])
                ]),
            ]),
            paragraph(72, 82, [ str(72, 82) ]),
        ]
    };
}

#[test]
fn blockquote_multiline_attribution_2() {
    parses_to! {
        parser: RstParser,
        input: "\
Paragraph.

   -- Invalid attribution

   Block quote.

   -- Attribution line one
      and line two

Paragraph.
",
        rule: Rule::document,
        tokens: [
            paragraph(0, 10, [ str(0, 10) ]),
            block_quote(12, 102, [
                paragraph(15, 37, [ str(15, 37) ]),
                paragraph(42, 54, [ str(42, 54) ]),
                attribution(56, 102, [
                    line(62, 83, [ str(62, 82) ]),
                    line(83, 102, [ str(83, 101) ])
                ])
            ]),
            paragraph(103, 113, [ str(103, 113) ]),
        ]
    };
}

#[test]
fn blockquote_back_to_back() {
    parses_to! {
        parser: RstParser,
        input: "\
Paragraph.

   Block quote 1.

   -- Attribution 1

   Block quote 2.

   --Attribution 2

Paragraph.
",
        rule: Rule::document,
        tokens: [
            paragraph(0, 10, [ str(0, 10) ]),
            block_quote(12, 51, [
                paragraph(15, 29, [ str(15, 29) ]),
                attribution(31, 51, [ line(37, 51, [ str(37, 50) ]) ]),
            ]),
            block_quote(52, 90, [
                paragraph(55, 69, [ str(55, 69) ]),
                attribution(71, 90, [ line(76, 90, [ str(76, 89) ]) ]),
            ]),
            paragraph(91, 101, [ str(91, 101) ]),
        ]
    };
}
#[test]

fn block_quote_directive() {
    parses_to! {
        parser: RstParser,
        input: "\
.. epigraph::

   Single line

The end
",
        rule: Rule::document,
        tokens: [
            block_quote_directive(0, 31, [
                block_quote_type(3, 11),
                paragraph(18, 29, [ str(18, 29) ]),
            ]),
            paragraph(31, 38, [ str(31, 38) ]),
        ]
    };
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn literal_block() {
    parses_to! {
        parser: RstParser,
        input: "\
::

   print('x')

   # second line

The end
",
        rule: Rule::document,
        tokens: [
            literal_block(0, 36, [
                literal_lines(7, 36, [
                    literal_line(7, 18),
                    literal_line_blank(18, 19),
                    literal_line(22, 36),
                ]),
            ]),
            paragraph(37, 44, [ str(37, 44) ]),
        ]
    };
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn code_directive() {
    parses_to! {
        parser: RstParser,
        input: "\
.. code::

   Single line

.. code-block:: python

   print('x')

   # second line

The end
",
        rule: Rule::document,
        tokens: [
            code_directive(0, 26, [
                literal_lines(14, 26, [ literal_line(14, 26) ]),
            ]),
            code_directive(27, 83, [
                source(43, 49),
                literal_lines(54, 83, [
                    literal_line(54, 65),
                    literal_line_blank(65, 66),
                    literal_line(69, 83),
                ]),
            ]),
            paragraph(84, 91, [ str(84, 91) ]),
        ]
    };
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn raw() {
    parses_to! {
        parser: RstParser,
        input: "\
.. raw:: html

   hello <span>world</span>

.. raw:: html

   hello <pre>world

   parse</pre> this

The end
",
        rule: Rule::document,
        tokens: [
            raw_directive(0, 43, [
                raw_output_format(9, 13),
                raw_block(18, 43, [
                    raw_line(18, 43),
                ]),
            ]),
            raw_directive(44, 100, [
                raw_output_format(53, 57),
                raw_block(62, 100, [
                    raw_line(62, 79),
                    raw_line_blank(79, 80),
                    raw_line(83, 100),
                ]),
            ]),
            paragraph(101, 108, [ str(101, 108) ]),
        ]
    };
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn comments() {
    parses_to! {
        parser: RstParser,
        input: "\
.. This is a comment

..
   This as well.

..
   _so: is this!
..
   [and] this!
..
   this:: too!
..
   |even| this:: !
.. With a title..

   and a blank line...
   followed by a non-blank line

   and another one.
..
.. Comments can also be
   run-in like this
",
        rule: Rule::document,
        tokens: [
            block_comment(0, 22, [
                comment_line(3, 21),
            ]),
            block_comment(22, 43, [
                comment_line(28, 42),
            ]),
            block_comment(43, 63, [
                comment_line(49, 63),
            ]),
            block_comment(63, 81, [
                comment_line(69, 81),
            ]),
            block_comment(81, 99, [
                comment_line(87, 99),
            ]),
            block_comment(99, 121, [
                comment_line(105, 121),
            ]),
            block_comment(121, 216, [
                comment_line(124, 139),
                comment_line_blank(139, 140),
                comment_line(143, 163),
                comment_line(166, 195),
                comment_line_blank(195, 196),
                comment_line(199, 216),
            ]),
            block_comment(216, 219),
            block_comment(219, 263, [
                comment_line(222, 243),
                comment_line(246, 263),
            ]),
        ]
    };
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn substitutions() {
    parses_to! {
        parser: RstParser,
        input: "\
A |subst| in-line

.. |subst| replace:: substitution
.. |subst2| replace:: it can also
   be hanging
",
        rule: Rule::document,
        tokens: [
            paragraph(0, 17, [
                str(0, 2),
                substitution_name(3, 8),
                str(9, 17),
            ]),
            substitution_def(19, 52, [
                substitution_name(23, 28),
                replace(30, 52, [ paragraph(40, 52, [str(40, 52)]) ]),
            ]),
            substitution_def(53, 101, [
                substitution_name(57, 63),
                replace(65, 101, [ paragraph(75, 101, [
                    str(75, 86), ws_newline(86, 87),
                    str(88, 100),
                ]) ]),
            ]),
        ]
    };
}

#[test]
fn substitution_in_literal() {
    parses_to! {
        parser: RstParser,
        input: "Just ``|code|``, really ``*code* |only|``",
        rule: Rule::document,
        tokens: [
            paragraph(0, 41, [
                str(0, 5),
                literal(7, 13),
                str(15, 24),
                literal(26, 39),
            ]),
        ]
    };
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn substitution_image() {
    parses_to! {
        parser: RstParser,
        input: "\
.. |subst| image:: thing.png
   :target: foo.html
",
        rule: Rule::document,
        tokens: [
            substitution_def(0, 50, [
                substitution_name(4, 9),
                image(11, 50, [
                    line(18, 29, [ str(18, 28) ]),
                    image_option(32, 50, [
                        image_opt_name(33, 39),
                        line(40, 50, [ str(40, 49) ]),
                    ]),
                ]),
            ]),
        ]
    };
}

#[allow(clippy::cognitive_complexity)]
#[test]
fn substitution_image_with_alt() {
    parses_to! {
        parser: RstParser,
        input: "\
.. |subst| image:: thing.png
   :alt: A thing
   :target: foo.html
",
        rule: Rule::document,
        tokens: [
            substitution_def(0, 67, [
                substitution_name(4, 9),
                image(11, 67, [
                    line(18, 29, [ str(18, 28) ]),
                    image_option(32, 46, [
                        image_opt_name(33, 36),
                        line(37, 46, [ str(37, 45) ]),
                    ]),
                    image_option(49, 67, [
                        image_opt_name(50, 56),
                        line(57, 67, [ str(57, 66) ]),
                    ]),
                ]),
            ]),
        ]
    };
}

// TODO: test images

#[allow(clippy::cognitive_complexity)]
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
                        str(34, 43), ws_newline(43, 44),
                        str(47, 58), ws_newline(58, 59),
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

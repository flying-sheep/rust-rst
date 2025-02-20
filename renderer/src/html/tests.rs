use pretty_assertions::assert_eq;

use rst_parser::parse;

use crate::html::render_html;

fn check_renders_to(rst: &str, expected: &str) {
    println!("Rendering:\n{}\n---", rst);
    let doc = parse(rst).expect("Cannot parse");
    let mut result_data: Vec<u8> = vec![];
    render_html(&doc, &mut result_data, false).expect("Render error");
    let result = String::from_utf8(result_data).expect("Could not decode");
    assert_eq!(result.as_str().trim(), expected);
    println!("{}", expected);
}

#[test]
fn simple_string() {
    check_renders_to("Simple String", "<p>Simple String</p>");
}

#[test]
fn simple_string_with_markup() {
    check_renders_to(
        "Simple String with *emph* and **strong**",
        "<p>Simple String with <em>emph</em> and <strong>strong</strong></p>",
    );
}

#[test]
fn inline_literal() {
    check_renders_to(
        "Simple String with an even simpler ``inline literal``",
        "<p>Simple String with an even simpler <code>inline literal</code></p>",
    );
}

/*
#[test]
fn test_reference_anonymous() {
    check_renders_to("\
A simple `anonymous reference`__

__ http://www.test.com/test_url
", "\
<p>A simple <a href=\"http://www.test.com/test_url\">anonymous reference</a></p>\
");
}
*/

#[test]
fn two_paragraphs() {
    check_renders_to(
        "One paragraph.\n\nTwo paragraphs.",
        "<p>One paragraph.</p>\n<p>Two paragraphs.</p>",
    );
}

#[test]
fn named_reference() {
    check_renders_to(
        "\
A simple `named reference`_ with stuff in between the
reference and the target.

.. _`named reference`: http://www.test.com/test_url
",
        "\
<p>A simple <a href=\"http://www.test.com/test_url\">named reference</a> with stuff in between the \
reference and the target.</p>\
",
    );
}

#[test]
fn standalone_hyperlinks() {
    check_renders_to(
        "\
Some http://url and a not_url_scheme:foo that is not supposed to be an url.
",
        "\
<p>Some <a href=\"http://url/\">http://url</a> and a not_url_scheme:foo that is not supposed to be an url.</p>\
",
    );
}

#[test]
fn substitution() {
    check_renders_to(
        "\
A |subst|.

.. |subst| replace:: text substitution
",
        "<p>A text substitution.</p>",
    );
}

#[test]
fn not_substitution_literal() {
    check_renders_to(
        "\
hello ``foo.map(|world| world + 42)``

.. |world| replace:: something different

.. code::

   foo.map(|world| world + 42)

::

    hay! |x|
",
        "<p>hello <code>foo.map(|world| world + 42)</code></p>
<pre><code>foo.map(|world| world + 42)\n</code></pre>
<pre>hay! |x|\n</pre>",
    );
}

/*
#[test]
fn test_section_hierarchy() {
    check_renders_to("\
+++++
Title
+++++

Subtitle
========

Some stuff

Section
-------

Some more stuff

Another Section
...............

And even more stuff
", "\
<p>Some stuff</p>
<section id=\"section\">
<h1>Section</h1>
<p>Some more stuff</p>
<section id=\"another-section\">
<h2>Another Section</h2>
<p>And even more stuff</p>
</section>
</section>\
");
}

#[test]
fn test_docinfo_title() {
    check_renders_to("\
+++++
Title
+++++

:author: me

Some stuff
", "\
<main id=\"title\">
<h1 class=\"title\">Title</h1>
<dl class=\"docinfo simple\">
<dt class=\"author\">Author</dt>
<dd class=\"author\"><p>me</p></dd>
</dl>
<p>Some stuff</p>
</main>\
");
}
*/

#[test]
fn section_hierarchy() {
    check_renders_to(
        "\
+++++
Title
+++++

Not A Subtitle
==============

Some stuff

Section
-------

Some more stuff

Another Section
...............

And even more stuff
",
        "\
<section id=\"title\">
<h1>Title</h1>
<section id=\"not-a-subtitle\">
<h2>Not A Subtitle</h2>
<p>Some stuff</p>
<section id=\"section\">
<h3>Section</h3>
<p>Some more stuff</p>
<section id=\"another-section\">
<h4>Another Section</h4>
<p>And even more stuff</p>
</section>
</section>
</section>
</section>\
",
    );
}

#[test]
fn many_sections() {
    check_renders_to(
        "\
+++++++++
heading 1
+++++++++

heading 2
=========

First stuff

heading 2a
----------

First detail

heading 3
=========

Second stuff

heading 3a
----------

Second detail
",
        "\
<section id=\"heading-1\">
<h1>heading 1</h1>
<section id=\"heading-2\">
<h2>heading 2</h2>
<p>First stuff</p>
<section id=\"heading-2a\">
<h3>heading 2a</h3>
<p>First detail</p>
</section>
</section>
<section id=\"heading-3\">
<h2>heading 3</h2>
<p>Second stuff</p>
<section id=\"heading-3a\">
<h3>heading 3a</h3>
<p>Second detail</p>
</section>
</section>
</section>\
",
    );
}

#[test]
fn bullet_list() {
    check_renders_to(
        "\
* bullet
* list
",
        "\
<ul>
<li><p>bullet</p></li>
<li><p>list</p></li>
</ul>\
",
    );
}

/*
#[test]
fn test_table() {
    check_renders_to("\
.. table::
   :align: right

   +-----+-----+
   |  1  |  2  |
   +-----+-----+
   |  3  |  4  |
   +-----+-----+
", "\
<table class=\"align-right\">
<colgroup>
<col style=\"width: 50%%\" />
<col style=\"width: 50%%\" />
</colgroup>
<tbody>
<tr><td><p>1</p></td>
<td><p>2</p></td>
</tr>
<tr><td><p>3</p></td>
<td><p>4</p></td>
</tr>
</tbody>
</table>\
");
}
*/

#[test]
fn code() {
    check_renders_to(
        "\
.. code:: python

   def foo():
       print('Hi!')

       # comment
",
        "\
<pre><code class=\"language-python\">def foo():
    print('Hi!')

    # comment
</code></pre>\
",
    );
}

#[test]
fn raw_html() {
    check_renders_to(
        "\
.. raw:: html

   hello <span>world</span>
   <p>paragraph

   paragraph</p>

after

.. raw:: something_else

   into a black hole this goes
",
        "\
hello <span>world</span>
<p>paragraph

paragraph</p>

<p>after</p>\
",
    );
}

#[test]
fn comments() {
    check_renders_to(
        "\
.. Run-in
   comment

..

    block-like

    with blank lines
",
        "\
<!--Run-in
comment
-->
<!--block-like

with blank lines
-->\
",
    );
}

/*
#[test]
fn test_field_list() {
    check_renders_to("\
Not a docinfo.

:This: .. _target:

       is
:a:
:simple:
:field: list
", "\
<p>Not a docinfo.</p>
<dl class=\"field-list\">
<dt>This</dt>
<dd><p id=\"target\">is</p>
</dd>
<dt>a</dt>
<dd><p></p></dd>
<dt>simple</dt>
<dd><p></p></dd>
<dt>field</dt>
<dd><p>list</p>
</dd>
</dl>\
");
}
*/

/*
#[test]
fn test_field_list_long() {
    check_renders_to("\
Not a docinfo.

:This is: a
:simple field list with loooong field: names
", "\
<p>Not a docinfo.</p>
<dl class=\"field-list\">
<dt>This is</dt>
<dd><p>a</p>
</dd>
<dt>simple field list with loooong field</dt>
<dd><p>names</p>
</dd>
</dl>\
");
}
*/

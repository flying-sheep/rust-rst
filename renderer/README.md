`rst_renderer`
==============

Part of the [`rst`][rst] crate family.
This crate contains the HTML renderer (which supports most of what the parser supports),
as well as the broken XML and JSON renderers.
Suggestions and PRs welcome on how to get them right!

```rust
let document = Document::with_children(vec![...]); // or rst_parser::parse()
let stream = std::io::stdout();
let standalone = true;  // wrap in <!doctype html><html></html>
render_html(document, stream, standalone);
```

[rst]: https://github.com/flying-sheep/rust-rst/#readme

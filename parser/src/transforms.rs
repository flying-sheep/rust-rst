/*
http://docutils.sourceforge.net/docs/ref/rst/restructuredtext.html#hyperlink-targets

Links can have internal or external targets.
In the source, targets look like:

    .. targetname1:
    .. targetname2:

    some paragraph or list item or so

or:

    .. targetname1:
    .. targetname2: https://link

There’s also anonymous links and targets without names.

TODO: continue documenting how it’s done via https://repo.or.cz/docutils.git/blob/HEAD:/docutils/docutils/transforms/references.py
*/

mod references;
mod transform;
mod visit;

use document_tree::Document;

use self::references::TargetCollector;
pub use self::transform::Transform;
pub use self::visit::Visit;

#[must_use]
pub fn resolve_references(doc: Document) -> Document {
    let mut references = TargetCollector::default();
    references.visit(&doc);
    references.transform(doc)
}

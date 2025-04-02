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
mod visit;
mod visit_mut;

use document_tree::{Document, HasChildren as _};

use self::references::TargetCollector;
pub use self::visit::Visit;
pub use self::visit_mut::VisitMut;

pub fn resolve_references(mut doc: Document) -> Document {
    let mut references = TargetCollector::default();
    for c in doc.children() {
        references.visit_structural_sub_element(c);
    }
    for c in doc.children_mut() {
        references.visit_structural_sub_element_mut(c);
    }
    doc
}

mod references;
mod transform;
mod visit;

use document_tree::Document;

use self::references::TargetCollector;
pub use self::transform::{IteratorMaker, Transform};
pub use self::visit::Visit;

#[must_use]
pub fn resolve_references(doc: Document) -> Document {
    let mut references = TargetCollector::default();
    references.visit(&doc);
    references.transform(doc)
}

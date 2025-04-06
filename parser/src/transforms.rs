mod standard;
mod transform;
mod visit;

use document_tree::Document;

use self::standard::pass_2;
pub use self::transform::{IteratorMaker, Transform};
pub use self::visit::Visit;

#[must_use]
pub fn standard_transform(doc: Document) -> Document {
    // TODO: pass 1 to add IDs
    pass_2(doc)
}

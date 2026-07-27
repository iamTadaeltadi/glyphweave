//! Supported OpenType GPOS positioning retention and remapping.

mod anchor;
mod cursive_positioning;
mod value_format;
mod value_record;

pub use anchor::{Anchor, AnchorKind};
pub use cursive_positioning::{CursivePositioning, EntryExit};
pub use value_format::ValueFormat;
pub use value_record::ValueRecord;

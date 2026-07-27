//! Supported OpenType GPOS positioning retention and remapping.

mod anchor;
mod cursive_positioning;
mod mark_to_base_positioning;
mod mark_to_ligature_positioning;
mod pair_positioning;
mod single_positioning;
mod value_format;
mod value_record;

pub use anchor::{Anchor, AnchorKind};
pub use cursive_positioning::{CursivePositioning, EntryExit};
pub use mark_to_base_positioning::{
    BaseRecord, MarkBaseAttachment, MarkRecord, MarkToBasePositioning,
};
pub use mark_to_ligature_positioning::{
    LigatureAttachment, MarkLigaturePositioning, MarkLigatureRecord,
};
pub use pair_positioning::{PairAdjustment, PairPositioning, PairValue};
pub use single_positioning::SinglePositioning;
pub use value_format::ValueFormat;
pub use value_record::ValueRecord;

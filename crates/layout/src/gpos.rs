//! Supported OpenType GPOS positioning retention and remapping.

mod anchor;
mod chained_context_positioning;
mod context_positioning;
mod cursive_positioning;
mod lookup;
mod lookup_list;
mod mark_to_base_positioning;
mod mark_to_ligature_positioning;
mod mark_to_mark_positioning;
mod pair_positioning;
mod remap;
mod remapped;
mod single_positioning;
mod subtable;
mod value_format;
mod value_record;

pub use anchor::{Anchor, AnchorKind};
pub use chained_context_positioning::ChainedContextPositioning;
pub use context_positioning::{
    ContextPositioning, ContextPositioningRecord, ContextPositioningRule,
};
pub use cursive_positioning::{CursivePositioning, EntryExit};
pub use lookup::GposLookup;
pub use lookup_list::GposLookupList;
pub use mark_to_base_positioning::{
    BaseRecord, MarkBaseAttachment, MarkRecord, MarkToBasePositioning,
};
pub use mark_to_ligature_positioning::{
    LigatureAttachment, MarkLigaturePositioning, MarkLigatureRecord,
};
pub use mark_to_mark_positioning::{Mark1Record, Mark2Record, MarkToMarkPositioning};
pub use pair_positioning::{PairAdjustment, PairPositioning, PairValue};
pub use remapped::{RemappedGpos, RemappedGposLookup};
pub use single_positioning::SinglePositioning;
pub use subtable::GposSubtable;
pub use value_format::ValueFormat;
pub use value_record::ValueRecord;

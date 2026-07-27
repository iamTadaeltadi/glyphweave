//! Supported OpenType GPOS positioning retention and remapping.

mod pair_positioning;
mod single_positioning;
mod value_format;
mod value_record;

pub use pair_positioning::{PairAdjustment, PairPositioning, PairValue};
pub use single_positioning::SinglePositioning;
pub use value_format::ValueFormat;
pub use value_record::ValueRecord;

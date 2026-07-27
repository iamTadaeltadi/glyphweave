//! Supported OpenType GPOS positioning retention and remapping.

mod single_positioning;
mod value_format;
mod value_record;

pub use single_positioning::SinglePositioning;
pub use value_format::ValueFormat;
pub use value_record::ValueRecord;

//! Cohesive rendering groups for layout diagnostics.

use core::fmt;

use super::LayoutCause;

mod gsub;
mod lookup;

pub(super) fn lookup(cause: &LayoutCause, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    lookup::write(cause, formatter).expect("Lookup cause routed to Lookup renderer")
}

pub(super) fn gsub(cause: &LayoutCause, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    gsub::write(cause, formatter).expect("GSUB cause routed to GSUB renderer")
}

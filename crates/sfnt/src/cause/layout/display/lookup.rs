//! Shared Lookup and extension diagnostic rendering.

use core::fmt;

use super::super::LayoutCause;

pub(super) fn write(
    cause: &LayoutCause,
    formatter: &mut fmt::Formatter<'_>,
) -> Option<fmt::Result> {
    let result = match *cause {
        LayoutCause::InvalidLookupType { table, lookup_type } => {
            write!(formatter, "invalid {table} lookup type {lookup_type}")
        }
        LayoutCause::UnexpectedLayoutTableKind { expected, actual } => {
            write!(formatter, "expected {expected} Lookup, found {actual} Lookup")
        }
        LayoutCause::EmptyLookupSubtables { table, lookup_type } => {
            write!(formatter, "{table} lookup type {lookup_type} has no subtables")
        }
        LayoutCause::ReservedLookupFlags { flags } => {
            write!(formatter, "lookup flags {flags:#06x} set reserved bits")
        }
        LayoutCause::UnsupportedExtensionFormat { table, format } => {
            write!(formatter, "unsupported {table} extension format {format}")
        }
        LayoutCause::RecursiveExtensionLookupType { table, lookup_type } => write!(
            formatter,
            "{table} extension cannot target lookup type {lookup_type}"
        ),
        LayoutCause::MismatchedExtensionLookupTypes {
            table,
            expected,
            actual,
        } => write!(
            formatter,
            "{table} extension lookup type {actual} differs from preceding type {expected}"
        ),
        _ => return None,
    };
    Some(result)
}

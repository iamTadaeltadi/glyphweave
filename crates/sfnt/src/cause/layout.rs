//! OpenType layout-specific diagnostic causes.

use core::fmt;

/// A format or invariant failure in a shared OpenType layout structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutCause {
    /// A Coverage table used an unknown format selector.
    UnsupportedCoverageFormat {
        /// The unrecognized selector.
        format: u16,
    },
    /// A Class Definition table used an unknown format selector.
    UnsupportedClassDefinitionFormat {
        /// The unrecognized selector.
        format: u16,
    },
}

impl fmt::Display for LayoutCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnsupportedCoverageFormat { format } => {
                write!(f, "unsupported Coverage format {format}")
            }
            Self::UnsupportedClassDefinitionFormat { format } => {
                write!(f, "unsupported Class Definition format {format}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutCause;

    #[test]
    fn unsupported_coverage_format_names_the_selector() {
        let cause = LayoutCause::UnsupportedCoverageFormat { format: 9 };
        assert_eq!(cause.to_string(), "unsupported Coverage format 9");
    }

    #[test]
    fn unsupported_class_definition_format_names_the_selector() {
        let cause = LayoutCause::UnsupportedClassDefinitionFormat { format: 7 };
        assert_eq!(
            cause.to_string(),
            "unsupported Class Definition format 7"
        );
    }
}

//! Span-aware Diagnostics: typed pipeline failures and warnings.

use core::fmt;

use crate::cause::Cause;
use crate::span::ByteSpan;

/// The pipeline stage a Diagnostic originates from.
///
/// The set of stages is fixed by the pass-based architecture but marked `#[non_exhaustive]`
/// so that adding a future stage never breaks an external match.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Stage {
    /// Bounded decoding of SFNT containers, tables, outlines, and layout structures.
    Decode,
    /// Resolution of Unicode requests into an initial Glyph Set.
    Resolve,
    /// Transitive Glyph Closure over components and substitutions.
    Closure,
    /// Construction of the dense New Glyph ID Remap.
    Remap,
    /// Table Pass rewriting of retained tables.
    Rewrite,
    /// Canonical SFNT emission.
    Emit,
    /// Independent Verification of an emitted Font Artifact.
    Verify,
}

impl Stage {
    /// The lowercase stable name of the stage, used when rendering a Diagnostic.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Decode => "decode",
            Self::Resolve => "resolve",
            Self::Closure => "closure",
            Self::Remap => "remap",
            Self::Rewrite => "rewrite",
            Self::Emit => "emit",
            Self::Verify => "verify",
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Whether a Diagnostic stops the pipeline or only annotates its output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Severity {
    /// The pipeline cannot proceed.
    Failure,
    /// The pipeline can proceed, but the result deviates from an ideal subset.
    Warning,
}

/// A typed failure or warning tied to a pipeline [`Stage`] and, when the failure originates in
/// input, a [`ByteSpan`].
///
/// A `Diagnostic` is the single error type the computational crates return. It is cheap to copy
/// and carries enough structure for the CLI to render a precise, byte-located message without
/// re-deriving where the failure happened.
///
/// ```
/// use glyphweave_sfnt::{ByteSpan, Cause, Diagnostic, Stage};
///
/// let span = ByteSpan::sized(4, 2).unwrap();
/// let diagnostic = Diagnostic::at(Stage::Decode, span, Cause::UnexpectedEnd {
///     needed: 2,
///     available: 0,
/// });
/// assert!(diagnostic.is_failure());
/// assert_eq!(diagnostic.to_string(), "decode at 4..6: needed 2 byte(s) but only 0 remained");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    stage: Stage,
    severity: Severity,
    span: Option<ByteSpan>,
    cause: Cause,
}

impl Diagnostic {
    /// Creates a failure Diagnostic tied to a source span.
    #[must_use]
    pub const fn at(stage: Stage, span: ByteSpan, cause: Cause) -> Self {
        Self {
            stage,
            severity: Severity::Failure,
            span: Some(span),
            cause,
        }
    }

    /// Creates a failure Diagnostic that is not tied to a specific span.
    #[must_use]
    pub const fn failure(stage: Stage, cause: Cause) -> Self {
        Self {
            stage,
            severity: Severity::Failure,
            span: None,
            cause,
        }
    }

    /// Creates a warning Diagnostic tied to a source span.
    #[must_use]
    pub const fn warn_at(stage: Stage, span: ByteSpan, cause: Cause) -> Self {
        Self {
            stage,
            severity: Severity::Warning,
            span: Some(span),
            cause,
        }
    }

    /// The stage the Diagnostic originates from.
    #[must_use]
    pub const fn stage(self) -> Stage {
        self.stage
    }

    /// The severity of the Diagnostic.
    #[must_use]
    pub const fn severity(self) -> Severity {
        self.severity
    }

    /// The source span, present when the Diagnostic originates in input bytes.
    #[must_use]
    pub const fn span(self) -> Option<ByteSpan> {
        self.span
    }

    /// The domain-specific cause.
    #[must_use]
    pub const fn cause(self) -> Cause {
        self.cause
    }

    /// Returns `true` when the Diagnostic is a [`Severity::Failure`].
    #[must_use]
    pub const fn is_failure(self) -> bool {
        matches!(self.severity, Severity::Failure)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => write!(f, "{} at {}: {}", self.stage, span, self.cause),
            None => write!(f, "{}: {}", self.stage, self.cause),
        }
    }
}

impl std::error::Error for Diagnostic {}

/// The result of a fallible decode, carrying a [`Diagnostic`] on failure.
pub type Decoded<T> = Result<T, Diagnostic>;

#[cfg(test)]
mod tests {
    use super::{Diagnostic, Severity, Stage};
    use crate::cause::Cause;
    use crate::span::ByteSpan;

    #[test]
    fn stage_names_are_stable_and_lowercase() {
        let stages = [
            (Stage::Decode, "decode"),
            (Stage::Resolve, "resolve"),
            (Stage::Closure, "closure"),
            (Stage::Remap, "remap"),
            (Stage::Rewrite, "rewrite"),
            (Stage::Emit, "emit"),
            (Stage::Verify, "verify"),
        ];
        for (stage, name) in stages {
            assert_eq!(stage.name(), name);
            assert_eq!(stage.to_string(), name);
        }
    }

    #[test]
    fn failure_carries_stage_span_and_cause() {
        let span = ByteSpan::sized(4, 2).unwrap();
        let cause = Cause::UnexpectedEnd {
            needed: 2,
            available: 0,
        };
        let diagnostic = Diagnostic::at(Stage::Decode, span, cause);
        assert_eq!(diagnostic.stage(), Stage::Decode);
        assert_eq!(diagnostic.severity(), Severity::Failure);
        assert_eq!(diagnostic.span(), Some(span));
        assert_eq!(diagnostic.cause(), cause);
        assert!(diagnostic.is_failure());
    }

    #[test]
    fn spanless_failure_omits_a_location() {
        let diagnostic =
            Diagnostic::failure(Stage::Remap, Cause::SpanOutOfBounds { end: 3, length: 1 });
        assert_eq!(diagnostic.span(), None);
        assert_eq!(
            diagnostic.to_string(),
            "remap: span ends at 3 but only 1 byte(s) are available"
        );
    }

    #[test]
    fn warnings_are_not_failures() {
        let span = ByteSpan::empty_at(0);
        let diagnostic = Diagnostic::warn_at(
            Stage::Verify,
            span,
            Cause::UnexpectedEnd {
                needed: 1,
                available: 0,
            },
        );
        assert_eq!(diagnostic.severity(), Severity::Warning);
        assert!(!diagnostic.is_failure());
    }

    #[test]
    fn display_locates_failures_with_a_span() {
        let span = ByteSpan::between(8, 12).unwrap();
        let diagnostic = Diagnostic::at(
            Stage::Decode,
            span,
            Cause::UnexpectedEnd {
                needed: 4,
                available: 1,
            },
        );
        assert_eq!(
            diagnostic.to_string(),
            "decode at 8..12: needed 4 byte(s) but only 1 remained"
        );
    }

    #[test]
    fn implements_the_error_trait() {
        let diagnostic = Diagnostic::failure(
            Stage::Emit,
            Cause::UnexpectedEnd {
                needed: 1,
                available: 0,
            },
        );
        let boxed: Box<dyn std::error::Error> = Box::new(diagnostic);
        assert_eq!(
            boxed.to_string(),
            "emit: needed 1 byte(s) but only 0 remained"
        );
    }
}

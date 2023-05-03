//! Portability behavior for optional differential executables.

mod support;

use support::reference_tool::subset;

#[test]
fn unavailable_reference_tool_skips_without_an_artifact() {
    assert_eq!(subset("glyphweave-reference-tool-does-not-exist", 5), None);
}

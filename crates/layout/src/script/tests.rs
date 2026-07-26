use glyphweave_sfnt::Tag;

use super::Script;
use crate::LanguageSystem;

#[test]
fn sorts_language_systems_by_tag() {
    let script = Script::new(
        None,
        [
            (Tag::new(*b"URD "), LanguageSystem::new(None, [2])),
            (Tag::new(*b"DEU "), LanguageSystem::new(None, [1])),
        ],
    );
    let tags: Vec<_> = script.languages().iter().map(|(tag, _)| *tag).collect();
    assert_eq!(tags, [Tag::new(*b"DEU "), Tag::new(*b"URD ")]);
}

#[test]
fn keeps_the_first_language_system_for_a_repeated_tag() {
    let script = Script::new(
        None,
        [
            (Tag::new(*b"DEU "), LanguageSystem::new(Some(1), [])),
            (Tag::new(*b"DEU "), LanguageSystem::new(Some(9), [])),
        ],
    );
    assert_eq!(script.languages().len(), 1);
    assert_eq!(
        script
            .language(Tag::new(*b"DEU "))
            .unwrap()
            .required_feature(),
        Some(1)
    );
}

#[test]
fn looks_up_present_and_absent_tags() {
    let script = Script::new(
        Some(LanguageSystem::new(Some(0), [])),
        [(Tag::new(*b"DEU "), LanguageSystem::new(None, [5]))],
    );
    assert!(script.default_language().is_some());
    assert_eq!(
        script
            .language(Tag::new(*b"DEU "))
            .unwrap()
            .feature_indices(),
        &[5]
    );
    assert_eq!(script.language(Tag::new(*b"ZZZ ")), None);
}

#[test]
fn empty_script_has_no_default_or_languages() {
    let script = Script::default();
    assert!(script.is_empty());
    assert_eq!(script.default_language(), None);
    assert!(script.languages().is_empty());
}

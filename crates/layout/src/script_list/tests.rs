use glyphweave_sfnt::Tag;

use super::ScriptList;
use crate::Script;

#[test]
fn default_script_list_is_empty() {
    let scripts = ScriptList::default();
    assert!(scripts.is_empty());
    assert_eq!(scripts.len(), 0);
    assert!(scripts.tags().next().is_none());
}

#[test]
fn scripts_are_sorted_by_tag() {
    let scripts = ScriptList::new([
        (Tag::new(*b"latn"), Script::default()),
        (Tag::new(*b"arab"), Script::default()),
    ]);
    assert!(scripts.tags().eq([Tag::new(*b"arab"), Tag::new(*b"latn")]));
}

#[test]
fn first_repeated_script_tag_wins() {
    let first = Script::default();
    let second = Script::new(Some(crate::LanguageSystem::default()), []);
    let scripts = ScriptList::new([
        (Tag::new(*b"latn"), first.clone()),
        (Tag::new(*b"latn"), second),
    ]);
    assert_eq!(scripts.script(Tag::new(*b"latn")), Some(&first));
}

#[test]
fn lookup_distinguishes_present_and_absent_tags() {
    let scripts = ScriptList::new([(Tag::new(*b"DFLT"), Script::default())]);
    assert!(scripts.script(Tag::new(*b"DFLT")).is_some());
    assert_eq!(scripts.script(Tag::new(*b"latn")), None);
    assert_eq!(scripts.scripts().len(), 1);
}

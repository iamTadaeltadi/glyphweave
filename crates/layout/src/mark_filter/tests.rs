use super::MarkFilter;

#[test]
fn no_declaration_processes_all_marks() {
    assert_eq!(MarkFilter::resolve(false, None, None), MarkFilter::None);
}

#[test]
fn attachment_class_is_the_weakest_filter() {
    assert_eq!(
        MarkFilter::resolve(false, None, Some(7)),
        MarkFilter::AttachmentClass(7)
    );
}

#[test]
fn filtering_set_dominates_attachment_class() {
    assert_eq!(
        MarkFilter::resolve(false, Some(12), Some(7)),
        MarkFilter::Set(12)
    );
}

#[test]
fn ignoring_marks_dominates_every_declaration() {
    assert_eq!(
        MarkFilter::resolve(true, Some(12), Some(7)),
        MarkFilter::IgnoreAll
    );
}

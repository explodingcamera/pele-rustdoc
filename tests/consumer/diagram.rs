#[test]
fn vendor_macro_is_independent_of_the_call_site() {
    let rendered = pele_rustdoc::pele!("flowchart LR\n A --> B");
    assert!(rendered.contains("data:text/javascript,"));
    assert!(rendered.contains("flowchart LR\n A --> B"));
    assert!(!rendered.contains("cdn.jsdelivr.net"));
}

use pele_rustdoc::pele;

const INLINE: &str = pele!("flowchart LR\n A --> B");
const FILE: &str = pele!(include_str!("../src/architecture.mmd"));
const FRAMED: &str = pele!("flowchart LR\n A --> B", framed);
const LEFT: &str = pele!("flowchart LR\n A --> B", left);
const CENTER: &str = pele!("flowchart LR\n A --> B", center);
const RIGHT_FRAMED: &str = pele!("flowchart LR\n A --> B", right, framed);
const FRAMED_LEFT: &str = pele!("flowchart LR\n A --> B", framed, left);

#[test]
fn inline_diagram() {
    assert!(INLINE.contains("```mermaid"));
    assert!(INLINE.contains("margin-bottom:10px"));
    assert!(!INLINE.contains("A --&gt; B"));
    assert!(INLINE.contains("flowchart LR\n A --> B"));

    #[cfg(not(feature = "vendor"))]
    assert!(INLINE.contains("integrity=\"sha384-"));

    #[cfg(feature = "vendor")]
    {
        assert!(INLINE.contains("data:text/javascript,"));
        assert!(!INLINE.contains("cdn.jsdelivr.net"));
    }
}

#[test]
fn file_diagram() {
    assert!(FILE.contains("sequenceDiagram"));
}

#[test]
fn framed_diagram() {
    assert!(
        FRAMED.contains(
            "text-align:center;padding:14px;border-radius:var(--code-block-border-radius)"
        )
    );
    assert!(INLINE.contains("text-align:center;--pele-bg"));
}

#[test]
fn alignment() {
    assert!(LEFT.contains("text-align:left;--pele-bg"));
    assert!(CENTER.contains("text-align:center;--pele-bg"));
    assert!(RIGHT_FRAMED.contains("text-align:right;padding:14px"));
    assert!(FRAMED_LEFT.contains("text-align:left;padding:14px"));
}

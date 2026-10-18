# pele-rustdoc

[Documentation / Examples](https://docs.rs/pele-rustdoc)

Render Mermaid diagrams in rustdoc with [Pele](https://pele.run/).

```rust
use pele_rustdoc::pele;

/// Inline diagram
#[doc = pele!(r#"
flowchart LR
    A --> B --> C
"#)]
pub struct Engine;

/// Diagram from a file
#[doc = pele!(include_str!("architecture.mmd"))]
pub struct Architecture;
```

## Related crates

- [simple-mermaid](https://crates.io/crates/simple-mermaid)
- [merman-rustdoc](https://crates.io/crates/merman_rustdoc)

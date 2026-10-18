//! Mermaid diagrams in rustdoc, rendered by [Pele].
//!
//! Use [`pele!`] for inline Mermaid source or a file. Documentation works on
//! docs.rs without rustdoc flags or a custom wrapper.
//!
//! ```rust
//! use pele_rustdoc::pele;
//!
//! /// A simple flowchart
//! #[doc = pele!(r#"
//! flowchart LR
//!     A --> B --> C
//! "#)]
//! pub struct Engine;
//!
//! /// A diagram stored alongside this source file
//! #[doc = pele!(include_str!("architecture.mmd"))]
//! pub struct Architecture;
//! ```
//!
//! By default, diagrams load Pele from jsDelivr (about 252 KB gzipped, cached
//! across pages). The `vendor` feature embeds it instead (about 265 KB gzipped
//! per diagram). JavaScript is required in either case, and the Mermaid source
//! remains visible if it cannot run.
//!
//! [Pele]: https://pele.run/
#![doc = "\n## Examples\n\n#### Flowchart\n"]
#![doc = crate::pele!(r#"
flowchart LR
    subgraph Build["Build time"]
        Source["Mermaid source"] --> Macro["pele! macro"]
        Macro --> Html["rustdoc HTML"]
    end
    subgraph View["Page view"]
        Html --> Loader{"Pele available?"}
        Loader -->|Yes| Mount["pele.mount()"]
        Mount --> SVG["Responsive SVG"]
        Loader -->|No| Fallback["Readable Mermaid source"]
    end
    Theme["Rustdoc light / dark"] -.-> SVG
"#)]
#![doc = "\n#### Sequence diagram\n"]
#![doc = crate::pele!(include_str!("architecture.mmd"), left, framed)]
#![doc = "\n#### Sankey diagram\n"]
#![doc = crate::pele!(r#"
sankey-beta
Mermaid source,Flowcharts,40
Mermaid source,Sequences,35
Mermaid source,Charts,25
Flowcharts,SVG output,40
Sequences,SVG output,35
Charts,SVG output,25
"#)]
#![no_std]

/// Embeds a Mermaid diagram in a `#[doc = ...]` attribute.
///
/// Pass a string literal for an inline diagram, or use `include_str!` to read
/// a file. File paths are relative to the Rust file containing the macro.
///
/// ```rust
/// use pele_rustdoc::pele;
///
/// #[doc = pele!(r#"
/// flowchart LR
///     A --> B
/// "#)]
/// pub struct Inline;
///
/// #[doc = pele!(include_str!("architecture.mmd"), left, framed)]
/// pub struct FromFile;
/// ```
///
/// Choose `left`, `center` (the default), or `right` for alignment, and optionally
/// add `framed` for a rustdoc code-block background.
#[macro_export]
macro_rules! pele {
    ($source:expr $(, $option:ident)*) => {
        $crate::pele!(@options $source; center; ""; $($option),*)
    };
    (@options $source:expr; $align:ident; $frame:literal; ) => {
        $crate::pele!(@render $source, $align, $frame)
    };
    (@options $source:expr; $align:ident; $frame:literal; left $(, $rest:ident)*) => {
        $crate::pele!(@options $source; left; $frame; $($rest),*)
    };
    (@options $source:expr; $align:ident; $frame:literal; center $(, $rest:ident)*) => {
        $crate::pele!(@options $source; center; $frame; $($rest),*)
    };
    (@options $source:expr; $align:ident; $frame:literal; right $(, $rest:ident)*) => {
        $crate::pele!(@options $source; right; $frame; $($rest),*)
    };
    (@options $source:expr; $align:ident; $frame:literal; framed $(, $rest:ident)*) => {
        $crate::pele!(@options $source; $align; "padding:14px;border-radius:var(--code-block-border-radius);background:var(--code-block-background-color);"; $($rest),*)
    };
    (@render $source:expr, $align:ident, $frame:literal) => {
        concat!(
            "\n```mermaid\n",
            $source,
            "\n```\n",
            $crate::__pele_script!(),
            "<script>\n",
            "(function (script) {\n",
            "  var block = script.previousElementSibling.previousElementSibling;\n",
            "  var code = block && block.querySelector('code');\n",
            "  if (code && window.pele) {\n",
            "    var source = code.textContent;\n",
            "    var diagram = document.createElement('div');\n",
            "    diagram.className = 'pele';\n",
            "    diagram.style.cssText = 'text-align:",
            stringify!($align),
            ";",
            $frame,
            "--pele-bg:var(--main-background-color);",
            "--pele-fg:var(--main-color);--pele-muted:var(--main-color);",
            "--pele-line:var(--main-color);--pele-surface:var(--code-block-background-color);",
            "--pele-surface-alt:var(--code-block-background-color);",
            "--pele-border:var(--border-color);--pele-accent:var(--link-color);",
            "margin-bottom:10px';\n",
            "    block.replaceWith(diagram);\n",
            "    pele.mount(diagram, source);\n",
            "  }\n",
            "})(document.currentScript);\n",
            "</script>\n"
        )
    };
}

#[cfg(not(feature = "vendor"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __pele_script {
    () => {
        "<script src=\"https://cdn.jsdelivr.net/npm/pele@1.6.0/dist/pele.global.js\" integrity=\"sha384-4lHoXtKDt+x/E+htiTc83bpFWyREQzJCMudNQHirqKiCTW7Mx790I/5Q7criNZ1G\" crossorigin=\"anonymous\"></script>\n"
    };
}

#[cfg(feature = "vendor")]
#[doc(hidden)]
#[macro_export]
macro_rules! __pele_script {
    () => {
        concat!(
            "<script src=\"data:text/javascript,",
            include_str!("../vendor/pele.global.url"),
            "\"></script>\n"
        )
    };
}

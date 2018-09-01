//! Miscellaneous shell features

/// Abscissa logo to display when in debug mode
pub const LOGO_ASCII_ART: &str = r#"
    ┌┄┄┄┬┄┄┄┬┄┄┄┬┄┄┄┄┄┄┄┄┄┄┄┐
    │   │   │   │           │
    ├┄┄┄┼┄┄┄┼┄.━┿━.┄┄┄┄┄┄┄┄┄┤
    │   │   │╱  │  ╲        │
    ├┄┄┄┴┄┄┄╱┄┄┄┼┄┄┄╲┄┄┄┄┄┄┄┤
    │      ╱    │    ╲      │        Abscissa Application Framework
    ├┄┄┄┄┄╱┄┄┄┄┄┼┄┄┄┄┄╲┄┄┄┄┄┤        Version: {}
    │    ╱      │      ╲    │        https://github.com/iqlusioninc/abscissa
    │   ╱       │       ╲   │
    │  ╱        │        ╲  │
    │ ╱         │         ╲ │
 ━━━┷━━━━━━━━━━━┷━━━━━━━━━━━┷━━━
                ⁰
  a   b   s   c   i   s   s   a
"#;

/// Display framework information when in debug mode
#[cfg(feature = "logging")]
pub fn print_framework_info() {
    let framework_info = LOGO_ASCII_ART.replace("{}", env!("CARGO_PKG_VERSION"));
    debug!("{}", framework_info);
}

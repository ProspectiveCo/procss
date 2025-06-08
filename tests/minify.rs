// ┌───────────────────────────────────────────────────────────────────────────┐
// │                                                                           │
// │  ██████╗ ██████╗  ██████╗   Copyright (C) 2022, The Prospective Company   │
// │  ██╔══██╗██╔══██╗██╔═══██╗                                                │
// │  ██████╔╝██████╔╝██║   ██║  This file is part of the Procss library,      │
// │  ██╔═══╝ ██╔══██╗██║   ██║  distributed under the terms of the            │
// │  ██║     ██║  ██║╚██████╔╝  Apache License 2.0.  The full license can     │
// │  ╚═╝     ╚═╝  ╚═╝ ╚═════╝   be found in the LICENSE file.                 │
// │                                                                           │
// └───────────────────────────────────────────────────────────────────────────┘

#![feature(assert_matches)]

#[cfg(test)]
use std::assert_matches::assert_matches;

use procss::{parse, RenderCss};

#[test]
fn test_minify_percent_followed_by_minus() {
    assert_matches!(
        parse("div{width:calc(100% - 24px)}")
            .map(|x| x.as_css_string())
            .as_deref(),
        Ok("div{width:calc(100% - 24px);}")
    )
}

#[test]
fn test_minify_percent_followed_by_plus() {
    assert_matches!(
        parse("div{width:calc(100% + 28px)}")
            .map(|x| x.as_css_string())
            .as_deref(),
        Ok("div{width:calc(100% + 28px);}")
    )
}

#[test]
fn test_minify_svg() {
    assert_matches!(
        parse(r#"div { background: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' version='1.1' preserveAspectRatio='none' viewBox='0 0 100 100'><path d='M100 0 L0 100 ' stroke='black' stroke-width='1'/><path d='M0 0 L100 100 ' stroke='black' stroke-width='1'/></svg>") #8b868045; }"#)
            .map(|x| x.as_css_string())
            .as_deref(),
        Ok("div{background:url(\"data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' version='1.1' preserveAspectRatio='none' viewBox='0 0 100 100'><path d='M100 0 L0 100 ' stroke='black' stroke-width='1'/><path d='M0 0 L100 100 ' stroke='black' stroke-width='1'/></svg>\")#8b868045;}")
    )
}

#[test]
fn test_minify_complex_calc() {
    assert_matches!(
        parse(r#"div { padding: calc(17px + ((var(--status-bar--height) - 48px) / 2)); }"#)
            .map(|x| x.as_css_string())
            .as_deref(),
        Ok("div{padding:calc(17px + ((var(--status-bar--height) - 48px)/2));}")
    )
}

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

use procss::transformers::inline_url;
use procss::{parse, RenderCss};

#[test]
fn test_inline_url() {
    let ctx = procss::utils::fs::read_context();
    ctx.expect()
        .returning(|_| Ok("abcde".as_bytes().to_owned()));

    assert_matches!(
        parse(
            "
            div.open {
                color: url(\"/test.svg\");
            }
        "
        )
        .map(|x| {
            let mut css = x.flatten_tree();
            inline_url("")(&mut css);
            css.as_css_string()
        })
        .as_deref(),
        Ok("div.open{color:url(\"data:image/svg+xml;base64,YWJjZGU=\");}")
    )
}

#[test]
fn test_single_quotes_inline_url() {
    let ctx = procss::utils::fs::read_context();
    ctx.expect()
        .returning(|_| Ok("abcde".as_bytes().to_owned()));

    assert_matches!(
        parse(
            "
            div.open {
                color: url(\'/test.svg\');
            }
        "
        )
        .map(|x| {
            let mut css = x.flatten_tree();
            inline_url("")(&mut css);
            css.as_css_string()
        })
        .as_deref(),
        Ok("div.open{color:url(\"data:image/svg+xml;base64,YWJjZGU=\");}")
    )
}

#[test]
fn test_omit_non_path_inline_url() {
    let ctx = procss::utils::fs::read_context();
    ctx.expect()
        .returning(|_| Ok("abcde".as_bytes().to_owned()));

    assert_matches!(
        parse(
            "
            div.open {
                color: url(\"test.svg\");
            }
        "
        )
        .map(|x| {
            let mut css = x.flatten_tree();
            inline_url("")(&mut css);
            css.as_css_string()
        })
        .as_deref(),
        Ok("div.open{color:url(\"test.svg\");}")
    )
}

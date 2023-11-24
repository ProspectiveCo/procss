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

use procss::transformers::{apply_var, remove_var};
use procss::{parse, RenderCss};

#[test]
fn test_var() {
    assert_matches!(
        parse(
            "
            @evilred: #FF1111;

            div.open {
                color: @evilred;
            }
        "
        )
        .map(|mut x| {
            apply_var(&mut x);
            let mut flat = x.flatten_tree();
            remove_var(&mut flat);
            flat.as_css_string()
        })
        .as_deref(),
        Ok("div.open{color:#FF1111;}")
    )
}

#[test]
fn test_var_overlapping_name() {
    assert_matches!(
        parse(
            "
            @blue: #CCCCFF;
            @bluemore: #0000FF;
            div.open {
                color: @bluemore;
            }
        "
        )
        .map(|mut x| {
            apply_var(&mut x);
            let mut flat = x.flatten_tree();
            remove_var(&mut flat);
            flat.as_css_string()
        })
        .as_deref(),
        Ok("div.open{color:#0000FF;}")
    )
}

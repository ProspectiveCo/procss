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

use procss::transformers::{apply_mixin, remove_mixin};
use procss::{parse, RenderCss};

#[test]
fn test_advanced_mixin() {
    assert_matches!(
        parse(
            "
            @mixin test {
                color: green;
                opacity: 0;
            }

            div.open {
                color: red;
                @include test;
            }
        "
        )
        .map(|mut x| {
            apply_mixin(&mut x);
            let mut flatten = x.flatten_tree();
            remove_mixin(&mut flatten);
            flatten.as_css_string()
        })
        .as_deref(),
        Ok("div.open{color:red;}div.open{color:green;opacity:0;}")
    )
}

#[ignore]
#[test]
fn test_transitive_mixin() {
    assert_matches!(
        parse(
            "
            @mixin test {
                color: green;
            }

            @mixin test2 {
                @include test;
                opacity: 0;
            }

            div.open {
                color: red;
                @include test2;
            }
        "
        )
        .map(|mut x| {
            apply_mixin(&mut x);
            x.flatten_tree().as_css_string()
        })
        .as_deref(),
        Ok("div.open{color:red;}div.open{color:green;opacity:0;}")
    )
}

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
fn test_simple() {
    assert_matches!(
        parse(
            "
        @meta {
            div {
                color: red;
            }
        }
        "
        )
        .map(|x| x.as_css_string())
        .as_deref(),
        Ok("@meta{div{color:red;}}")
    )
}

#[test]
fn test_simple_flatten() {
    assert_matches!(
        parse(
            "
        @meta {
            div {
                color: red;
            }
        }
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok("@meta{div{color:red;}}")
    )
}

#[test]
fn test_nested_within_qualified() {
    assert_matches!(
        parse(
            "
        @meta {
            div {
                .open {
                    color: red;
                }
            }
        }
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok("@meta{div .open{color:red;}}")
    )
}

#[test]
fn test_double_nested_within_qualified() {
    assert_matches!(
        parse(
            "
        @meta {
            @alpha {
                span {
                    color: green;
                }
            }

            div {
                .open {
                    color: red;
                }
            }
        }
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok("@meta{@alpha{span{color:green;}}div .open{color:red;}}")
    )
}

#[test]
fn test_import() {
    assert_matches!(
        parse(
            "
        @import \"test\";

        div {
            color: green;
        }
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok("@import \"test\";div{color:green;}")
    )
}

#[test]
fn test_font_face() {
    assert_matches!(
        parse(
            "
            @font-face {
                font-family: \"Open Sans\";
                font-display: block;
                src: url(./font/open-sans.woff2) format(\"truetype\");
            }
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok("@font-face{font-family:\"Open \
            Sans\";font-display:block;src:url(./font/open-sans.woff2) format(\"truetype\");}")
    )
}

#[test]
fn test_media() {
    assert_matches!(
        parse(
            "
            @media (max-width: 1250px) {
                div {
                    color: red;
                }
            }
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok("@media (max-width:1250px){div{color:red;}}")
    )
}

#[test]
fn test_nested_media_preserved_after_flatten() {
    assert_matches!(
        parse(
            "
            @media (max-width: 1250px) {
                @media (min-width: 50px) {
                    div {
                        color: red;
                    }

                    div span {
                        color: blue;
                    }
                }
            }
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok(
            "@media (max-width:1250px){@media (min-width:50px){div{color:red;}div \
             span{color:blue;}}}"
        )
    )
}

#[test]
fn test_simple_ruleset() {
    assert_matches!(
        parse(
            "
            @test {
                div {
                    color: red;
                }

                span {
                    color: blue;
                }
            }
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok("@test{div{color:red;}span{color:blue;}}")
    )
}

// #[ignore]
#[test]
fn test_keyframes() {
    assert_matches!(
        parse(
            "
            @keyframes animation {
                0% {
                    filter: opacity(1);
                    transform: none;
                }
            
                100% {
                    filter: opacity(0.2);
                    transform: none;
                }
            }
            
        "
        )
        .map(|x| x.flatten_tree().as_css_string())
        .as_deref(),
        Ok("@keyframes \
            animation{0%{filter:opacity(1);transform:none;}100%{filter:opacity(0.2);transform:\
            none;}}")
    )
}

#[test]
fn test_simple_mixin() {
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
        .map(|x| x.as_css_string())
        .as_deref(),
        Ok("@mixin test{color:green;opacity:0;}div.open{color:red;@include test;}")
    )
}

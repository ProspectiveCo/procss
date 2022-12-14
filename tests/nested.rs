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
fn test_nested() {
    assert_matches!(
        parse("div{div{color:red}}")
            .map(|x| x.as_css_string())
            .as_deref(),
        Ok("div{div{color:red;}}")
    )
}

#[test]
fn test_flat_nested() {
    assert_matches!(
        parse("div{div{color:red}}")
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok("div div{color:red;}")
    )
}

#[test]
fn test_ambiguous_rule() {
    let complex = "
        div {
            div:hover{
                color: red
            }
        }
    ";

    assert_matches!(
        parse(complex)
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok("div div:hover{color:red;}")
    )
}

#[test]
fn test_self() {
    let complex = "
        div {
            &:hover{
                color: red
            }
        }
    ";

    assert_matches!(
        parse(complex)
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok("div:hover{color:red;}")
    )
}

#[test]
fn test_id_overrides_parent() {
    let complex = "
        #test {
            &#test2:before {
                color: red;
            }
        }
    ";

    assert_matches!(
        parse(complex)
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok("#test2:before{color:red;}")
    )
}

#[test]
fn test_desc_all() {
    let complex = "
        .test {
            & > * {
                color: red;
            }
        }

    ";

    assert_matches!(
        parse(complex)
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok(".test>*{color:red;}")
    )
}

#[test]
fn test_self_attr() {
    let complex = "
        div {
            &[data-label] {
                color: red;
            }
        }
    ";

    assert_matches!(
        parse(complex)
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok("div[data-label]{color:red;}")
    )
}

// https://github.com/sass/sass-spec#hrx
#[test]
fn test_scss_example() {
    let complex = "
        ul {
            margin-left: 1em;
            li {
                list-style-type: none;
            }
        }
    ";

    assert_matches!(
        parse(complex)
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok("ul{margin-left:1em;}ul li{list-style-type:none;}")
    )
}

#[test]
fn test_nested_order_is_preserved() {
    let complex = "
        div {
            & {
                color: red;
            }
            color: green;
        }
    ";

    assert_matches!(
        parse(complex)
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok("div{color:red;}div{color:green;}")
    )
}

#[test]
fn test_complex_nesting() {
    let complex = "
input {
    &:before {
        color: white;
    }
    &#my_id:before {
        color: red;
    }
}
    ";

    assert_matches!(
        parse(complex)
            .map(|x| x.flatten_tree().as_css_string())
            .as_deref(),
        Ok("input:before{color:white;}input#my_id:before{color:red;}")
    )
}

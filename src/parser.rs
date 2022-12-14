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

use anyhow::anyhow;
use nom::error::{convert_error, ParseError, VerboseError};
use nom::{Err, IResult};

/// A trait for CSS AST types which can be parsed from a String.
pub trait ParseCss<'a>
where
    Self: Sized,
{
    /// Parse an input string into the trait implementor, parameterized by an
    /// invoker-chosen `E` error type which allows compile-time choice between
    /// fast or debug parser implementations.
    fn parse<E>(input: &'a str) -> IResult<&'a str, Self, E>
    where
        E: ParseError<&'a str>;
}

pub fn unwrap_parse_error(input: &str, err: Err<VerboseError<&str>>) -> anyhow::Error {
    match err {
        Err::Error(e) | Err::Failure(e) => {
            anyhow!("Error parsing, unknown:\n{}", convert_error(input, e))
        }
        Err::Incomplete(needed) => anyhow!("Error parsing, unexpected input:\n {:?}", needed),
    }
}

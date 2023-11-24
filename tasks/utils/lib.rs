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

use std::process::{exit, Command};

use once_cell::sync::Lazy;
use regex::Regex;

pub trait SimpleCommand {
    fn execute(&mut self);
}

impl SimpleCommand for Command {
    fn execute(&mut self) {
        let code = self.status().expect("Failed to execute").code();
        match code {
            Some(0) => (),
            Some(x) => exit(x),
            None => exit(1),
        }
    }
}

static DEFAULT_TARGET: Lazy<String> = Lazy::new(|| {
    let re = Regex::new(r"(?m)host:\s*(.+?)$").unwrap();
    let result = Command::new("rustc").args(["-vV"]).output().unwrap();
    let target = std::str::from_utf8(&result.stdout).unwrap();
    let target = re.captures_iter(target).next().unwrap()[1].to_owned();
    target
});

pub fn get_default_target() -> &'static str {
    &DEFAULT_TARGET
}
